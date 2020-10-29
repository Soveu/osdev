#!/usr/bin/python

BOOTLOADER_PATH_DEBUG = "goodloader/target/i686-goodloader-none/debug/goodloader"
BOOTLOADER_PATH_RELEASE = "goodloader/target/i686-goodloader-none/release/goodloader"

KERNEL_PATH_DEBUG = "kernel/target/amd64-kernel-none/debug/kernel"
KERNEL_PATH_RELEASE = "kernel/target/amd64-kernel-none/release/kernel"

QEMU = "qemu-system-x86_64"
QEMU_ARGS = [
    "-nodefaults",
    "-display", "none",
    "-serial", "stdio",

    "-m", "1G",
    "-cpu", "host",
    "-enable-kvm"
]

import subprocess
import os

def run_debug():
    build_debug()
    print("\n------ Running QEMU -----------\n")
    args = QEMU_ARGS + ["-kernel", BOOTLOADER_PATH_DEBUG, "-initrd", KERNEL_PATH_DEBUG]
    os.execvp(QEMU, args)

def build_debug():
    print("\n----- Building goodloader -----\n")
    bootbuild = subprocess.run(["cargo", "build"], check=True, cwd="./goodloader")
    print("\n----- Building kernel ---------\n")
    kernelbuild = subprocess.run(["cargo", "build"], check=True, cwd="./kernel")
    
run_debug()
