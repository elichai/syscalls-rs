use crate::arch::Syscalls;
use crate::close;
use crate::{result, syscall};
use core::convert::{TryFrom, TryInto};
use core::{
    mem::{self, MaybeUninit},
    ptr,
};
use std::ffi::OsStr;
use std::io;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::os::unix::{
    ffi::OsStrExt,
    io::{AsRawFd, RawFd},
};
use std::path::Path;

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

/// Constants used to specify the protocol family to be used in [`socket`](fn.socket.html).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AddressFamily {
    /// Local communication ([`unix(7)`](http://man7.org/linux/man-pages/man7/unix.7.html)).
    Unix = libc::AF_UNIX as isize,
    /// IPv4 internet protocols ([`ip(7)`](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Inet = libc::AF_INET as isize,
    /// IPv6 internet protocols ([`ipv6(7)`](http://man7.org/linux/man-pages/man7/ipv6.7.html)).
    Inet6 = libc::AF_INET6 as isize,
}

/// Constants used to specify the communication semantics when creating a
/// socket with [`socket`](fn.socket.html)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SockType {
    /// Provides sequenced, reliable, two-way, connection based byte streams.
    /// An out-of-band data transmission mechanism may be supported.
    Stream = libc::SOCK_STREAM as isize,
    /// Supports datagrams - connectionless, unreliable messages of a fixed
    /// maximum length.
    Datagram = libc::SOCK_DGRAM as isize,
    /// Provides a sequenced, reliable, two-way connection based data
    /// transmission path for datagrams of fixed maximum length; a consumer is
    /// required to read an entire packet with each input system call.
    SeqPacket = libc::SOCK_SEQPACKET as isize,
    /// Provides raw network protocol access.
    Raw = libc::SOCK_RAW as isize,
    /// Provides a reliable datagram layer that does not guarantee ordering.
    Rdm = libc::SOCK_RDM as isize,
}

/// Constants used in [`socket`](fn.socket.html) and [`socketpair`](fn.socketpair.html)
/// to specify the protocol to use.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SockProtocol {
    /// TCP protocol ([ip(7)](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Tcp = libc::IPPROTO_TCP as isize,
    /// UDP protocol ([ip(7)](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Udp = libc::IPPROTO_UDP as isize,
}

/// Additional socket options.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct SockFlags(isize);

impl SockFlags {
    /// Creates new `SocketFlags`.
    pub fn new() -> Self {
        Self(0)
    }

    /// Set non-blocking mode on the new socket.
    pub fn nonblock(mut self) -> Self {
        self.0 |= libc::SOCK_NONBLOCK as isize;
        self
    }

    /// Set close-on-exec on the new descriptor.
    pub fn cloexec(mut self) -> Self {
        self.0 |= libc::SOCK_CLOEXEC as isize;
        self
    }
}

impl core::fmt::Debug for SockFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SockFlags")
            .field("NONBLOCK", &(self.0 & libc::SOCK_NONBLOCK as isize > 0))
            .field("CLOEXEC", &(self.0 & libc::SOCK_CLOEXEC as isize > 0))
            .finish()
    }
}

#[inline]
pub fn socket(
    domain: AddressFamily,
    sock: SockType,
    flags: SockFlags,
    protocol: Option<SockProtocol>,
) -> io::Result<Fd> {
    // flags only supported by kernel >= 2.6.27
    let ty = sock as isize | flags.0;
    let protocol = protocol.map(|proto| proto as isize).unwrap_or(0);
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

#[derive(Clone, Copy)]
pub struct SockAddrUnix(libc::sockaddr_un);

impl SockAddrUnix {
    fn as_path(&self) -> &Path {
        let bytes = unsafe { &*(&self.0.sun_path[..] as *const [i8] as *const [u8]) };
        OsStr::from_bytes(bytes).as_ref()
    }
}

impl core::fmt::Debug for SockAddrUnix {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.as_path().fmt(f)
    }
}

impl core::cmp::PartialEq for SockAddrUnix {
    fn eq(&self, other: &Self) -> bool {
        self.as_path().eq(other.as_path())
    }
}

impl core::cmp::Eq for SockAddrUnix {}

impl TryFrom<&Path> for SockAddrUnix {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut addr = libc::sockaddr_un {
            sun_family: libc::AF_UNIX as libc::sa_family_t,
            sun_path: [0; 108],
        };

        let bytes = path.as_os_str().as_bytes();

