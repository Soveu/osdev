#![no_std]
#![no_main]

#![feature(lang_items)]
#![feature(asm)]

mod mem;
mod vga;
use x86_64;

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn recurse(x: u8) {
    let a = &[x];
    vga::VGA_TEXT.lock().print(a);
    recurse(x+1);
}

#[no_mangle]
fn rust_start() {
    vga::VGA_TEXT.lock().print(b"Henlo!");

    print!("asdf");

    vga::VGA_TEXT.lock().print(b"NEWLINE\n");

    println!("asdf");
}

