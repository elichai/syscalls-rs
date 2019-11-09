use crate::socket::SockOptLevel;

macro_rules! derive_sockopt {
    ($opt:ident, $level:expr, $name:expr) => {
        impl $crate::socket::SockOpt for $opt {
            fn level(&self) -> $crate::socket::SockOptLevel {
                $level
            }

            fn name(&self) -> isize {
                $name as _
            }

            fn as_ffi(&self) -> &libc::c_int {
                &self.0
            }

            fn as_ffi_mut(&mut self) -> &mut libc::c_int {
                &mut self.0
            }
        }
    };
}

macro_rules! protocol_opt {
    ($opt:ident, $level:expr, $name:expr, $ty:ty) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
        pub struct $opt(libc::c_int);

        impl $opt {
            pub fn new(val: $ty) -> Self {
                Self(val.to_cint())
            }

            pub fn value(&self) -> $ty {
                <$ty>::from_cint(self.0)
            }
        }

        derive_sockopt!($opt, $level, $name);
    };
}

trait CoerceCint {
    fn to_cint(self) -> libc::c_int;
    fn from_cint(cint: libc::c_int) -> Self;
}

impl CoerceCint for bool {
    #[inline(always)]
    fn to_cint(self) -> libc::c_int {
        if self {
            1
        } else {
            0
        }
    }

    #[inline(always)]
    fn from_cint(cint: libc::c_int) -> Self {
        cint > 0
    }
}

impl CoerceCint for u8 {
    #[inline(always)]
    fn to_cint(self) -> libc::c_int {
        self as _
    }

    #[inline(always)]
    fn from_cint(cint: libc::c_int) -> Self {
        cint as _
    }
}

impl CoerceCint for u32 {
    #[inline(always)]
    fn to_cint(self) -> libc::c_int {
        self as _
    }

    #[inline(always)]
    fn from_cint(cint: libc::c_int) -> Self {
        cint as _
    }
}

#[macro_use]
macro_rules! protocol_opt_bool {
    ($opt:ident, $level:expr, $name:expr) => {
        protocol_opt!($opt, $level, $name, bool);
    };
}

#[macro_use]
macro_rules! protocol_opt_u8 {
    ($opt:ident, $level:expr, $name:expr) => {
        protocol_opt!($opt, $level, $name, u8);
    };
}

#[macro_use]
macro_rules! protocol_opt_u32 {
    ($opt:ident, $level:expr, $name:expr) => {
        protocol_opt!($opt, $level, $name, u32);
    };
}

#[macro_use]
macro_rules! protocol_opt_enum {
    ($opt:ident, $level:expr, $name:expr, $enum:ty) => {
        protocol_opt!($opt, $level, $name, $enum);
    };
}

/// Explicit congestion notification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Ecn {
    Ect0 = 0b10,
    Ect1 = 0b01,
    Ce = 0b11,
}

impl Ecn {
    fn from_bits(bits: u8) -> Option<Self> {
        Some(match bits & 0b11 {
            0b10 => Ecn::Ect0,
            0b01 => Ecn::Ect1,
            0b11 => Ecn::Ce,
            _ => return None,
        })
    }

    fn to_bits(ecn: Option<Ecn>) -> u8 {
        ecn.map(|ecn| ecn as u8).unwrap_or(0)
    }
}

impl Ip4TOS {
    pub fn ecn(&self) -> Option<Ecn> {
        Ecn::from_bits(self.value())
    }

    pub fn set_ecn(mut self, ecn: Option<Ecn>) -> Self {
        self.0 |= Ecn::to_bits(ecn) as libc::c_int;
        self
    }
}

impl Ip6TrafficClass {
    pub fn ecn(&self) -> Option<Ecn> {
        Ecn::from_bits(self.value())
    }

    pub fn set_ecn(mut self, ecn: Option<Ecn>) -> Self {
        self.0 |= Ecn::to_bits(ecn) as libc::c_int;
        self
    }
}

