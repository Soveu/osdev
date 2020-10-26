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
