use crate::arch::Syscalls;
use crate::{result, syscall};
use std::io;

use libc::{
    AF_INET, AF_INET6, AF_UNIX, SOCK_CLOEXEC, SOCK_DGRAM, SOCK_NONBLOCK, SOCK_RAW, SOCK_RDM,
    SOCK_SEQPACKET, SOCK_STREAM,
};

use linux_sys::{IPPROTO_TCP, IPPROTO_UDP};

/// Constants used to specify the protocol family to be used in [`socket`](fn.socket.html).
/// TODO: Should we include them all?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AddressFamily {
    /// Local communication ([`unix(7)`](http://man7.org/linux/man-pages/man7/unix.7.html)).
    Unix = AF_UNIX as isize,
    /// IPv4 internet protocols ([`ip(7)`](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Inet = AF_INET as isize,
    /// IPv6 internet protocols ([`ipv6(7)`](http://man7.org/linux/man-pages/man7/ipv6.7.html)).
    Inet6 = AF_INET6 as isize,
}

/// Constants used to specify the communication semantics when creating a
/// socket with [`socket`](fn.socket.html)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SockType {
    /// Provides sequenced, reliable, two-way, connection based byte streams.
    /// An out-of-band data transmission mechanism may be supported.
    Stream = SOCK_STREAM as isize,
    /// Supports datagrams - connectionless, unreliable messages of a fixed
    /// maximum length.
    Datagram = SOCK_DGRAM as isize,
    /// Provides a sequenced, reliable, two-way connection based data
    /// transmission path for datagrams of fixed maximum length; a consumer is
    /// required to read an entire packet with each input system call.
    SeqPacket = SOCK_SEQPACKET as isize,
    /// Provides raw network protocol access.
    Raw = SOCK_RAW as isize,
    /// Provides a reliable datagram layer that does not guarantee ordering.
    Rdm = SOCK_RDM as isize,
}

/// Constants used in [`socket`](fn.socket.html) and [`socketpair`](fn.socketpair.html)
/// to specify the protocol to use.
/// Which protocols should we add?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SockProtocol {
    /// TCP protocol ([ip(7)](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Tcp = IPPROTO_TCP as isize,
    /// UDP protocol ([ip(7)](http://man7.org/linux/man-pages/man7/ip.7.html)).
    Udp = IPPROTO_UDP as isize,
}

/// Additional socket options.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Default)]
pub struct SockFlags(isize);

impl SockFlags {
    /// Creates new `SocketFlags`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set non-blocking mode on the new socket.
    pub fn nonblock(mut self) -> Self {
        self.0 |= SOCK_NONBLOCK as isize;
        self
    }

    /// Set close-on-exec on the new descriptor.
    pub fn cloexec(mut self) -> Self {
        self.0 |= SOCK_CLOEXEC as isize;
        self
    }
}

impl core::fmt::Debug for SockFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SockFlags")
            .field("NONBLOCK", &(self.0 & SOCK_NONBLOCK as isize > 0))
            .field("CLOEXEC", &(self.0 & SOCK_CLOEXEC as isize > 0))
            .finish()
    }
}

#[inline]
pub unsafe fn socket(
    domain: AddressFamily,
    sock_type: SockType,
    flags: SockFlags,
    protocol: Option<SockProtocol>,
) -> io::Result<usize> {
    // flags only supported by kernel >= 2.6.27
    let ty = sock_type as isize | flags.0;
    let protocol = protocol.map(|proto| proto as isize).unwrap_or(0);
    let res = syscall!(Syscalls::Socket, domain as isize, ty, protocol);
    result!(res)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::close;
    use std::os::unix::io::{AsRawFd, RawFd};

    pub struct Socket {
        fd: RawFd,
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
            Self { fd }
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

    #[test]
    fn test_socket() {
        let rawfd = unsafe {
            socket(
                AddressFamily::Inet,
                SockType::Datagram,
                SockFlags::new(),
                None,
            )
        }
        .unwrap();
        Socket::new(rawfd as RawFd);
    }
}
