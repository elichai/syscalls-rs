use crate::arch::Syscalls;
use crate::socket::cmsg;
use crate::{result, syscall};
use core::convert::{TryFrom, TryInto};
use core::{
    mem::{self, MaybeUninit},
    ptr,
};
use std::ffi::OsStr;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::os::unix::{
    ffi::OsStrExt,
    io::{AsRawFd, RawFd},
};
use std::path::{Path, PathBuf};

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
) -> io::Result<RawFd> {
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
    result!(res).map(|fd| fd as RawFd)
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
        let ip = Ipv4Addr::from(addr);
        let port = u16::from_be_bytes(self.0.sin_port.to_ne_bytes());
        SocketAddrV4::new(ip, port)
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
            sin_port: addr.port().to_be() as libc::in_port_t,
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
        let ip = Ipv6Addr::from(addr);
        let port = u16::from_be_bytes(self.0.sin6_port.to_ne_bytes());
        SocketAddrV6::new(ip, port, self.0.sin6_flowinfo, self.0.sin6_scope_id)
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
            sin6_port: addr.port().to_be() as libc::in_port_t,
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
                ptr::read(addr as *const libc::sockaddr_storage as *const libc::sockaddr_un)
            })),
            libc::AF_INET => SockAddr::Inet(SockAddrInet(unsafe {
                ptr::read(addr as *const libc::sockaddr_storage as *const libc::sockaddr_in)
            })),
            libc::AF_INET6 => SockAddr::Inet6(SockAddrInet6(unsafe {
                ptr::read(addr as *const libc::sockaddr_storage as *const libc::sockaddr_in6)
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

impl TryFrom<SockAddr> for PathBuf {
    type Error = io::Error;

    fn try_from(addr: SockAddr) -> Result<Self, Self::Error> {
        match addr {
            SockAddr::Unix(addr) => Ok(addr.as_path().into()),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "inet address is not convertible to unix address",
            )),
        }
    }
}

impl From<SocketAddrV4> for SockAddr {
    fn from(addr: SocketAddrV4) -> Self {
        SockAddr::Inet(SockAddrInet::from(addr))
    }
}

impl From<SocketAddrV6> for SockAddr {
    fn from(addr: SocketAddrV6) -> Self {
        SockAddr::Inet6(SockAddrInet6::from(addr))
    }
}

impl From<SocketAddr> for SockAddr {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => addr.into(),
            SocketAddr::V6(addr) => addr.into(),
        }
    }
}

impl TryFrom<SockAddr> for SocketAddrV4 {
    type Error = io::Error;

    fn try_from(addr: SockAddr) -> Result<Self, Self::Error> {
        match addr {
            SockAddr::Inet(addr) => Ok(addr.to_addr()),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "unix address is not convertible to ip4 address",
            )),
        }
    }
}

impl TryFrom<SockAddr> for SocketAddrV6 {
    type Error = io::Error;

    fn try_from(addr: SockAddr) -> Result<Self, Self::Error> {
        match addr {
            SockAddr::Inet6(addr) => Ok(addr.to_addr()),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "unix address is not convertible to ip4 address",
            )),
        }
    }
}

impl TryFrom<SockAddr> for SocketAddr {
    type Error = io::Error;

    fn try_from(addr: SockAddr) -> Result<Self, Self::Error> {
        match addr {
            SockAddr::Inet(addr) => Ok(SocketAddr::V4(addr.to_addr())),
            SockAddr::Inet6(addr) => Ok(SocketAddr::V6(addr.to_addr())),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "unix address is not convertible to inet address",
            )),
        }
    }
}

