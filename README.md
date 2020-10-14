# Soveu's kernel

no-name, for now just a hobby kernel, built for working with QEMU

Dependencies for build:
 - rustup with nightly Rust
 - `rust-src` component from rustup
 - yasm
 - ld (rust-lld doesn't want to link elf64 into elf32 :sadface:)

Currently it uses window QEMU, so graphical environment is also required

Building and running is as easy as `cargo build/run`

