#![no_std]
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
    x86_64::instructions::interrupts::int3();

    vga::print(b"HELLO FROM RUST!", vga::Color::Message);
    vga::print(b"WTF", vga::Color::Message);
}

