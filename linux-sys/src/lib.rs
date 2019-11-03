mod generic;



#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "arm")]
pub use arm::*;

#[cfg(target_arch = "aarch64")]
mod arm64;
#[cfg(target_arch = "aarch64")]
pub use arm64::*;

#[cfg(target_arch = "hexagon")]
mod hexagon;
#[cfg(target_arch = "hexagon")]
pub use hexagon::*;

#[cfg(target_arch = "mips")]
mod mips;
#[cfg(target_arch = "mips")]
pub use hexagon::*;


#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
mod powerpc;
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub use powerpc::*;

#[cfg(any(target_arch = "riscv", target_arch = "riscv64"))]
mod riscv;
#[cfg(any(target_arch = "riscv", target_arch = "riscv64"))]
pub use riscv::*;

#[cfg(target_arch = "s390x")]
mod s390;
#[cfg(target_arch = "s390x")]
pub use s390::*;

#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
mod sparc;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub use sparc::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::*;