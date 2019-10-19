#![feature(asm)]
#![allow(clippy::missing_safety_doc)]

mod arch;
pub(crate) mod utils;

use arch::Syscalls;
use std::ffi::CStr;
use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem::size_of};

// TODO: Remove libc. Currently *only* used for getting typedefs for flags.
use libc::{flock, O_CREAT, O_LARGEFILE, O_TMPFILE};

// Checking that RawFd, raw pointers, and usize can all be losslessly casted into isize. (without losing bits)
// TODO: Is there a better way to do this? https://github.com/rust-lang/rfcs/issues/2784
static_assert!(size_of::<isize>() >= size_of::<RawFd>());
static_assert!(size_of::<isize>() >= size_of::<*const ()>());
static_assert!(size_of::<isize>() >= size_of::<usize>());

// TODO: Is there any way to make this safe? https://github.com/rust-lang/rfcs/issues/1043#issuecomment-542904091
// TODO: Read into all ways that writing the a "bad" file descriptor violate rust's safety.
// TODO: Or find a way to make a trait that shifts the responsibility of saftey to the implementor of the trait.
// TODO Update: So if we have an unsafe trait for `AsRawFd` than that will shift the responsibility to the implementor and should allow us to make this function safe.
#[inline]
pub unsafe fn write<F: AsRawFd>(fd: &mut F, msg: &[u8]) -> io::Result<usize> {
    let res = syscall!(
        Syscalls::Write,
        fd.as_raw_fd() as isize,
        msg.as_ptr() as isize,
        msg.len() as isize
    );
    result!(res)
}

#[inline]
pub unsafe fn read<F: AsRawFd>(fd: &F, buf: &mut [u8]) -> io::Result<usize> {
    let res = syscall!(
        Syscalls::Read,
        fd.as_raw_fd() as isize,
        buf.as_mut_ptr() as isize,
        buf.len() as isize
    );
    result!(res)
}

// TODO: Should we just call openat? (that's what glibc and the kernel itself do).
// In kernels older than 3.2 this requires a special racy handling for FD_CLOEXEC. But rust doesn't support these kernels anyway https://github.com/rust-lang/libc/issues/1412#issuecomment-543621431
#[inline]
pub unsafe fn open(path: &CStr, oflags: i32, mode: Option<u32>) -> io::Result<usize> {
    // TODO: Look into a `#ifdef __O_TMPFILE` in glibc. are there times when we don't care about this? Maybe old kernels?.
    let mut mode_t = 0;
    if (oflags & O_CREAT) != 0 || (oflags & O_TMPFILE) == O_TMPFILE {
        if let Some(mode) = mode {
            mode_t = mode;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Used O_CREAT/O_TMPFILE but didn't provide a mode",
            ));
        }
    }
    let res = syscall!(
        Syscalls::Open,
        path.as_ptr() as isize,
        oflags as isize,
        mode_t as isize
    );
    result!(res)
}

// TODO: maybe this should just be the default?.
#[inline]
pub unsafe fn open64(path: &CStr, oflags: i32, mode: Option<u32>) -> io::Result<usize> {
    open(path, oflags | O_LARGEFILE, mode)
}

// TODO: Is there any reason to make this unsafe?.
#[inline]
pub fn _exit(status: i32) -> ! {
    loop {
        unsafe {
            syscall!(Syscalls::ExitGroup, status as isize);
            syscall!(Syscalls::Exit, status as isize);
        }
    }
}

pub enum FcntlArg<'a> {
    Flock(&'a mut flock),
    Flags(u32),
    None,
}

impl From<FcntlArg<'_>> for isize {
    fn from(arg: FcntlArg) -> isize {
        match arg {
            FcntlArg::Flock(r) => r as *mut flock as isize,
            FcntlArg::Flags(flag) => flag as isize,
            FcntlArg::None => 0isize,
        }
    }
}

