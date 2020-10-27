#![no_std]
#![no_main]

#![feature(asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(const_fn)]

pub mod multiboot_header;
pub mod x86;
pub mod serial_io;
pub mod multiboot;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    x86::disable_interrupts();

    print!("\nPANIK\n");
    print!("\n{:?}\n", info);

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
    print!("Hello!\n");
    assert_eq!(magic, 0x2BADB002);
    print!("Loaded by multiboot\n");

    let multiboot = multiboot_info as *const multiboot::Info;
    let multiboot = unsafe { &*multiboot };

    print!("\ncmdline: {:?}\n", multiboot.cmdline());

    print!("\nModules:\n");

    for module in multiboot.modules() {
        print!("{:?} size: {}\n", module.name(), module.bytes().len());
    }

    print!("\nDone\n");

    todo();
}

