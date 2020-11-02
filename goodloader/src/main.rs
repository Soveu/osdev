#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(const_fn)]

pub mod multiboot;
pub mod serial_io;
pub mod x86;

use elf;

const MAGIC: i32 = 0x1BADB002;
const ALIGN_MODULES: i32 = 1 << 0;
const MEMINFO: i32 = 1 << 1;
const FLAGS: i32 = ALIGN_MODULES | MEMINFO;

#[used]
#[link_section = ".multiboot_header"]
static HEADER: [i32; 3] = [MAGIC, FLAGS, 0 - MAGIC - FLAGS];

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    x86::disable_interrupts();

    print!("\nPANIK\n");
    print!("{:?}\n", info);

    loop {
        x86::halt();
    }
}

#[no_mangle]
#[naked]
pub fn _start() -> ! {
    #[repr(align(16))]
    struct ProgramStack([u8; STACKSIZE]);

    const STACKSIZE: usize = 1usize << 16;
    static mut STACK: ProgramStack = ProgramStack([0u8; STACKSIZE]);

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

    let kernel = multiboot
        .modules()
        .first()
        .expect("no module was loaded with -initrd");

    let kernel_elf = elf::Amd64::from_bytes(kernel.bytes()).unwrap();

    print!("\nProgram headers:\n");
    for header in kernel_elf.program_headers().unwrap() {
        print!("{:?}\n", header);
    }

    print!("\nMultiboot memory maps:\n");
    for map in multiboot.memory_map().unwrap() {
        print!("{:?}\n", map);
    }

    print!("\nDone\n");
    loop {
        x86::halt();
    }
}

#[allow(dead_code)]
extern "fastcall" fn setup_long_mode(_jump: *const u8) -> ! {
    #[repr(align(4096))]
    pub struct Pml4([u64; 512]);

    #[repr(align(4096))]
    pub struct DirectoryPointerTable([u64; 512]);

    static mut PDPT: DirectoryPointerTable = DirectoryPointerTable([0; 512]);
    static mut PML4: Pml4 = Pml4([0; 512]);

    /* Stolen from https://wiki.osdev.org/Entering_Long_Mode_Directly */
    static GDT: [u64; 3] = [
        0x00_0_0_00_00_00000000, /* Null Descriptor - should be present */
        0x00_2_0_9A_00_00000000, /* 64-bit code descriptor (exec/read) */
        0x00_0_0_92_00_00000000, /* 64-bit data descriptor (read/write) */
    ];

    print!("\n");

    /* SAFETY: we are on single thread and nobody has &mut to this object */
    let pml4 = unsafe { &mut PML4 };
    let pdpt = unsafe { &mut PDPT };

    /* Intel Software Developer Manual, Volume 3, Chapter 4 (Paging) */
    pml4.0[0] = (pdpt as *mut _ as u64) | (1 << 0/* present */) | (1 << 1/* writeable */);
    pdpt.0[0] = (0 << 30) | (1 << 0) | (1 << 1) | (1 << 7/* its a page, not directory */);

    #[repr(C, align(4))]
    struct GDTPointer(u16, u16, *const u64);

    let gdt_pointer = GDTPointer(0, 24 - 1, &GDT as *const u64);

    print!("Setting the PAE and PGE bit\n");
    unsafe {
        asm!(
            "mov cr4, eax",
            in("eax") 0b10100000,
            options(nostack, nomem)
        )
    }

    print!("Pointing CR3 at the PML4\n");
    unsafe {
        asm!(
            "mov cr3, eax",
            in("eax") pml4.0.as_ptr(),
            options(nostack, nomem)
        )
    }

    print!("Activating long mode\n");
    unsafe {
        asm!(
            "rdmsr",
            "or eax, 0x00000100",
            "wrmsr",
            "mov ebx, cr0",
            "or ebx, 0x80000001",
            "mov cr0, ebx",
            in("ecx") 0xC0000080u32,
            out("eax") _,
            out("ebx") _,
            options(nostack, nomem)
        )
    }

    print!("Loading GDT\n");
    unsafe {
        asm!(
            "lgdt [eax]",
            in("eax") &gdt_pointer.1,
            options(nostack, nomem)
        )
    }

    print!("Taking a deep breath...");
    for _ in 0..1000000 {
        unsafe { asm!("", options(nostack, nomem)) }
    }
    print!(" Long jump!\n");

    unsafe {
        asm!(
            "ljmp 0x0008, offset {}",
            sym long_mode,
            options(nostack, nomem, noreturn),
        )
    }
}

#[naked]
fn long_mode() -> ! {
    unsafe {
        asm!(
            "mov ds, bx",
            "mov es, bx",
            "mov fs, bx",
            "mov gs, bx",
            "mov ss, bx",
            in("ebx") 0x0010,
            options(nostack, nomem),
        )
    }

    loop {
        x86::halt();
    }
}
