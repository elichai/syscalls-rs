#!/usr/bin/env python3

from collections import namedtuple
import os
import subprocess
import os.path
import sys

PLATFORMS = ["arm", "arm64", "hexagon", "mips",
             "powerpc", "riscv", "s390", "sparc", "x86"]
BINDGEN_RAW_LINES = "'#![allow(dead_code,non_camel_case_types,non_snake_case)]'"
BINDGEN_FLAGS = "--default-enum-style=rust --rust-target=nightly --use-array-pointers-in-arguments --raw-line " + BINDGEN_RAW_LINES
CLANG_FLAGS = "-nostdinc"


headers_list = ["fcntl", "time", "fs"]


def call(command, cwd=None):
    print(command)
    try:
        subprocess.check_call(command, shell=True, cwd=cwd)
    except subprocess.CalledProcessError as e:
        print("Execution failed, error:", e.returncode, file=sys.stderr)
        exit(e.returncode)


def exists(cmd, how_to_install):
    try:
        subprocess.check_call("which " + cmd, shell=True)
    except subprocess.CalledProcessError as e:
        print("command: ", cmd, "Doesn't exists")
        print("Please install using:", how_to_install)
        print(e)
        exit(1)


def generate_bindings():
    for platform in PLATFORMS:
        platform_path = "./headers_install/{}/include".format(platform)
        for header in headers_list:
            path = platform_path + "/linux"
            file = path + "/" + header + ".h"
            output = "./src/{}/{}.rs".format(platform, header)
            if os.path.exists(file):
                call("bindgen {} {} -o {} -- {} -I{}".format(BINDGEN_FLAGS,
                                                             file, output, CLANG_FLAGS, platform_path))


def generate_headers():
    for platform in PLATFORMS:
        command = "make headers_install ARCH={} INSTALL_HDR_PATH=../headers_install/{}".format(
            platform, platform)
        call(command, "./linux")


def main():
    exists("bindgen", "cargo install bindgen")
    print("Generating linux headers")
    generate_headers()
    print("Generating rust bindings")
    generate_bindings()


if __name__ == "__main__":
    # execute only if run as a script
    main()