/// Differentiated service code point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Dscp(u8);

impl Dscp {
    pub fn new(value: u8) -> Self {
        debug_assert!(value != 0);
        debug_assert!(value & 0b111111 == value);
        Self(value)
    }

    fn from_bits(bits: u8) -> Option<Self> {
        match bits >> 2 {
            0 => None,
            u => Some(Self(u)),
        }
    }

    fn to_bits(dscp: Option<Dscp>) -> u8 {
        dscp.map(|dscp| dscp.0).unwrap_or(0) << 2
    }
}

impl Ip4TOS {
    pub fn dscp(&self) -> Option<Dscp> {
        Dscp::from_bits(self.value())
    }

    pub fn set_dscp(mut self, dscp: Option<Dscp>) -> Self {
        self.0 |= Dscp::to_bits(dscp) as libc::c_int;
        self
    }
}

impl Ip6TrafficClass {
    pub fn dscp(&self) -> Option<Dscp> {
        Dscp::from_bits(self.value())
    }

    pub fn set_dscp(mut self, dscp: Option<Dscp>) -> Self {
        self.0 |= Dscp::to_bits(dscp) as libc::c_int;
        self
    }
}

// Traffic class
protocol_opt_bool!(Ip4RecvTOS, SockOptLevel::Ip4, libc::IP_RECVTOS);
protocol_opt_u8!(Ip4TOS, SockOptLevel::Ip4, libc::IP_TOS);
protocol_opt_bool!(
    Ip6RecvTrafficClass,
    SockOptLevel::Ip6,
    libc::IPV6_RECVTCLASS
);
protocol_opt_u8!(Ip6TrafficClass, SockOptLevel::Ip6, libc::IPV6_TCLASS);

// Hop limit
protocol_opt_bool!(Ip4RecvTTL, SockOptLevel::Ip4, /*libc::IP_RECVTTL*/ 12);
protocol_opt_u8!(Ip4TTL, SockOptLevel::Ip4, libc::IP_TTL);
protocol_opt_bool!(
    Ip6RecvHopLimit,
    SockOptLevel::Ip6,
    /*libc::IPV6_RECVHOPLIMIT*/ 51
);
protocol_opt_u32!(
    Ip6HopLimit,
    SockOptLevel::Ip6,
    /*libc::IPV6_HOPLIMIT*/ 52
);

// Flow label
protocol_opt_bool!(
    Ip6AutoFlowLabel,
    SockOptLevel::Ip6,
    /*libc::IPV6_AUTOFLOWLABEL*/ 60
);
protocol_opt_bool!(Ip6SendFlowInfo, SockOptLevel::Ip6, libc::IPV6_FLOWINFO_SEND);
protocol_opt_bool!(Ip6RecvFlowInfo, SockOptLevel::Ip6, libc::IPV6_FLOWINFO);
protocol_opt_u32!(Ip6FlowInfo, SockOptLevel::Ip6, libc::IPV6_FLOWINFO);

// Recveive error
protocol_opt_bool!(Ip4RecvErr, SockOptLevel::Ip4, libc::IP_RECVERR);
protocol_opt_bool!(
    Ip6RecvErr,
    SockOptLevel::Ip6,
    /*libc::IPV6_RECVERR*/ 25
);

// MTU
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiscoverMTU {
    Dont = 0 as isize,
    Want = 1 as isize,
    Do = 2 as isize,
    Probe = 3 as isize,
}

impl CoerceCint for DiscoverMTU {
    fn to_cint(self) -> libc::c_int {
        self as _
    }

    fn from_cint(cint: libc::c_int) -> Self {
        match cint {
            /*libc::IP_PMTUDISC_DONT*/ 0 => DiscoverMTU::Dont,
            /*libc::IP_PMTUDISC_WANT*/ 1 => DiscoverMTU::Want,
            /*libc::IP_PMTUDISC_DO*/ 2 => DiscoverMTU::Do,
            /*libc::IP_PMTUDISC_PROBE*/ 3 => DiscoverMTU::Probe,
            _ => unreachable!(),
        }
    }
}

