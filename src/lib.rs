#![feature(asm)]


#[macro_use]
mod arch;


use std::os::unix::io::RawFd;
use std::io;

use arch::Syscalls;
use std::os::raw::{c_long, c_ulong};

pub unsafe fn write(fd: RawFd, msg: &[u8]) -> Result<usize, io::Error> {
    let res = syscall!(Syscalls::Write as c_long, c_long::from(fd), msg.as_ptr() as c_long, msg.len() as c_long);
    if res < 0 {
        Err(io::Error::from_raw_os_error(res as i32))
    } else {
        Ok(res as usize)
    }
}


#[cfg(test)]
#[test]
fn test_print() {
    use std::os::unix::io::AsRawFd;
    let msg = "Hello World\n";
    let res = unsafe { write(io::stdout().as_raw_fd(), msg.as_bytes()) };
    println!("res: {:?}", res);
    assert!(res.is_ok());
}