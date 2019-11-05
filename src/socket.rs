use crate::arch::Syscalls;
use crate::close;
use crate::{result, syscall};
use core::mem::size_of_val;
use libc::{
    sockaddr, socklen_t, AF_INET, AF_INET6, SOCK_CLOEXEC, SOCK_DGRAM, SOCK_NONBLOCK, SOCK_RAW,
    SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
};
use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};
use std::{
    io,
    mem::{self, MaybeUninit},
    ptr,
};

pub struct Fd(usize);

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0 as RawFd
    }
}

impl AsRawFd for &Fd {
    fn as_raw_fd(&self) -> RawFd {
        (*self).as_raw_fd()
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe {
            close(self).ok();
        }
    }
}

fn socket_addr(addr: SocketAddr) -> (*const sockaddr, socklen_t) {
    match addr {
        SocketAddr::V4(ref addr) => (
            addr as *const _ as *const sockaddr,
            size_of_val(addr) as socklen_t,
        ),
        SocketAddr::V6(ref addr) => (
            addr as *const _ as *const sockaddr,
            size_of_val(addr) as socklen_t,
        ),
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum SocketType {
    Stream = SOCK_STREAM,
    Dgram = SOCK_DGRAM,
    SeqPacket = SOCK_SEQPACKET,
    Raw = SOCK_RAW,
    Rdm = SOCK_RDM,
    // SOCK_PACKET is obsolete
}

#[derive(Clone, Copy)]
pub struct SocketFlags(i32);

impl SocketFlags {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn nonblock(mut self) -> Self {
        self.0 |= SOCK_NONBLOCK;
        self
    }

    pub fn cloexec(mut self) -> Self {
        self.0 |= SOCK_CLOEXEC;
        self
    }
}

#[inline]
pub fn socket(addr: SocketAddr, sock: SocketType, flags: SocketFlags) -> io::Result<Fd> {
    // flags only supported by kernel >= 2.6.27
    let domain = match addr {
        SocketAddr::V4(_) => AF_INET,
        SocketAddr::V6(_) => AF_INET6,
    };
    let ty = sock as i32 | flags.0;
    // TODO expose protocol
    let protocol = 0;
    let res = unsafe {
        syscall!(
            Syscalls::Socket,
            domain as isize,
            ty as isize,
            protocol as isize
        )
    };
    result!(res).map(Fd)
}

#[inline]
pub fn bind<F: AsRawFd>(socket: F, addr: SocketAddr) -> io::Result<()> {
    let (addr, addr_len) = socket_addr(addr);
    let res = unsafe {
        syscall!(
            Syscalls::Bind,
            socket.as_raw_fd() as isize,
            addr as isize,
            addr_len as isize
        )
    };
    result!(res).map(|_| ())
}

#[inline]
pub fn getsockname<F: AsRawFd>(socket: F) -> io::Result<SocketAddr> {
    let mut address = MaybeUninit::<libc::sockaddr_storage>::uninit();
    let mut address_len = mem::size_of_val(&address);
    let res = unsafe {
        syscall!(
            Syscalls::Getsockname,
            socket.as_raw_fd() as isize,
            address.as_mut_ptr() as isize,
            &mut address_len as *mut _ as isize
        )
    };
    let address = unsafe { address.assume_init() };
    result!(res).map(|_| match address.ss_family as libc::c_int {
        AF_INET => SocketAddr::V4(unsafe { ptr::read(&address as *const _ as _) }),
        AF_INET6 => SocketAddr::V6(unsafe { ptr::read(&address as *const _ as _) }),
        _ => unreachable!(),
    })
}

#[inline]
pub fn sendmsg<F: AsRawFd>(
    socket: F,
    addr: SocketAddr,
    //ctrl: MsgCtrl,
    msg: &[u8],
    flags: u32,
) -> io::Result<usize> {
    let (name, namelen) = socket_addr(addr);
    let mut iov = libc::iovec {
        iov_base: msg.as_ptr() as *const _ as *mut _,
        iov_len: msg.len(),
    };
    let hdr = libc::msghdr {
        msg_name: name as _,
        msg_namelen: namelen as _,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    loop {
        let res = unsafe {
            syscall!(
                Syscalls::Sendmsg,
                socket.as_raw_fd() as isize,
                &hdr as *const _ as isize,
                flags as isize
            )
        };
        return match result!(res) {
            Ok(n) => Ok(n),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => Err(e),
        };
    }
}

#[inline]
pub fn recvmsg<F: AsRawFd>(
    socket: F,
    buf: &mut [u8],
    flags: u32,
) -> io::Result<(usize, SocketAddr /*, MsgCtrl*/)> {
    let mut address = MaybeUninit::<libc::sockaddr_storage>::uninit();
    let address_len = mem::size_of_val(&address);
    let mut iov = libc::iovec {
        iov_base: buf.as_mut_ptr() as *mut _,
        iov_len: buf.len(),
    };
    let mut hdr = libc::msghdr {
        msg_name: address.as_mut_ptr() as *mut _,
        msg_namelen: address_len as _,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let n = loop {
        let res = unsafe {
            syscall!(
                Syscalls::Recvmsg,
                socket.as_raw_fd() as isize,
                &mut hdr as *mut _ as isize,
                flags as isize,
            )
        };
        match result!(res) {
            Ok(n) => break n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    };
    let address = unsafe { address.assume_init() };
    let address = match address.ss_family as libc::c_int {
        AF_INET => SocketAddr::V4(unsafe { ptr::read(&address as *const _ as _) }),
        AF_INET6 => SocketAddr::V6(unsafe { ptr::read(&address as *const _ as _) }),
        _ => unreachable!(),
    };
    Ok((n, address))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_ip4_no_flags() {
        let ip4 = "127.0.0.1:0".parse().unwrap();
        let fd1 = socket(ip4, SocketType::Dgram, SocketFlags::new()).unwrap();
        let fd2 = socket(ip4, SocketType::Dgram, SocketFlags::new()).unwrap();
        bind(&fd1, ip4).unwrap();
        bind(&fd2, ip4).unwrap();
        let addr1 = getsockname(&fd1).unwrap();
        let addr2 = getsockname(&fd2).unwrap();
        println!("{:?} {:?}", addr1, addr2);
        sendmsg(&fd1, addr2, b"hello", 0).unwrap();
        let mut buf = [0u8; 10];
        let (len, addr) = recvmsg(&fd2, &mut buf, 0).unwrap();
        assert_eq!(addr, addr1);
        assert_eq!(buf[..len], b"hello"[..]);
    }

    #[test]
    fn test_socket_ip6_with_flags() {
        let ip6 = "[::1]:0".parse().unwrap();
        let flags = SocketFlags::new().cloexec().nonblock();
        let fd1 = socket(ip6, SocketType::Dgram, flags).unwrap();
        let fd2 = socket(ip6, SocketType::Dgram, flags).unwrap();
        bind(&fd1, ip6).unwrap();
        bind(&fd2, ip6).unwrap();
        let addr1 = getsockname(&fd1).unwrap();
        let addr2 = getsockname(&fd2).unwrap();
        println!("{:?} {:?}", addr1, addr2);
        sendmsg(&fd1, addr2, b"hello", 0).unwrap();
        let mut buf = [0u8; 10];
        let (len, addr) = recvmsg(&fd2, &mut buf, 0).unwrap();
        assert_eq!(addr, addr1);
        assert_eq!(buf[..len], b"hello"[..]);
    }
}
