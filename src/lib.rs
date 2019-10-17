#![feature(asm)]

mod arch;
pub(crate) mod utils;

use arch::Syscalls;
use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem::size_of};

// Checking that RawFd, raw pointers, and usize can all be losslessly casted into isize. (without losing bits)
// TODO: Is there a better way to do this? https://github.com/rust-lang/rfcs/issues/2784
static_assert!(size_of::<isize>() >= size_of::<RawFd>());
static_assert!(size_of::<isize>() >= size_of::<*const ()>());
static_assert!(size_of::<isize>() >= size_of::<usize>());

// TODO: Is there any way to make this safe? https://github.com/rust-lang/rfcs/issues/1043#issuecomment-542904091
// TODO: Read into all ways that writing the a "bad" file descriptor violate rust's safety.
// TODO: Or find a way to make a trait that shifts the responsibility of saftey to the implementor of the trait.
// TODO Update: So if we have an unsafe trait for `AsRawFd` than that will shift the responsibility to the implementor and should allow us to make this function safe.
pub unsafe fn write<F: AsRawFd>(fd: &mut F, msg: &[u8]) -> Result<usize, io::Error> {
    let res = syscall!(
        Syscalls::Write.into(),
        fd.as_raw_fd() as isize,
        msg.as_ptr() as isize,
        msg.len() as isize
    );
    if res < 0 {
        // TODO: Is there a better way to do this then negating twice? maybe checking if the MSB is set? is that even better?
        // TODO: Add our own Error enum with all the errors in errno.h
        Err(io::Error::from_raw_os_error(-res as i32))
    } else {
        Ok(res as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::write;
    use std::io;
    use std::os::unix::io::{AsRawFd, RawFd};

    #[test]
    fn test_print() {
        let msg = "Hello World\n";
        let res = unsafe { write(&mut io::stdout(), msg.as_bytes()) };
        println!("res: {:?}", res);
        assert!(res.is_ok());
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
        println!("res: {:?}", res);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "Bad file descriptor (os error 9)");
    }
}
