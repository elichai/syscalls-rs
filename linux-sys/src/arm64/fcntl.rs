/* automatically generated by rust-bindgen */

#![allow(dead_code,non_camel_case_types,non_snake_case)]

pub const O_DIRECTORY: u32 = 16384;
pub const O_NOFOLLOW: u32 = 32768;
pub const O_DIRECT: u32 = 65536;
pub const O_LARGEFILE: u32 = 131072;
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const O_ACCMODE: u32 = 3;
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 64;
pub const O_EXCL: u32 = 128;
pub const O_NOCTTY: u32 = 256;
pub const O_TRUNC: u32 = 512;
pub const O_APPEND: u32 = 1024;
pub const O_NONBLOCK: u32 = 2048;
pub const O_DSYNC: u32 = 4096;
pub const FASYNC: u32 = 8192;
pub const O_NOATIME: u32 = 262144;
pub const O_CLOEXEC: u32 = 524288;
pub const __O_SYNC: u32 = 1048576;
pub const O_SYNC: u32 = 1052672;
pub const O_PATH: u32 = 2097152;
pub const __O_TMPFILE: u32 = 4194304;
pub const O_TMPFILE: u32 = 4210688;
pub const O_TMPFILE_MASK: u32 = 4210752;
pub const O_NDELAY: u32 = 2048;
pub const F_DUPFD: u32 = 0;
pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_SETFL: u32 = 4;
pub const F_GETLK: u32 = 5;
pub const F_SETLK: u32 = 6;
pub const F_SETLKW: u32 = 7;
pub const F_SETOWN: u32 = 8;
pub const F_GETOWN: u32 = 9;
pub const F_SETSIG: u32 = 10;
pub const F_GETSIG: u32 = 11;
pub const F_GETLK64: u32 = 12;
pub const F_SETLK64: u32 = 13;
pub const F_SETLKW64: u32 = 14;
pub const F_SETOWN_EX: u32 = 15;
pub const F_GETOWN_EX: u32 = 16;
pub const F_GETOWNER_UIDS: u32 = 17;
pub const F_OFD_GETLK: u32 = 36;
pub const F_OFD_SETLK: u32 = 37;
pub const F_OFD_SETLKW: u32 = 38;
pub const F_OWNER_TID: u32 = 0;
pub const F_OWNER_PID: u32 = 1;
pub const F_OWNER_PGRP: u32 = 2;
pub const FD_CLOEXEC: u32 = 1;
pub const F_RDLCK: u32 = 0;
pub const F_WRLCK: u32 = 1;
pub const F_UNLCK: u32 = 2;
pub const F_EXLCK: u32 = 4;
pub const F_SHLCK: u32 = 8;
pub const LOCK_SH: u32 = 1;
pub const LOCK_EX: u32 = 2;
pub const LOCK_NB: u32 = 4;
pub const LOCK_UN: u32 = 8;
pub const LOCK_MAND: u32 = 32;
pub const LOCK_READ: u32 = 64;
pub const LOCK_WRITE: u32 = 128;
pub const LOCK_RW: u32 = 192;
pub const F_LINUX_SPECIFIC_BASE: u32 = 1024;
pub const F_SETLEASE: u32 = 1024;
pub const F_GETLEASE: u32 = 1025;
pub const F_CANCELLK: u32 = 1029;
pub const F_DUPFD_CLOEXEC: u32 = 1030;
pub const F_NOTIFY: u32 = 1026;
pub const F_SETPIPE_SZ: u32 = 1031;
pub const F_GETPIPE_SZ: u32 = 1032;
pub const F_ADD_SEALS: u32 = 1033;
pub const F_GET_SEALS: u32 = 1034;
pub const F_SEAL_SEAL: u32 = 1;
pub const F_SEAL_SHRINK: u32 = 2;
pub const F_SEAL_GROW: u32 = 4;
pub const F_SEAL_WRITE: u32 = 8;
pub const F_SEAL_FUTURE_WRITE: u32 = 16;
pub const F_GET_RW_HINT: u32 = 1035;
pub const F_SET_RW_HINT: u32 = 1036;
pub const F_GET_FILE_RW_HINT: u32 = 1037;
pub const F_SET_FILE_RW_HINT: u32 = 1038;
pub const RWF_WRITE_LIFE_NOT_SET: u32 = 0;
pub const RWH_WRITE_LIFE_NONE: u32 = 1;
pub const RWH_WRITE_LIFE_SHORT: u32 = 2;
pub const RWH_WRITE_LIFE_MEDIUM: u32 = 3;
pub const RWH_WRITE_LIFE_LONG: u32 = 4;
pub const RWH_WRITE_LIFE_EXTREME: u32 = 5;
pub const DN_ACCESS: u32 = 1;
pub const DN_MODIFY: u32 = 2;
pub const DN_CREATE: u32 = 4;
pub const DN_DELETE: u32 = 8;
pub const DN_RENAME: u32 = 16;
pub const DN_ATTRIB: u32 = 32;
pub const DN_MULTISHOT: u32 = 2147483648;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
pub const AT_REMOVEDIR: u32 = 512;
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
pub const AT_NO_AUTOMOUNT: u32 = 2048;
pub const AT_EMPTY_PATH: u32 = 4096;
pub const AT_STATX_SYNC_TYPE: u32 = 24576;
pub const AT_STATX_SYNC_AS_STAT: u32 = 0;
pub const AT_STATX_FORCE_SYNC: u32 = 8192;
pub const AT_STATX_DONT_SYNC: u32 = 16384;
pub const AT_RECURSIVE: u32 = 32768;
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
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
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
pub struct f_owner_ex {
    pub type_: ::std::os::raw::c_int,
    pub pid: __kernel_pid_t,
}
#[test]
fn bindgen_test_layout_f_owner_ex() {
    assert_eq!(
        ::std::mem::size_of::<f_owner_ex>(),
        8usize,
        concat!("Size of: ", stringify!(f_owner_ex))
    );
    assert_eq!(
        ::std::mem::align_of::<f_owner_ex>(),
        4usize,
        concat!("Alignment of ", stringify!(f_owner_ex))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<f_owner_ex>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(f_owner_ex),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<f_owner_ex>())).pid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(f_owner_ex),
            "::",
            stringify!(pid)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,
}
#[test]
fn bindgen_test_layout_flock() {
    assert_eq!(
        ::std::mem::size_of::<flock>(),
        32usize,
        concat!("Size of: ", stringify!(flock))
    );
    assert_eq!(
        ::std::mem::align_of::<flock>(),
        8usize,
        concat!("Alignment of ", stringify!(flock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_whence as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_whence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_start as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_pid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_pid)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock64 {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,
}
#[test]
fn bindgen_test_layout_flock64() {
    assert_eq!(
        ::std::mem::size_of::<flock64>(),
        32usize,
        concat!("Size of: ", stringify!(flock64))
    );
    assert_eq!(
        ::std::mem::align_of::<flock64>(),
        8usize,
        concat!("Alignment of ", stringify!(flock64))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock64>())).l_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flock64),
            "::",
            stringify!(l_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock64>())).l_whence as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(flock64),
            "::",
            stringify!(l_whence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock64>())).l_start as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(flock64),
            "::",
            stringify!(l_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock64>())).l_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(flock64),
            "::",
            stringify!(l_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock64>())).l_pid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(flock64),
            "::",
            stringify!(l_pid)
        )
    );
}
