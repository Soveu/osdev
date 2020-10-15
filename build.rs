use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=asm/boot.asm");
    println!("cargo:rerun-if-changed=asm/linker.asm");

    println!("cargo:warning=Running yasm");
    let status = Command::new("yasm")
        .args(&["-f", "elf64", "asm/boot.asm"])
        .status()
        .unwrap();

    let code = status.code().unwrap();
    println!("cargo:warning=yasm ended with code {}", code);
    assert_eq!(code, 0);
}