        if bytes.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "paths may not contain interior null bytes",
            ));
        }

        if bytes.len() >= addr.sun_path.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be shorter than SUN_LEN",
            ));
        }

        for (dst, src) in addr.sun_path.iter_mut().zip(bytes.iter()) {
            *dst = *src as libc::c_char;
        }

        Ok(SockAddrUnix(addr))
    }
}

#[derive(Clone, Copy)]
pub struct SockAddrInet(libc::sockaddr_in);

impl SockAddrInet {
    fn to_addr(&self) -> SocketAddrV4 {
        let addr = self.0.sin_addr.s_addr.to_ne_bytes();
        let ip = std::net::Ipv4Addr::from(addr);
        SocketAddrV4::new(ip, self.0.sin_port)
    }
}

impl core::fmt::Debug for SockAddrInet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_addr().fmt(f)
    }
}

impl core::cmp::PartialEq for SockAddrInet {
    fn eq(&self, other: &Self) -> bool {
        self.to_addr().eq(&other.to_addr())
    }
}

impl core::cmp::Eq for SockAddrInet {}

impl From<SocketAddrV4> for SockAddrInet {
    fn from(addr: SocketAddrV4) -> Self {
        let addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: addr.port() as libc::in_port_t,
            sin_addr: libc::in_addr {
                s_addr: u32::from_ne_bytes(addr.ip().octets()),
            },
            sin_zero: [0; 8],
        };
        Self(addr)
    }
}

#[derive(Clone, Copy)]
pub struct SockAddrInet6(libc::sockaddr_in6);

impl SockAddrInet6 {
    fn to_addr(&self) -> SocketAddrV6 {
        let addr = self.0.sin6_addr.s6_addr;
        let ip = std::net::Ipv6Addr::from(addr);
        SocketAddrV6::new(
            ip,
            self.0.sin6_port,
            self.0.sin6_flowinfo,
            self.0.sin6_scope_id,
        )
    }
}

impl core::fmt::Debug for SockAddrInet6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_addr().fmt(f)
    }
}

impl core::cmp::PartialEq for SockAddrInet6 {
    fn eq(&self, other: &Self) -> bool {
        self.to_addr().eq(&other.to_addr())
    }
}

impl core::cmp::Eq for SockAddrInet6 {}

impl From<SocketAddrV6> for SockAddrInet6 {
    fn from(addr: SocketAddrV6) -> Self {
        Self(libc::sockaddr_in6 {
            sin6_family: libc::AF_INET6 as libc::sa_family_t,
            sin6_port: addr.port() as libc::in_port_t,
            sin6_addr: libc::in6_addr {
                s6_addr: addr.ip().octets(),
            },
            sin6_flowinfo: addr.flowinfo(),
            sin6_scope_id: addr.scope_id(),
        })
    }
}

/// SocketAddr
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SockAddr {
    Unix(SockAddrUnix),
    Inet(SockAddrInet),
    Inet6(SockAddrInet6),
}

impl SockAddr {
    fn as_ffi(&self) -> (*const libc::sockaddr, libc::socklen_t) {
        use SockAddr::*;
        match self {
            Unix(addr) => (
                &addr.0 as *const libc::sockaddr_un as *const libc::sockaddr,
                mem::size_of_val(&addr.0) as libc::socklen_t,
            ),
            Inet(addr) => (
                &addr.0 as *const libc::sockaddr_in as *const libc::sockaddr,
                mem::size_of_val(&addr.0) as libc::socklen_t,
            ),
            Inet6(addr) => (
                &addr.0 as *const libc::sockaddr_in6 as *const libc::sockaddr,
                mem::size_of_val(&addr.0) as libc::socklen_t,
            ),
        }
    }

