/* automatically generated by rust-bindgen */

#![allow(dead_code,non_camel_case_types,non_snake_case)]

pub const __FD_SETSIZE: u32 = 1024;
pub const ITIMER_REAL: u32 = 0;
pub const ITIMER_VIRTUAL: u32 = 1;
pub const ITIMER_PROF: u32 = 2;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_SGI_CYCLE: u32 = 10;
pub const CLOCK_TAI: u32 = 11;
pub const MAX_CLOCKS: u32 = 16;
pub const CLOCKS_MASK: u32 = 1;
pub const CLOCKS_MONO: u32 = 1;
pub const TIMER_ABSTIME: u32 = 1;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout___kernel_timespec() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_timespec>(),
        16usize,
        concat!("Size of: ", stringify!(__kernel_timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec,
    pub it_value: __kernel_timespec,
}
#[test]
fn bindgen_test_layout___kernel_itimerspec() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_itimerspec>(),
        32usize,
        concat!("Size of: ", stringify!(__kernel_itimerspec))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_itimerspec>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_itimerspec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_itimerspec>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_itimerspec),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_itimerspec>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_itimerspec),
            "::",
            stringify!(it_value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}
#[test]
fn bindgen_test_layout___kernel_old_timeval() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_old_timeval>(),
        16usize,
        concat!("Size of: ", stringify!(__kernel_old_timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_old_timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_old_timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_old_timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_old_timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_old_timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_old_timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
#[test]
fn bindgen_test_layout___kernel_sock_timeval() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_sock_timeval>(),
        16usize,
        concat!("Size of: ", stringify!(__kernel_sock_timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_sock_timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_sock_timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_sock_timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_sock_timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_sock_timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_sock_timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __kernel_time_t,
    pub tv_nsec: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __kernel_time_t,
    pub tv_usec: __kernel_suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: ::std::os::raw::c_int,
    pub tz_dsttime: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_timezone() {
    assert_eq!(
        ::std::mem::size_of::<timezone>(),
        8usize,
        concat!("Size of: ", stringify!(timezone))
    );
    assert_eq!(
        ::std::mem::align_of::<timezone>(),
        4usize,
        concat!("Alignment of ", stringify!(timezone))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timezone>())).tz_minuteswest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_minuteswest)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timezone>())).tz_dsttime as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_dsttime)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[test]
fn bindgen_test_layout_itimerspec() {
    assert_eq!(
        ::std::mem::size_of::<itimerspec>(),
        32usize,
        concat!("Size of: ", stringify!(itimerspec))
    );
    assert_eq!(
        ::std::mem::align_of::<itimerspec>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerspec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerspec>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerspec>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
#[test]
fn bindgen_test_layout_itimerval() {
    assert_eq!(
        ::std::mem::size_of::<itimerval>(),
        32usize,
        concat!("Size of: ", stringify!(itimerval))
    );
    assert_eq!(
        ::std::mem::align_of::<itimerval>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerval>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerval>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_value)
        )
    );
}
