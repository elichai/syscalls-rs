mod x86_64;

pub use x86_64::*;

macro_rules! syscall {
    ($n:expr) => {
        syscall0($n)
    };
    ($n:expr, $a1:expr) => {
        syscall1($n, $a1)
    };
    ($n:expr, $a1:expr, $a2:expr) => {
        syscall2($n, $a1, $a2)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => {
        syscall3($n, $a1, $a2, $a3)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        syscall4($n, $a1, $a2, $a3, $a4)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        syscall5($n, $a1, $a2, $a3, $a4, $a5)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a6:expr) => {
        syscall6($n, $a1, $a2, $a3, $a4, $a6)
    };
}

#[cfg(test)]
#[test]
fn test_print() {
    let write = 1;
    let msg = "Hello World\n";
    let res = unsafe { syscall!(write, 1, msg.as_ptr() as _, msg.len() as _) };
    println!("res: {}", res);
}