    fn from_ffi(addr: &libc::sockaddr_storage) -> Self {
        match addr.ss_family as libc::c_int {
            libc::AF_UNIX => SockAddr::Unix(SockAddrUnix(unsafe {
                ptr::read(addr as *const _ as *const _)
            })),
            libc::AF_INET => SockAddr::Inet(SockAddrInet(unsafe {
                ptr::read(addr as *const _ as *const _)
            })),
            libc::AF_INET6 => SockAddr::Inet6(SockAddrInet6(unsafe {
                ptr::read(addr as *const _ as *const _)
            })),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&Path> for SockAddr {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(SockAddr::Unix(path.try_into()?))
    }
}

impl TryFrom<SocketAddr> for SockAddr {
    type Error = io::Error;

    fn try_from(addr: SocketAddr) -> Result<Self, Self::Error> {
        Ok(match addr {
            SocketAddr::V4(addr) => SockAddr::Inet(SockAddrInet::from(addr)),
            SocketAddr::V6(addr) => SockAddr::Inet6(SockAddrInet6::from(addr)),
        })
    }
}

#[inline]
pub fn bind<F: AsRawFd, A: TryInto<SockAddr, Error = io::Error>>(
    socket: F,
    addr: A,
) -> io::Result<()> {
    let (addr, addr_len) = addr.try_into()?.as_ffi();
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
pub fn getsockname<F: AsRawFd>(socket: F) -> io::Result<SockAddr> {
    let mut address = MaybeUninit::<libc::sockaddr_storage>::uninit();
    let mut address_len = mem::size_of::<libc::sockaddr_storage>();
    let res = unsafe {
        syscall!(
            Syscalls::Getsockname,
            socket.as_raw_fd() as isize,
            address.as_mut_ptr() as isize,
            &mut address_len as *mut _ as isize
        )
    };
    let address = unsafe { address.assume_init() };
    result!(res).map(|_| SockAddr::from_ffi(&address))
}

#[inline]
pub fn sendmsg<F: AsRawFd>(
    socket: F,
    addr: SockAddr,
    //ctrl: MsgCtrl,
    msg: &[u8],
    flags: u32,
) -> io::Result<usize> {
    let (name, namelen) = addr.as_ffi();
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
) -> io::Result<(usize, SockAddr /*, MsgCtrl*/)> {
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
    let address = SockAddr::from_ffi(&address);
    Ok((n, address))
}

#[derive(Clone, Copy, Debug)]
pub enum SockOptLevel {
    Socket = libc::SOL_SOCKET as isize,
    Ip4 = libc::SOL_IP as isize,
    Ip6 = libc::SOL_IPV6 as isize,
    Udp = libc::SOL_UDP as isize,
    Tcp = libc::SOL_TCP as isize,
    Icmp6 = libc::SOL_ICMPV6 as isize,
}

#[inline]
pub fn getsockopt<F: AsRawFd, T>(
    socket: F,
    level: SockOptLevel,
    name: i32,
    val: &mut T,
) -> io::Result<()> {
    let mut len = mem::size_of_val(val);
    let res = unsafe {
        syscall!(
            Syscalls::Getsockopt,
            socket.as_raw_fd() as isize,
            level as isize,
            name as isize,
            val as *mut _ as isize,
            &mut len as *mut _ as isize,
        )
    };
    result!(res).map(|_| ())
}

#[inline]
pub fn setsockopt<F: AsRawFd, T: core::fmt::Debug>(
    socket: F,
    level: SockOptLevel,
    name: i32,
    val: &T,
) -> io::Result<()> {
    let len = mem::size_of_val(val) as libc::socklen_t;
    let res = unsafe {
        syscall!(
            Syscalls::Setsockopt,
            socket.as_raw_fd() as isize,
            level as i32 as isize,
            name as isize,
            val as *const _ as isize,
            len as isize,
        )
    };
    result!(res).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_ip4_no_flags() {
        let ip4: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let fd1 = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlags::new(),
            None,
        )
        .unwrap();
        let fd2 = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlags::new(),
            None,
        )
        .unwrap();
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
    #[ignore]
    fn test_socket_ip6_with_flags() {
        let ip6: SocketAddr = "[::1]:0".parse().unwrap();
        let flags = SockFlags::new().cloexec().nonblock();
        let fd1 = socket(AddressFamily::Inet6, SockType::Datagram, flags, None).unwrap();
        let fd2 = socket(AddressFamily::Inet6, SockType::Datagram, flags, None).unwrap();
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

    #[test]
    fn test_opts_ip4() {
        let fd1 = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlags::new(),
            None,
        )
        .unwrap();
        let on: libc::c_int = 1;
        setsockopt(&fd1, SockOptLevel::Ip4, libc::IP_RECVTOS, &on).unwrap();
        let mut on: libc::c_int = 0;
        getsockopt(&fd1, SockOptLevel::Ip4, libc::IP_RECVTOS, &mut on).unwrap();
        assert_eq!(on, 1);
    }

    #[test]
    #[ignore]
    fn test_opts_ip6() {
        let fd1 = socket(
            AddressFamily::Inet6,
            SockType::Datagram,
            SockFlags::new(),
            None,
        )
        .unwrap();
        let on: libc::c_int = 1;
        setsockopt(&fd1, SockOptLevel::Ip6, libc::IPV6_RECVTCLASS, &on).unwrap();
        let mut on: libc::c_int = 0;
        getsockopt(&fd1, SockOptLevel::Ip6, libc::IPV6_RECVTCLASS, &mut on).unwrap();
        assert_eq!(on, 1);
    }
}
