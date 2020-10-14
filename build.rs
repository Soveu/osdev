use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=asm/boot.asm");
    println!("cargo:rerun-if-changed=asm/linker.ld");

    Command::new("yasm")
        .args(&["-f", "elf64", "asm/boot.asm"])
        .status()
        .unwrap();
}

