#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]

mod mem;
mod vga;
use x86_64;

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}

fn recurse(x: u8) {
    let a = &[x];
    vga::VGA_TEXT.lock().print(a);
    recurse(x + 1);
}

#[inline(never)]
fn crash() {
    println!("MMMMMMMMMMMMMMMM");
}

#[no_mangle]
fn rust_start() {
    let f: fn() = crash;
    let f: usize = unsafe { core::mem::transmute(f) };

    /* this magically doesnt crash */
    print!("{:?}", f);

    for _ in 0..(1 << 30) {
        unsafe {
            asm!("nop");
        }
    }

    /* this does */
    crash();
    println!("AAAAAAAAAAAAA");

    unreachable!();
}
