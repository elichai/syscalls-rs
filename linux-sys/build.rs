use bindgen::{builder, EnumVariation, RustTarget};
use std::env;
use std::path::PathBuf;

const ISSUE: &str = "https://github.com/elichai/syscalls-rs/issues";

// We can rely on include order to first include our `#include <linux/*.h>` before the system ones as long as the compiler is unix compliant
// See: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/c99.html
// https://gcc.gnu.org/onlinedocs/cpp/Invocation.html#Invocation
// https://llvm.org/docs/CodingStandards.html#include-style

fn main() {
    let path = get_target_arch_dir();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut build = builder();
    if cfg!(feature = "cty") {
        build = build.ctypes_prefix("::cty");
    };
    build
        .use_core()
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .rust_target(RustTarget::Nightly)
        .array_pointers_in_arguments(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        //        .clang_arg("-nostdinc") // Do we need to disable C's std or not? stdlib enabled because of size_t
        .clang_arg(format!("-I{}/include", path))
//        .header("stddef.h") // This is required because of missing `size_t` declaration in the header.
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings, wrapper.h")
        .write_to_file(out_path.join("wrapper.rs"))
        .expect("Couldn't write bindings to file");
}

fn unrecognized_arch() -> ! {
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "unix" {
        let target = env::var("TARGET").unwrap();
        panic!(
            "unix target isn't recognize: {}, please report to: {}",
            target, ISSUE
        );
    } else {
        panic!("Tried to use linux-sys on a non unix OS");
    }
}

fn get_target_arch_dir() -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let str_arch = match arch.as_str() {
        "i686" | "x86_64" | "i586" | "x86" => "x86",
        "aarch64" => "arm64",
        "arm" | "armv5te" | "armv7" | "armv4t" | "thumbv7neon" => "arm",
        "mips" | "mips64" | "mips64el" | "mipsel" | "mipsisa32r6" | "mipsisa32r6el"
        | "mipsisa64r6" | "mipsisa64r6el" => "mips",
        "powerpc" | "powerpc64" | "powerpc64le" => "powerpc",
        "riscv32imac" | "riscv32imc" | "riscv64gc" | "riscv64imac" | "riscv32i" => "riscv",
        "s390x" => "s390",
        "sparc64" | "sparcv9" | "sparc" => "sparc",
        "hexagon" => "hexagon",
        "armebv7r" | "asmjs" | "i386" | "thumbv6m" | "thumbv7em" | "thumbv7m" | "wasm32"
        | "armv6" | "msp430" | "nvptx64" | "thumbv7a" | "thumbv8m" => unrecognized_arch(),
        _ => panic!("Unrecognized arch: {}. please report to: {}", arch, ISSUE),
    };
    format!("./headers_install/{}", str_arch)
}