protocol_opt_enum!(
    Ip4DiscoverMTU,
    SockOptLevel::Ip4,
    /*libc::IP_MTU_DISCOVER*/ 10,
    DiscoverMTU
);
protocol_opt_enum!(
    Ip6DiscoverMTU,
    SockOptLevel::Ip6,
    /*libc::IP_MTU_DISCOVER*/ 23,
    DiscoverMTU
);
// Only works on connected sockets
protocol_opt_u32!(Ip4MTU, SockOptLevel::Ip4, /*libc::IP_MTU*/ 14);
protocol_opt_u32!(Ip6MTU, SockOptLevel::Ip6, /*libc::IPV6_MTU*/ 25);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::tests::*;
    use crate::socket::*;
    use core::convert::TryInto;
    use std::io::ErrorKind;
    use std::net::SocketAddrV6;

    fn set_option<T>(socket: &Socket, default: T, new: T)
    where
        T: SockOpt + Copy + core::fmt::Debug + Default + Eq,
    {
        let result = getsockopt::<_, T>(socket).unwrap();
        assert_eq!(result, default);
        setsockopt(socket, new).unwrap();
        let result = getsockopt::<_, T>(socket).unwrap();
        assert_eq!(result, new);
    }

    fn send_options(socket: &Socket, to: &Socket, options: &[&dyn SockOpt]) {
        sendmsg(socket, to.addr.as_ref(), b"hello", options, MsgFlags::new()).unwrap();
    }

    fn recv_options(socket: &Socket, from: &Socket, options: &mut [&mut dyn SockOpt]) {
        let mut buf = [0u8; 10];
        let (len, addr) = recvmsg(socket, &mut buf, options, MsgFlags::new()).unwrap();
        assert_eq!(addr, from.addr.unwrap());
        assert_eq!(buf[..len], b"hello"[..]);
    }

    #[test]
    fn test_tos() {
        let (s1, s2) = socket_pair(AddressFamily::Inet, SockType::Datagram);
        set_option(&s2, Ip4RecvTOS::new(false), Ip4RecvTOS::new(true));
        let tos = Ip4TOS::default()
            .set_ecn(Some(Ecn::Ect0))
            .set_dscp(Some(Dscp::new(46)));
        send_options(&s1, &s2, &[&tos]);
        let mut tos = Ip4TOS::default();
        recv_options(&s2, &s1, &mut [&mut tos]);
        assert_eq!(tos.ecn(), Some(Ecn::Ect0));
        assert_eq!(tos.dscp(), Some(Dscp::new(46)));
    }

    #[test]
    fn test_ip6_tclass() {
        let (s1, s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        set_option(
            &s1,
            Ip6RecvTrafficClass::new(false),
            Ip6RecvTrafficClass::new(true),
        );
        set_option(
            &s2,
            Ip6RecvTrafficClass::new(false),
            Ip6RecvTrafficClass::new(true),
        );
        let tclass = Ip6TrafficClass::default()
            .set_ecn(Some(Ecn::Ect0))
            .set_dscp(Some(Dscp::new(46)));
        send_options(&s1, &s2, &[&tclass]);
        let mut tclass = Ip6TrafficClass::default();
        recv_options(&s2, &s1, &mut [&mut tclass]);
        assert_eq!(tclass.ecn(), Some(Ecn::Ect0));
        assert_eq!(tclass.dscp(), Some(Dscp::new(46)));
    }

    #[test]
    fn test_ttl() {
        let (s1, s2) = socket_pair(AddressFamily::Inet, SockType::Datagram);
        set_option(&s1, Ip4TTL::new(64), Ip4TTL::new(1));
        set_option(&s2, Ip4RecvTTL::new(false), Ip4RecvTTL::new(true));
        send_options(&s1, &s2, &[]);
        let mut ttl = Ip4TTL::default();
        recv_options(&s2, &s1, &mut [&mut ttl]);
        assert_eq!(ttl.value(), 1);
    }

    #[test]
    fn test_hoplimit() {
        let (s1, s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        set_option(&s2, Ip6RecvHopLimit::new(false), Ip6RecvHopLimit::new(true));
        send_options(&s1, &s2, &[&Ip6HopLimit::new(1)]);
        let mut hops = Ip6HopLimit::default();
        recv_options(&s2, &s1, &mut [&mut hops]);
        assert_eq!(hops.value(), 1);
    }

    #[test]
    fn test_flowlabel() {
        let (s1, mut s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        let mut addr: SocketAddrV6 = s2.addr.unwrap().try_into().unwrap();
        // flow info needs to be larger than 255
        addr.set_flowinfo(256);
        s2.addr = Some(addr.into());
        set_option(
            &s1,
            Ip6AutoFlowLabel::new(false),
            Ip6AutoFlowLabel::new(false),
        );
        set_option(&s1, Ip6SendFlowInfo::new(false), Ip6SendFlowInfo::new(true));
        set_option(&s2, Ip6RecvFlowInfo::new(false), Ip6RecvFlowInfo::new(true));
        send_options(&s1, &s2, &[]);
        let mut flow = Ip6FlowInfo::default();
        recv_options(&s2, &s1, &mut [&mut flow]);
        assert_eq!(flow.value(), 256);
    }

    #[test]
    fn test_ip4_recverr() {
        let (s1, mut s2) = socket_pair(AddressFamily::Inet, SockType::Datagram);
        set_option(&s1, Ip4RecvErr::new(false), Ip4RecvErr::new(true));
        s2.close();

        let flags = MsgFlags::new().err_queue();
        let mut buf = [0u8; 255];

        let err = recvmsg(&s1, &mut buf, &mut [], flags).err().unwrap();
        assert_eq!(err.kind(), ErrorKind::WouldBlock);

        send_options(&s1, &s2, &[]);

        let (len, addr) = recvmsg(&s1, &mut buf, &mut [], flags).unwrap();
        assert_eq!(addr, s2.addr.unwrap());
        assert_eq!(&buf[..len], &b"hello"[..]);

        // TODO get cmsg IP_RECVERR
    }

    #[test]
    fn test_ip6_recverr() {
        let (s1, mut s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        set_option(&s1, Ip6RecvErr::new(false), Ip6RecvErr::new(true));
        s2.close();

        let flags = MsgFlags::new().err_queue();
        let mut buf = [0u8; 255];

        let err = recvmsg(&s1, &mut buf, &mut [], flags).err().unwrap();
        assert_eq!(err.kind(), ErrorKind::WouldBlock);

        send_options(&s1, &s2, &[]);

        let (len, addr) = recvmsg(&s1, &mut buf, &mut [], flags).unwrap();
        assert_eq!(addr, s2.addr.unwrap());
        assert_eq!(&buf[..len], &b"hello"[..]);

        // TODO get cmsg IP_RECVERR
    }

    #[test]
    fn test_ip4_mtu_probe() {
        let (s1, s2) = socket_pair(AddressFamily::Inet, SockType::Datagram);
        set_option(
            &s2,
            Ip4DiscoverMTU::new(DiscoverMTU::Want),
            Ip4DiscoverMTU::new(DiscoverMTU::Probe),
        );
        send_options(&s1, &s2, &[]);
        recv_options(&s2, &s1, &mut []);

        // TODO test error to big
    }

    #[test]
    fn test_ip6_mtu_probe() {
        let (s1, s2) = socket_pair(AddressFamily::Inet6, SockType::Datagram);
        set_option(
            &s2,
            Ip6DiscoverMTU::new(DiscoverMTU::Want),
            Ip6DiscoverMTU::new(DiscoverMTU::Probe),
        );
        send_options(&s1, &s2, &[]);
        recv_options(&s2, &s1, &mut []);

        // TODO test error to big
    }
}