#[inline]
pub fn bind<F: AsRawFd>(socket: F, addr: SockAddr) -> io::Result<()> {
    let (addr, addr_len) = addr.as_ffi();
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
pub fn getpeername<F: AsRawFd>(socket: F) -> io::Result<SockAddr> {
    let mut address = MaybeUninit::<libc::sockaddr_storage>::uninit();
    let mut address_len = mem::size_of::<libc::sockaddr_storage>();
    let res = unsafe {
        syscall!(
            Syscalls::Getpeername,
            socket.as_raw_fd() as isize,
            address.as_mut_ptr() as isize,
            &mut address_len as *mut _ as isize,
        )
    };
    let address = unsafe { address.assume_init() };
    result!(res).map(|_| SockAddr::from_ffi(&address))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SockOptLevel {
    Socket = libc::SOL_SOCKET as isize,
    Ip4 = libc::SOL_IP as isize,
    Ip6 = libc::SOL_IPV6 as isize,
    Udp = libc::SOL_UDP as isize,
    Tcp = libc::SOL_TCP as isize,
    Icmp6 = libc::SOL_ICMPV6 as isize,
}

pub trait SockOpt {
    fn level(&self) -> SockOptLevel;
    fn name(&self) -> isize;
    fn as_ffi(&self) -> &libc::c_int;
    fn as_ffi_mut(&mut self) -> &mut libc::c_int;
}

#[inline]
pub fn getsockopt<F: AsRawFd, T: SockOpt + Default>(socket: F) -> io::Result<T> {
    let mut opt = T::default();
    let level = opt.level();
    let name = opt.name();
    let val: &mut libc::c_int = &mut opt.as_ffi_mut();
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
    result!(res).map(|_| opt)
}

#[inline]
pub fn setsockopt<F: AsRawFd, T: SockOpt>(socket: F, opt: T) -> io::Result<()> {
    let val: &libc::c_int = &opt.as_ffi();
    let len = mem::size_of_val(val) as libc::socklen_t;
    let res = unsafe {
        syscall!(
            Syscalls::Setsockopt,
            socket.as_raw_fd() as isize,
            opt.level() as isize,
            opt.name() as isize,
            val as *const _ as isize,
            len as isize,
        )
    };
    result!(res).map(|_| ())
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct MsgFlags(isize);

impl MsgFlags {
    /// Creates new `MsgFlags`.
    pub fn new() -> Self {
        Self(0)
    }

    /// MSG_CONFIRM (Since Linux 2.3.15)
    /// Tell the link layer that forward progress happened: you got a
    /// successful reply from the other side. If the link layer doesn't get
    /// this it will regularly reprobe the neighbor (e.g., via a unicast ARP).
    /// Only valid on SOCK_DGRAM and SOCK_RAW sockets and currently only
    /// implemented for IPv4 and IPv6. See arp(7) for details.
    pub fn confirm(mut self) -> Self {
        self.0 |= libc::MSG_CONFIRM as isize;
        self
    }

    /// MSG_DONTROUTE
    /// Don't use a gateway to send out the packet, only send to hosts on
    /// directly connected networks. This is usually used only by diagnostic or
    /// routing programs. This is only defined for protocol families that route;
    /// packet sockets don't.
    pub fn dont_route(mut self) -> Self {
        self.0 |= libc::MSG_DONTROUTE as isize;
        self
    }

    /// MSG_DONTWAIT (since Linux 2.2)
    /// Enables nonblocking operation; if the operation would block, EAGAIN or
    /// EWOULDBLOCK is returned (this can also be enabled using the O_NONBLOCK
    /// flag with the F_SETFL fcntl(2)).
    pub fn dont_wait(mut self) -> Self {
        self.0 |= libc::MSG_DONTWAIT as isize;
        self
    }

    /// MSG_EOR (since Linux 2.2)
    /// Terminates a record (when this notion is supported, as for sockets of
    /// type SOCK_SEQPACKET).
    pub fn eor(mut self) -> Self {
        self.0 |= libc::MSG_EOR as isize;
        self
    }

    /// MSG_MORE (Since Linux 2.4.4)
    /// The caller has more data to send. This flag is used with TCP sockets to
    /// obtain the same effect as the TCP_CORK socket option (see tcp(7)), with
    /// the difference that this flag can be set on a per-call basis.
    /// Since Linux 2.6, this flag is also supported for UDP sockets, and
    /// informs the kernel to package all of the data sent in calls with this
    /// flag set into a single datagram which is only transmitted when a call
    /// is performed that does not specify this flag. (See also the UDP_CORK
    /// socket option described in udp(7).)
    pub fn more(mut self) -> Self {
        self.0 |= libc::MSG_MORE as isize;
        self
    }

    /// MSG_NOSIGNAL (since Linux 2.2)
    /// Requests not to send SIGPIPE on errors on stream oriented sockets when
    /// the other end breaks the connection. The EPIPE error is still returned.
    pub fn no_signal(mut self) -> Self {
        self.0 |= libc::MSG_NOSIGNAL as isize;
        self
    }

    /// MSG_OOB
    /// Sends out-of-band data on sockets that support this notion (e.g., of
    /// type SOCK_STREAM); the underlying protocol must also support out-of-band
    /// data.
    pub fn oob(mut self) -> Self {
        self.0 |= libc::MSG_OOB as isize;
        self
    }

    /// MSG_CMSG_CLOEXEC (recvmsg() only; since Linux 2.6.23)
    /// Set the close-on-exec flag for the file descriptor received via a UNIX
    /// domain file descriptor using the SCM_RIGHTS operation (described in unix(7)).
    /// This flag is useful for the same reasons as the O_CLOEXEC flag of open(2).
    pub fn cmsg_cloexec(mut self) -> Self {
        self.0 |= libc::MSG_CMSG_CLOEXEC as isize;
        self
    }

    /// MSG_ERRQUEUE (since Linux 2.2)
    /// This flag specifies that queued errors should be received from the
    /// socket error queue. The error is passed in an ancillary message with a
    /// type dependent on the protocol (for IPv4 IP_RECVERR). The user should
    /// supply a buffer of sufficient size. See cmsg(3) and ip(7) for more
    /// information. The payload of the original packet that caused the error
    /// is passed as normal data via msg_iovec. The original destination address
    /// of the datagram that caused the error is supplied via msg_name.
    /// For local errors, no address is passed (this can be checked with the
    /// cmsg_len member of the cmsghdr). For error receives, the MSG_ERRQUEUE
    /// is set in the msghdr. After an error has been passed, the pending
    /// socket error is regenerated based on the next queued error and will be
    /// passed on the next socket operation.
    pub fn err_queue(mut self) -> Self {
        self.0 |= libc::MSG_ERRQUEUE as isize;
        self
    }

    /// MSG_PEEK
    /// This flag causes the receive operation to return data from the beginning
    /// of the receive queue without removing that data from the queue. Thus, a
    /// subsequent receive call will return the same data.
    pub fn peek(mut self) -> Self {
        self.0 |= libc::MSG_PEEK as isize;
        self
    }

    /// MSG_TRUNC (since Linux 2.2)
    /// For raw (AF_PACKET), Internet datagram (since Linux 2.4.27/2.6.8),
    /// netlink (since Linux 2.6.22) and UNIX datagram (since Linux 3.4)
    /// sockets: return the real length of the packet or datagram, even when it
    /// was longer than the passed buffer. Not implemented for UNIX domain
    /// (unix(7)) sockets. For use with Internet stream sockets, see tcp(7).
    pub fn trunc(mut self) -> Self {
        self.0 |= libc::MSG_TRUNC as isize;
        self
    }

    /// MSG_WAITALL (since Linux 2.2)
    /// This flag requests that the operation block until the full request is
    /// satisfied. However, the call may still return less data than requested
    /// if a signal is caught, an error or disconnect occurs, or the next data
    /// to be received is of a different type than that returned.
    pub fn wait_all(mut self) -> Self {
        self.0 |= libc::MSG_WAITALL as isize;
        self
    }
}

impl core::fmt::Debug for MsgFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MsgFlags")
            .field("CONFIRM", &(self.0 & libc::MSG_CONFIRM as isize > 0))
            .field("DONTROUTE", &(self.0 & libc::MSG_DONTROUTE as isize > 0))
            .field("DONTWAIT", &(self.0 & libc::MSG_DONTWAIT as isize > 0))
            .field("EOR", &(self.0 & libc::MSG_EOR as isize > 0))
            .field("MORE", &(self.0 & libc::MSG_MORE as isize > 0))
            .field("NOSIGNAL", &(self.0 & libc::MSG_NOSIGNAL as isize > 0))
            .field("OOB", &(self.0 & libc::MSG_OOB as isize > 0))
            .field(
                "CMSG_CLOEXEC",
                &(self.0 & libc::MSG_CMSG_CLOEXEC as isize > 0),
            )
            .field("ERRQUEUE", &(self.0 & libc::MSG_ERRQUEUE as isize > 0))
            .field("PEEK", &(self.0 & libc::MSG_PEEK as isize > 0))
            .field("TRUNC", &(self.0 & libc::MSG_TRUNC as isize > 0))
            .field("WAITALL", &(self.0 & libc::MSG_WAITALL as isize > 0))
            .finish()
    }
}

#[inline]
pub fn sendmsg<F: AsRawFd>(
    socket: F,
    addr: Option<&SockAddr>,
    msg: &[u8],
    cmsgs: &[&dyn SockOpt],
    flags: MsgFlags,
) -> io::Result<usize> {
    let (name, namelen) = addr.map(|addr| addr.as_ffi()).unwrap_or((ptr::null(), 0));
    let mut iov = libc::iovec {
        iov_base: msg.as_ptr() as *const _ as *mut _,
        iov_len: msg.len(),
    };
    let mut ctrl = cmsg::Aligned(MaybeUninit::<[u8; 255]>::uninit());
    let mut hdr = libc::msghdr {
        msg_name: name as _,
        msg_namelen: namelen as _,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ctrl.0.as_mut_ptr() as _,
        msg_controllen: 255,
        msg_flags: 0,
    };
    let mut encoder = unsafe { cmsg::Encoder::new(&mut hdr) };
    for cmsg in cmsgs {
        encoder.push(cmsg.level() as i32, cmsg.name() as i32, *cmsg.as_ffi());
    }
    encoder.finish();
    loop {
        let res = unsafe {
            syscall!(
                Syscalls::Sendmsg,
                socket.as_raw_fd() as isize,
                &hdr as *const _ as isize,
                flags.0,
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
    cbuf: &mut [&mut dyn SockOpt],
    flags: MsgFlags,
) -> io::Result<(usize, SockAddr)> {
    let mut address = MaybeUninit::<libc::sockaddr_storage>::uninit();
    let address_len = mem::size_of_val(&address);
    let mut iov = libc::iovec {
        iov_base: buf.as_mut_ptr() as *mut _,
        iov_len: buf.len(),
    };
    let mut ctrl = cmsg::Aligned(MaybeUninit::<[u8; 255]>::uninit());
    let mut hdr = libc::msghdr {
        msg_name: address.as_mut_ptr() as *mut _,
        msg_namelen: address_len as _,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ctrl.0.as_mut_ptr() as _,
        msg_controllen: 255,
        msg_flags: 0,
    };
    let n = loop {
        let res = unsafe {
            syscall!(
                Syscalls::Recvmsg,
                socket.as_raw_fd() as isize,
                &mut hdr as *mut _ as isize,
                flags.0,
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
    for cmsg in unsafe { cmsg::Iter::new(&hdr) } {
        if let Some(dst) = cbuf
            .iter_mut()
            .find(|c| c.level() as i32 == cmsg.cmsg_level && c.name() as i32 == cmsg.cmsg_type)
        {
            *dst.as_ffi_mut() = unsafe { cmsg::decode::<libc::c_int>(cmsg) };
        }
    }
    Ok((n, address))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::close;

    pub struct Socket {
        fd: RawFd,
        pub addr: Option<SockAddr>,
    }

    impl Socket {
        pub fn close(&mut self) {
            unsafe {
                close(&*self).ok();
            }
            self.fd = -1;
        }
    }

    impl Socket {
        pub fn new(fd: RawFd) -> Self {
            Self { fd, addr: None }
        }
    }

    impl AsRawFd for Socket {
        fn as_raw_fd(&self) -> RawFd {
            self.fd
        }
    }

    impl AsRawFd for &Socket {
        fn as_raw_fd(&self) -> RawFd {
            (*self).as_raw_fd()
        }
    }

    impl Drop for Socket {
        fn drop(&mut self) {
            unsafe {
                close(self).ok();
            }
        }
    }

    pub fn localhost(domain: AddressFamily) -> SocketAddr {
        match domain {
            AddressFamily::Inet => {
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0))
            }
            AddressFamily::Inet6 => SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                0,
                0,
                0,
            )),
            _ => unimplemented!(),
        }
    }

    pub fn create_socket(domain: AddressFamily, ty: SockType) -> Socket {
        let mut fd = Socket::new(socket(domain, ty, SockFlags::new(), None).unwrap());
        bind(&fd, localhost(domain).into()).unwrap();
        fd.addr = Some(getsockname(&fd).unwrap());
        fd
    }

    pub fn socket_pair(domain: AddressFamily, ty: SockType) -> (Socket, Socket) {
        let s1 = create_socket(domain, ty);
        let s2 = create_socket(domain, ty);
        (s1, s2)
    }

    #[test]
    fn test_socket_ip4_udp() {
        let (s1, s2) = socket_pair(AddressFamily::Inet, SockType::Datagram);
        sendmsg(&s1, s2.addr.as_ref(), b"hello", &[], MsgFlags::new()).unwrap();
        let mut buf = [0u8; 10];
        let (len, addr) = recvmsg(&s2, &mut buf, &mut [], MsgFlags::new()).unwrap();
        assert_eq!(addr, s1.addr.unwrap());
        assert_eq!(buf[..len], b"hello"[..]);
    }

    #[test]
    fn test_socket_ip6_udp() {
        let (s1, s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        sendmsg(&s1, s2.addr.as_ref(), b"hello", &[], MsgFlags::new()).unwrap();
        let mut buf = [0u8; 10];
        let (len, addr) = recvmsg(&s2, &mut buf, &mut [], MsgFlags::new()).unwrap();
        assert_eq!(addr, s1.addr.unwrap());
        assert_eq!(buf[..len], b"hello"[..]);
    }

    #[test]
    fn test_correct_port() {
        let addr: SocketAddr = "127.0.0.1:40".parse().unwrap();
        let addr: SockAddr = addr.into();
        let addr: SocketAddr = addr.try_into().unwrap();
        assert_eq!(addr.port(), 40);
    }
}
