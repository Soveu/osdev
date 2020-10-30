#![no_std]
#![no_main]

#![feature(asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(const_fn)]

pub mod x86;
pub mod serial_io;
pub mod multiboot;

use elf;

const MAGIC: i32 = 0x1BADB002;
const ALIGN_MODULES: i32 = 1 << 0;
const MEMINFO: i32 = 1 << 1;
const FLAGS: i32 = ALIGN_MODULES | MEMINFO;

#[used]
#[link_section = ".multiboot_header"]
static HEADER: [i32; 3] = [
    MAGIC,
    FLAGS,
    0 - MAGIC - FLAGS,
];

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    x86::disable_interrupts();

    print!("\nPANIK\n");
    print!("{:?}\n", info);

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
    /* SAFETY: We need to setup stack here and 'call' start function with multiboot
     * magic and info struct pointer that bootloader gave us */
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

extern "fastcall" fn start(magic: i32, multiboot: *const multiboot::Info) -> ! {
    x86::disable_interrupts();
    print!("Hello!\n");

    assert_eq!(magic, 0x2BADB002);
    print!("Loaded by multiboot\n");

    /* SAFETY: We have checked that we were booted by multiboot */
    let multiboot = unsafe { &*multiboot };

    print!("\ncmdline: {:?}\n", multiboot.cmdline());
    print!("\nModules:\n");

    for module in multiboot.modules() {
        print!("size: {} {:?}\n", module.bytes().len(), module.name());
    }

    let kernel = multiboot.modules()
        .first()
        .expect("no module was loaded with -initrd");

    let kernel_elf = elf::Amd64::from_bytes(kernel.bytes()).unwrap();

    print!("\nProgram headers:\n");
    for header in kernel_elf.program_headers().unwrap() {
        print!("{:?}\n", header);
    }

    print!("\nDone\n");

    loop {
        x86::halt();
    }
}

