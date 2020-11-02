#!/usr/bin/python

BOOTLOADER_PATH = "goodloader/target/i686-goodloader-none/%s/goodloader"
KERNEL_PATH = "kernel/target/amd64-kernel-none/%s/kernel"

QEMU = "qemu-system-x86_64"
QEMU_ARGS = [
    "-nodefaults",
    "-display", "none",
    "-serial", "stdio",

    "-m", "512M",
#    "-S", "-s",
    "-cpu", "host",
    "-enable-kvm"
]

import subprocess
import os

def bootloader_path(debug):
    return BOOTLOADER_PATH % (["release", "debug"][debug])
def kernel_path(debug):
    return KERNEL_PATH % (["release", "debug"][debug])

def run(debug=True):
    build(debug)

    print("\n------ Running QEMU -----------\n")
    args = QEMU_ARGS + ["-kernel", bootloader_path(debug), "-initrd", kernel_path(debug)]
    os.execvp(QEMU, args)

def build(debug=True):
    args = ["cargo", "build"] + ["--release"] * (not debug)
    print("\n----- Building goodloader -----\n")
    bootbuild = subprocess.run(args, check=True, cwd="./goodloader")
    print("\n----- Building kernel ---------\n")
    kernelbuild = subprocess.run(args, check=True, cwd="./kernel")
    
run(debug=True)
