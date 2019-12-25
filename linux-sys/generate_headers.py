#!/usr/bin/env python3

import subprocess
import sys

PLATFORMS = ["arm", "arm64", "hexagon", "mips",
             "powerpc", "riscv", "s390", "sparc", "x86"]


def call(command, cwd=None):
    print(command)
    try:
        subprocess.check_call(command, shell=True, cwd=cwd)
    except subprocess.CalledProcessError as e:
        print("Execution failed, error:", e.returncode, file=sys.stderr)
        exit(e.returncode)


def generate_headers():
    for platform in PLATFORMS:
        command = "make headers_install ARCH={} INSTALL_HDR_PATH=../headers_install/{}".format(
            platform, platform)
        call(command, "./linux")


def main():
    print("Generating linux headers")
    generate_headers()


if __name__ == "__main__":
    main()
