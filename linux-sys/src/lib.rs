//!
//! # linux-sys
//! Notice: All the types here are autogenerated at build time per platform.
//! The fact that you see `type Foo = u8` doesn't mean you can just use `u8`.
//! please use the types directly as they can change per target/platform.

pub mod fcntl;
pub mod fs;
pub mod time;
pub mod signal;
