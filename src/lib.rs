#![feature(asm)]
#![allow(clippy::missing_safety_doc)]

mod arch;
pub mod socket;
pub(crate) mod utils;

use arch::Syscalls;
use std::ffi::{CStr, OsString};
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem::size_of, ptr};

// TODO: Remove libc. Currently *only* used for getting typedefs for flags.
use std::os::raw::c_char;
use std::path::PathBuf;

use libc::{SHUT_RD, SHUT_RDWR, SHUT_WR};

use linux_sys::{
    flock, timeval, AT_FDCWD, ERANGE, O_CLOEXEC, O_CREAT, O_LARGEFILE, O_TMPFILE, PATH_MAX,
    RENAME_EXCHANGE, RENAME_NOREPLACE,
};

// Checking that RawFd, raw pointers, and usize can all be losslessly casted into isize. (without losing bits)
// TODO: Is there a better way to do this? https://github.com/rust-lang/rfcs/issues/2784
static_assert!(size_of::<isize>() >= size_of::<RawFd>());
static_assert!(size_of::<isize>() >= size_of::<*const ()>());
static_assert!(size_of::<isize>() >= size_of::<usize>());

// A tool to ease manually making a file descriptor that implements a `AsRawFd`.
struct FileDescriptor(RawFd);
impl AsRawFd for FileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

