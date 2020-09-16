#![no_std]
#![feature(lang_items)]

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::ptr;

#[repr(C)]
struct VGACell {
    character:  u8,
    color:      u8,
}

fn vga_print(s: &[u8]) {
    let vgaptr = 0xB8000 as *mut VGACell;

    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            let ptr = vgaptr.add(i);
            let c = VGACell {
                character: byte,
                color: 0x2F,
            };
            ptr::write_volatile(ptr, c);
        }
    }
}

#[no_mangle]
fn rust_start() {
    vga_print(b"HELLO FROM RUST!");
}

