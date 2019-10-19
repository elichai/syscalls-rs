#[cfg(target_arch = "x86")]
mod i386;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86")]
pub use i386::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[macro_export]
macro_rules! syscall {
    ($n:expr) => {
        crate::arch::syscall0($n.into())
    };
    ($n:expr, $a1:expr) => {
        crate::arch::syscall1($n.into(), $a1)
    };
    ($n:expr, $a1:expr, $a2:expr) => {
        crate::arch::syscall2($n.into(), $a1, $a2)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => {
        crate::arch::syscall3($n.into(), $a1, $a2, $a3)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        crate::arch::syscall4($n.into(), $a1, $a2, $a3, $a4)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        crate::arch::syscall5($n.into(), $a1, $a2, $a3, $a4, $a5)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a6:expr) => {
        crate::arch::syscall6($n.into(), $a1, $a2, $a3, $a4, $a6)
    };
}

impl From<Syscalls> for isize {
    fn from(sys: Syscalls) -> isize {
        sys as isize
    }
}
