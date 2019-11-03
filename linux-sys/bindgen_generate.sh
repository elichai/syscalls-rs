#!/bin/sh
# set -x

PLATFORMS=(arm arm64 hexagon mips powerpc riscv s390 sparc x86)
BINDGEN_RAW_LINES="#![allow(dead_code,non_camel_case_types,non_snake_case)]"
BINDGEN_FLAGS="--default-enum-style=rust --rust-target=nightly --use-array-pointers-in-arguments --raw-line $BINDGEN_RAW_LINES"

fcntl="asm/fcntl.h"

for platform in "${PLATFORMS[@]}"; do
    headers="./linux/arch/$platform/include/uapi"
    
    [ -f $headers/$fcntl ] && C_INCLUDE_PATH=$headers bindgen $BINDGEN_FLAGS $headers/$fcntl -o  ./src/$platform/fcntl.rs
done

bindgen $BINDGEN_FLAGS ./linux/include/uapi/asm-generic/fcntl.h -o  ./src/generic/fcntl.rs