use std::os::raw::{c_long, c_ulong};

#[inline]
pub unsafe fn syscall0(n: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall1(n: c_long, a1: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall2(n: c_long, a1: c_long, a2: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) , "{rsi}"(a2)
          : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall3(n: c_long, a1: c_long, a2: c_long, a3: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) , "{rsi}"(a2),
          "{rdx}"(a3) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall4(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) , "{rsi}"(a2),
          "{rdx}"(a3), "{r10}"(a4) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall5(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long, a5: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) , "{rsi}"(a2),
          "{rdx}"(a3), "{r10}"(a4), "{r8}"(a5) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}

#[inline]
pub unsafe fn syscall6(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long, a5: c_long, a6: c_long) -> c_long {
    let mut ret: c_ulong;
    asm! {"syscall": "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) , "{rsi}"(a2),
          "{rdx}"(a3), "{r10}"(a4), "{r8}"(a5), "{r9}"(a6) : "rcx", "r11", "memory" : "volatile"};
    ret as c_long
}










