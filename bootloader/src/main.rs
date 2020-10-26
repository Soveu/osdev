#![no_std]
#![no_main]

#![feature(asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]

pub mod multiboot_header;
pub mod x86;

mod serial_io;

use serial_io::SerialIO;
use core::fmt::Write;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    todo();
}

fn todo() -> ! {
    x86::disable_interrupts();
    loop {
        x86::halt();
    }
}

#[repr(align(16))]
struct ProgramStack([u8; STACKSIZE]);

const STACKSIZE: usize = 1usize << 19;
static mut STACK: ProgramStack = ProgramStack([0u8; STACKSIZE]);

#[no_mangle]
#[naked]
pub fn _start() -> ! {
    unsafe {
        asm!(
            "mov esp, {}",
            "add esp, {}",
            "mov ebp, esp",
            "mov ecx, eax",
            "mov edx, ebx",
            "jmp {}",
            sym STACK,
            const STACKSIZE,
            sym start,
            options(nostack, nomem, noreturn),
        );
    }
}

extern "fastcall" fn start(magic: i32, multiboot_info: u32) -> ! {
    let mut io = SerialIO::new();
    write!(io, "Hello!\n");

    if magic != 0x2BADB002 {
        write!(io, "Bad magic: got {:x} expected 0x2BADB002\n", magic);
        todo();
    }

    let mb = unsafe {
        multiboot::Multiboot::new(
            multiboot_info as u64,
            |x, sz| { core::ptr::slice_from_raw_parts(x as *const u8, sz).as_ref() }
        )
    };

    let mb = match mb {
        Some(x) => x,
        None => {
            write!(io, "no multiboot\n");
            todo();
        },
    };

    if let Some(lower) = mb.lower_memory_bound() {
        write!(io, "Lower memory: {:x} kilobytes\n", lower);
    }
    if let Some(upper) = mb.upper_memory_bound() {
        write!(io, "Upper memory: {:x} kilobytes\n", upper);
    }

    write!(io, "Done\n");
    todo();
}

