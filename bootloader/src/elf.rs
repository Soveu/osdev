use core::num::NonZeroU64;

#[repr(C)]
struct HeaderIdent {
    ei_magic: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

#[repr(C)]
struct Header {
    e_ident: HeaderIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: Option<NonZeroU64>,
    e_shoff: Option<NonZeroU64>,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

#[repr(C)]
struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,

    /* Practical Binary Analysis says it should be zero, but readelf on some
     * binaries shows that it is equal to p_vaddr, so just ignore this field */
    p_paddr: u64,

    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

const MAGIC: [u8; 4] = b"\x7FELF";

const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;

const ELFDATALSB: u8 = 1;
const ELFDATAMSB: u8 = 2;

const EV_CURRENT: u8 = 1;

const ET_NONE: u16 = 0;
const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const ET_CORE: u16 = 4;

const EM_386: u16 = 3;
const EM_X86_64: u16 = 62;

const EHSIZE_X86: u16 = 52;
const EHSIZE_X64: u16 = 64;

const PT_LOAD: u32 = 1;

const PF_X: u32 = (1 << 0);
const PF_W: u32 = (1 << 1);
const PF_R: u32 = (1 << 2);

