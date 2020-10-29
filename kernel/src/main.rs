#![no_std]
#![no_main]

#![feature(asm)]

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt", options(nostack, nomem)); }
    }
}

#[no_mangle]
pub fn _start() {
    loop {
        unsafe { asm!("hlt", options(nostack, nomem)); }
    }
}