// Should I use the flock struct internally only and expose those details to in normal rust types?.
pub enum FcntlCommand<'a, F: AsRawFd> {
    Duplicate(&'a F), // TODO: Better abstraction for these 2 modes?
    DuplicateCloseExec(&'a F),
    GetFlags,
    SetFlags(i32),
    GetFileStatusAndAccessMode,
    SetFileStatus(i32),
    SetLock(&'a flock),
    SetLockWait(&'a flock),
    GetLock(&'a mut flock), // TODO: Consider making it a return value.
}

impl<F: AsRawFd> FcntlCommand<'_, F> {
    pub fn as_isize(&self) -> isize {
        use libc::{
            F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_GETFL, F_GETLK, F_SETFD, F_SETFL, F_SETLK,
            F_SETLKW,
        };
        use FcntlCommand::*;
        (match self {
            Duplicate(_) => F_DUPFD,
            DuplicateCloseExec(_) => F_DUPFD_CLOEXEC,
            GetFlags => F_GETFD,
            SetFlags(_) => F_SETFD,
            GetFileStatusAndAccessMode => F_GETFL,
            SetFileStatus(_) => F_SETFL,
            SetLock(_) => F_SETLK,
            SetLockWait(_) => F_SETLKW,
            GetLock(_) => F_GETLK,
        }) as isize
    }
}

// TODO: Both musl and glibc has ifdefs on `__USE_FILE_OFFSET64` and `__USE_LARGEFILE64` on 32bit machines. for a bigger `off_t` in flock.
#[inline]
pub unsafe fn fcntl<F: AsRawFd>(fd: F, cmd: u32, arg: FcntlArg) -> io::Result<usize> {
    let _ = (fd, cmd, arg);
    unimplemented!();
    // TODO: Requires a deeper thought and discussion on how these should be done best.
}

#[cfg(test)]
mod tests {
    use super::write;
    use std::ffi::CString;
    use std::fs::{remove_file, File, OpenOptions};
    use std::io::{self, Seek, SeekFrom, Write};
    use std::ops::{Deref, DerefMut};
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
    use std::path::PathBuf;
    use std::thread::current;

    use libc::{O_CLOEXEC, O_SYNC};

    struct TestFile(File, PathBuf);

    impl TestFile {
        pub fn new() -> io::Result<Self> {
            let path = PathBuf::from(&format!("{:?}.testfile", current().id()));
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open(&path)?;
            Ok(TestFile(file, path))
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            let _ = remove_file(&self.1);
        }
    }

    impl Deref for TestFile {
        type Target = File;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for TestFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[test]
    #[ignore]
    fn test_exit_pass() {
        super::_exit(0);
    }

    #[test]
    #[ignore]
    fn test_exit_fail() {
        super::_exit(1);
    }

    #[test]
    fn test_open() {
        let src = b"Hello World";
        let mut dest = [0u8; 11];
        let path = CString::new(format!("{:?}.testfile", current().id())).unwrap();
        let mut file = File::create(path.to_str().unwrap()).unwrap();
        file.write_all(src).unwrap();
        drop(file);

        let fd = unsafe { super::open(&path, O_CLOEXEC | O_SYNC, None) }.unwrap();
        let file = unsafe { File::from_raw_fd(fd as i32) };
        let res = unsafe { super::read(&file, &mut dest) }.unwrap();
        let _ = remove_file(path.to_str().unwrap());
        assert_eq!(res, src.len());
        assert_eq!(&dest, src);
    }

    #[test]
    fn test_read() {
        let src = b"Hello World";
        let mut dest = [0u8; 11];
        let mut file = TestFile::new().unwrap();
        file.write_all(src).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let res = unsafe { super::read(file.deref(), &mut dest) }.unwrap();
        assert_eq!(res, src.len());
        assert_eq!(&dest, src);
    }

    #[test]
    fn test_print() {
        let msg = "Hello World\n";
        let res = unsafe { write(&mut io::stdout(), msg.as_bytes()) }.unwrap();
        assert_eq!(res, msg.len());
    }

    #[test]
    fn test_fail() {
        struct A;
        impl AsRawFd for A {
            fn as_raw_fd(&self) -> RawFd {
                -1
            }
        }
        let msg = "Hello World\n";
        let res = unsafe { write(&mut A, msg.as_bytes()) };
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "Bad file descriptor (os error 9)");
    }
}