const CURRENT_CWD_FD: FileDescriptor = FileDescriptor(AT_FDCWD as _);

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
pub unsafe fn open(path: &CStr, oflags: u32, mode: Option<u32>) -> io::Result<usize> {
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
pub unsafe fn open64(path: &CStr, oflags: u32, mode: Option<u32>) -> io::Result<usize> {
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

// TODO: glibc just calls mkdirat with AT_FDCWD. musl has this as an ifdef. what should we do?.
// TODO: Should we return Result<()>?.
#[inline]
pub unsafe fn mkdir(path: &CStr, mode: u32) -> io::Result<usize> {
    let res = syscall!(Syscalls::Mkdir, path.as_ptr() as isize, mode as isize);
    result!(res)
}

// TODO: same comments as for mkdir but here it's `mrdir` vs `unlinkat`
#[inline]
pub unsafe fn rmdir(path: &CStr) -> io::Result<usize> {
    let res = syscall!(Syscalls::Rmdir, path.as_ptr() as isize);
    result!(res)
}

// TODO: musl has an aio barrier, glibc uses SYSCALL_CANCEL. what should we do here?.
#[inline]
pub unsafe fn close<F: AsRawFd>(fd: &F) -> io::Result<usize> {
    let res = syscall!(Syscalls::Close, fd.as_raw_fd() as isize);
    result!(res)
}

// The timezone is useless. see man gettimeofday(2).
#[inline]
pub fn gettimeofday() -> io::Result<timeval> {
    let mut time: MaybeUninit<timeval> = MaybeUninit::uninit();
    let res = unsafe {
        syscall!(
            Syscalls::Gettimeofday,
            time.as_mut_ptr() as isize,
            ptr::null_mut::<()>() as isize
        )
    };
    if res < 0 {
        Err(io::Error::from_raw_os_error(-res as i32))
    } else {
        debug_assert_eq!(res, 0);
        unsafe { Ok(time.assume_init()) }
    }
}

// TODO: There are only 2 falgs. should we just make it an enum?(Open question 5)
#[inline]
pub unsafe fn getrandom(buf: &mut [u8], flags: Option<u32>) -> io::Result<usize> {
    let flags = flags.unwrap_or(0);
    let res = syscall!(
        Syscalls::Getrandom,
        buf.as_mut_ptr() as isize,
        buf.len() as isize,
        flags as isize
    );
    result!(res)
}

// TODO: Any better abstraction for the pid? (https://doc.rust-lang.org/std/process/struct.Child.html#method.id)
#[inline]
pub unsafe fn kill(pid: u32, signal: u32) -> io::Result<usize> {
    let res = syscall!(Syscalls::Kill, pid as isize, signal as isize);
    result!(res)
}

#[inline]
pub unsafe fn getcwd() -> io::Result<PathBuf> {
    let mut buf = Vec::with_capacity(PATH_MAX as usize);
    let res = syscall!(Syscalls::Getcwd, buf.as_mut_ptr() as isize);
    if res < 0 {
        assert_ne!((-res as u32), ERANGE);
        Err(std::io::Error::from_raw_os_error(-res as i32))
    } else {
        assert!(!(res as *const c_char).is_null()); // Should I just replace with `assert_ne!(res, 0)`?.
        let ptr = buf.as_ptr() as *const c_char;
        let len = CStr::from_ptr(ptr).to_bytes().len();
        buf.set_len(len);
        Ok(PathBuf::from(OsString::from_vec(buf)))
    }
}

#[inline]
pub unsafe fn chdir(path: &CStr) -> io::Result<usize> {
    let res = syscall!(Syscalls::Chdir, path.as_ptr() as isize);
    result!(res)
}

#[inline]
pub unsafe fn fchdir<F: AsRawFd>(fd: &F) -> io::Result<usize> {
    let res = syscall!(Syscalls::Fchdir, fd.as_raw_fd() as isize);
    result!(res)
}

// `RENAME_EXCHANGE` and `RENAME_NOREPLACE` are mutually exclusive. so make sense to have an enum.
// TODO: Linux 3.18+ supports also `RENAME_WHITEOUT`. Open Question 12.
#[non_exhaustive]
pub enum RenameAt2Flags {
    ExchangeAtomically = RENAME_EXCHANGE as isize,
    RenameOnly = RENAME_NOREPLACE as isize,
}

// TODO: Should we return Result<()>?.
// TODO: Should we check if the path is absolute and if so pass 0 for the file descriptor? should the file descriptor be optional with 0 default?
#[inline]
pub unsafe fn renameat2<F1: AsRawFd, F2: AsRawFd>(
    old_fd: &F1,
    old_path: &CStr,
    new_fd: &F2,
    new_path: &CStr,
    flags: Option<RenameAt2Flags>,
) -> io::Result<usize> {
    let flags = flags.map(|f| f as isize).unwrap_or(0);
    let res = syscall!(
        Syscalls::Renameat2,
        old_fd.as_raw_fd() as isize,
        old_path.as_ptr() as isize,
        new_fd.as_raw_fd() as isize,
        new_path.as_ptr() as isize,
        flags,
    );
    result!(res)
}

#[inline]
pub unsafe fn rename(old_path: &CStr, new_path: &CStr) -> io::Result<usize> {
    renameat2(&CURRENT_CWD_FD, old_path, &CURRENT_CWD_FD, new_path, None)
}

// TODO: Return Result<()>.
#[inline]
pub unsafe fn dup3<F1: AsRawFd, F2: AsRawFd>(
    old_fd: &F1,
    new_fd: &F2,
    close_on_exec: Option<bool>,
) -> io::Result<usize> {
    let close_on_exec = close_on_exec
        .map(|f| if f { O_CLOEXEC } else { 0 })
        .unwrap_or(0);
    let res = syscall!(
        Syscalls::Dup3,
        old_fd.as_raw_fd() as isize,
        new_fd.as_raw_fd() as isize,
        close_on_exec as isize,
    );
    result!(res)
}

// See: https://doc.rust-lang.org/std/net/enum.Shutdown.html
pub enum Shutdown {
    Write = SHUT_WR as isize,
    Read = SHUT_RD as isize,
    Both = SHUT_RDWR as isize,
}

// TODO: Should sockets have a different interface than a file descriptor?
// TODO: Missing tests.
#[inline]
pub unsafe fn shutdown<F: AsRawFd>(socket: &F, how: Shutdown) -> io::Result<usize> {
    let res = syscall!(
        Syscalls::Shutdown,
        socket.as_raw_fd() as isize,
        how as isize
    );
    result!(res)
}

// TODO: Same question as in `open(2)`. should we just implement `fchmodat(2)` and call that?.
#[inline]
pub unsafe fn chmod(path: &CStr, mode: u32) -> io::Result<usize> {
    let res = syscall!(Syscalls::Chmod, path.as_ptr() as isize, mode as isize,);
    result!(res)
}

#[inline]
pub unsafe fn getuid() -> io::Result<u32> {
    let res = syscall!(Syscalls::Getuid);
    result!(res)
}

// TODO: Not thread safe. see open question 15.
#[inline]
pub unsafe fn setuid(id: u32) -> io::Result<()> {
    let res = syscall!(Syscalls::Setuid, id as isize);
    result_none!(res)
}

pub enum FcntlArg<'a> {
    Flock(&'a mut flock),
    Flags(u32),
    None,
}

impl From<FcntlArg<'_>> for isize {
    fn from(arg: FcntlArg<'_>) -> isize {
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
pub unsafe fn fcntl<F: AsRawFd>(fd: F, cmd: u32, arg: FcntlArg<'_>) -> io::Result<usize> {
    let _ = (fd, cmd, arg);
    unimplemented!();
    // TODO: Requires a deeper thought and discussion on how these should be done best.
}

#[cfg(test)]
mod tests {
    use super::write;
    use linux_sys::{O_CLOEXEC, O_RDWR, O_SYNC, SIGTERM};
    use std::env;
    use std::ffi::{CStr, CString};
    use std::fs::{remove_file, File, OpenOptions};
    use std::io::{self, Read, Seek, SeekFrom, Write};
    use std::ops::{Deref, DerefMut};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::FromRawFd;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestFile(File, PathBuf, bool);

    impl TestFile {
        pub fn new() -> io::Result<Self> {
            let path = Self::generate_new_path();
            Self::from_path_delete(path, true)
        }

        pub fn new_dont_delete() -> io::Result<Self> {
            let path = Self::generate_new_path();
            Self::from_path_delete(path, false)
        }

        pub fn generate_new_path() -> PathBuf {
            static FILES_COUNTER: AtomicU8 = AtomicU8::new(0);
            let curr = FILES_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = PathBuf::from(".").canonicalize().unwrap();
            path.join(&format!("{}.testfile", curr))
        }

        pub fn from_path_delete(path: PathBuf, delete: bool) -> io::Result<Self> {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open(&path)?;
            Ok(TestFile(file, path, delete))
        }

        pub fn path(&self) -> &Path {
            &self.1
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            if self.2 {
                let _ = remove_file(&self.1);
            }
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

    const DUMMY_FD: super::FileDescriptor = super::FileDescriptor(-1337);

    fn path_to_cstr(path: &Path) -> CString {
        CString::new(path.as_os_str().as_bytes()).unwrap()
    }

    #[test]
    fn test_chmod() {
        let mut file = TestFile::new_dont_delete().unwrap();
        let data = b"syscalls are cool";
        file.write_all(data).unwrap();
        let p_path = file.path().to_owned();
        let path = path_to_cstr(&p_path);
        drop(file);
        let res = unsafe { super::chmod(&path, 0o000) }.unwrap();
        assert_eq!(res, 0);
        let err = unsafe { super::open(&path, O_CLOEXEC | O_SYNC | O_RDWR, None) }.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(err.to_string(), "Permission denied (os error 13)");

        remove_file(&p_path).unwrap();
    }

    #[test]
    // Requires running with `cargo test -- --test-threads 1` because it changes the cwd.
    fn test_cwd() {
        let original = env::current_dir().unwrap();
        let raw = unsafe { super::getcwd() }.unwrap();
        assert_eq!(original, raw);

        let path = CStr::from_bytes_with_nul(b"../\0").unwrap();
        let r = unsafe { super::chdir(path) }.unwrap();
        assert_eq!(r, 0);
        let raw = unsafe { super::getcwd() }.unwrap();
        let mut popped = original.clone();
        popped.pop();
        assert_eq!(popped, raw);

        let path = CStr::from_bytes_with_nul(b"./syscalls-rs\0").unwrap();
        let fd = super::FileDescriptor(unsafe { super::open(path, O_CLOEXEC, None) }.unwrap() as _);
        let r = unsafe { super::fchdir(&fd) }.unwrap();
        assert_eq!(r, 0);
        let raw = unsafe { super::getcwd() }.unwrap();
        assert_eq!(original, raw);
    }

    #[test]
    fn test_rename() {
        let mut file1 = TestFile::new().unwrap();
        let data = b"syscalls are cool";
        file1.write_all(data).unwrap();

        let f2 = TestFile::generate_new_path();
        let res =
            unsafe { super::rename(&path_to_cstr(file1.path()), &path_to_cstr(&f2)) }.unwrap();
        assert_eq!(res, 0);

        let mut res = [0u8; 17];
        let mut file2 = File::open(&f2).unwrap();
        file2.read_exact(&mut res).unwrap();

        assert_eq!(res, *data);
        drop((file1, file2));
        remove_file(f2).unwrap();
    }

    #[test]
    fn test_renameat2() {
        let one = b"File1";
        let two = b"File2";
        let mut file1 = TestFile::new().unwrap();
        let mut file2 = TestFile::new().unwrap();
        file1.write_all(one).unwrap();
        file2.write_all(two).unwrap();
        let path1 = path_to_cstr(file1.path());
        let path2 = path_to_cstr(file2.path());
        let curr_dir = File::open(env::current_dir().unwrap()).unwrap();
        let res = unsafe {
            super::renameat2(
                &curr_dir,
                &path1,
                &curr_dir,
                &path2,
                Some(super::RenameAt2Flags::ExchangeAtomically),
            )
        }
        .unwrap();
        assert_eq!(res, 0);

        let mut file1_after = File::open(file1.path()).unwrap();
        let mut file2_after = File::open(file2.path()).unwrap();

        let mut res = [0u8; 5];
        file1_after.read_exact(&mut res).unwrap();
        assert_eq!(res, *two);

        file2_after.read_exact(&mut res).unwrap();
        assert_eq!(res, *one);
    }

    #[test]
    fn test_kill() {
        let mut child = Command::new("cat").spawn().unwrap();
        let res = unsafe { super::kill(child.id(), SIGTERM) }.unwrap();
        assert_eq!(res, 0);
        let res = child.wait().unwrap().code();
        assert_eq!(res, None);
    }

    #[test]
    fn test_rand() {
        let mut buf = [0u8; 32];
        let res = unsafe { super::getrandom(&mut buf, None) }.unwrap();
        assert_eq!(res, buf.len());
        assert_ne!(buf, [0u8; 32]);
    }

    #[test]
    fn test_gettimeofday() {
        const MICROS: i128 = 1_000_000;
        let sys_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as i128;
        let time = super::gettimeofday().unwrap();
        let gettime: i128 = time.tv_usec as i128 + (time.tv_sec as i128 * MICROS);
        let diff = sys_time - gettime;
        assert!(diff.abs() < MICROS); // the diff is less than a second.
    }

    #[test]
    fn test_close() {
        let file = TestFile::new().unwrap();
        let res = unsafe { super::close(file.deref()) }.unwrap();
        assert_eq!(res, 0);
        let err = unsafe { super::close(file.deref()) }.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "Bad file descriptor (os error 9)");
    }

    #[test]
    fn test_mkdir_rmdir() {
        let path = TestFile::generate_new_path();
        let c_path = path_to_cstr(&path);
        println!("{:?}", path);
        let res = unsafe { super::mkdir(&c_path, O_RDWR) }.unwrap();
        assert_eq!(res, 0);
        assert!(path.exists());
        let res = unsafe { super::rmdir(&c_path) }.unwrap();
        assert_eq!(res, 0);
        assert!(!path.exists());
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
    #[ignore]
    // root only.
    fn test_getuid() {
        unsafe {
            let orig = super::getuid().unwrap();
            super::setuid(7500).unwrap();
            assert_ne!(super::getuid().unwrap(), orig);
        }

    }


    #[test]
    fn test_open() {
        let src = b"Hello World";
        let mut dest = [0u8; 11];
        let path = path_to_cstr(&TestFile::generate_new_path());
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
    fn test_write_fail() {
        let msg = "Hello World\n";
        let res = unsafe { write(&mut DUMMY_FD, msg.as_bytes()) };
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "Bad file descriptor (os error 9)");
    }
}
