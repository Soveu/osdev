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

#[no_mangle]
fn rust_start() {
    vga::print(b"HELLO FROM RUST!", vga::Color::Message);
    vga::print(b"WTF", vga::Color::Message);
}

