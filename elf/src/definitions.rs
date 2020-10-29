#![allow(unused_parens)]
#![allow(dead_code)]

use bytemuck::{Zeroable, Pod};
use core::num::NonZeroU64;

pub const MAGIC: [u8; 4] = *b"\x7FELF";
pub const EV_CURRENT: u8 = 1;
pub const EHSIZE_X86: usize = 52;
pub const EHSIZE_X64: usize = 64;
pub const PT_LOAD: u32 = 1;
pub const PF_X: u32 = (1 << 0);
pub const PF_W: u32 = (1 << 1);
pub const PF_R: u32 = (1 << 2);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HeaderIdent {
    pub ei_magic: [u8; 4],
    pub ei_class: u8,
    pub ei_data: u8,
    pub ei_version: u8,
    pub ei_osabi: u8,
    pub ei_abiversion: u8,
    pub ei_pad: [u8; 7],
}

/*
impl HeaderIdent {
    pub fn os_abi(&self) -> Option<OsAbi> {
        let osabi = match self.ei_osabi {
            0   => OsAbi::None,
            1   => OsAbi::Hpux,
            2   => OsAbi::NetBSD,
            3   => OsAbi::GnuLinux,
            6   => OsAbi::Solaris,
            7   => OsAbi::Aix,
            8   => OsAbi::Irix,
            9   => OsAbi::FreeBSD,
            10  => OsAbi::Tru64,
            11  => OsAbi::Modesto,
            12  => OsAbi::OpenBSD,
            64  => OsAbi::ArmAEABI,
            97  => OsAbi::Arm,
            255 => OsAbi::Standalone,
            _ => return None,
        };

        return Some(osabi);
   }
}
*/

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub e_ident: HeaderIdent,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: Option<NonZeroU64>,
    pub e_phoff: Option<NonZeroU64>,
    pub e_shoff: Option<NonZeroU64>,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

/*
impl Header {
    pub fn machine(&self) -> Option<Machine> {
        let machine = match self.e_machine {
            0   => Machine::None,
            20  => Machine::PowerPC,
            21  => Machine::Power64,
            40  => Machine::Arm,
            3   => Machine::X86,
            62  => Machine::X64,
            224 => Machine::AmdGpu,
            243 => Machine::RiscV,
            _ => return None,
        };

        return Some(machine);
    }
}
*/

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,

    /* Practical Binary Analysis says it should be zero, but readelf on some
     * binaries shows that it is equal to p_vaddr */
    pub p_paddr: u64,

    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Class {
    Bits32 = 1,
    Bits64 = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Data {
    Lsb = 1,
    Msb = 2,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Type {
    None = 0,
    Relocatable  = 1,
    Executable = 2,
    SharedObject = 3,
    Core = 4,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Machine {
    None    = 0,
    PowerPC = 20,
    Power64 = 21,
    Arm     = 40,
    X86     = 3,
    X64     = 62,
    AmdGpu  = 224,
    RiscV   = 243,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OsAbi {
    None		= 0,    /* UNIX System V ABI */
    Hpux		= 1,    /* HP-UX */
    NetBSD		= 2,    /* NetBSD.  */
    GnuLinux 	= 3,    /* Object uses GNU ELF extensions.  */
    Solaris     = 6,    /* Sun Solaris.  */
    Aix	        = 7,    /* IBM AIX.  */
    Irix		= 8,    /* SGI Irix.  */
    FreeBSD     = 9,    /* FreeBSD.  */
    Tru64		= 10,	/* Compaq TRU64 UNIX.  */
    Modesto     = 11,	/* Novell Modesto.  */
    OpenBSD     = 12,	/* OpenBSD.  */
    ArmAEABI 	= 64,	/* ARM EABI */
    Arm         = 97,	/* ARM */
    Standalone	= 255,	/* Standalone (embedded) application */
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum SegmentType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interpreter = 3,
    Note = 4,
    SharedLib = 5,
    ProgramHeader = 6,
    ThreadLocalStorage = 7,
}

unsafe impl Zeroable for HeaderIdent {}
unsafe impl Pod for HeaderIdent {}

unsafe impl Zeroable for ProgramHeader {}
unsafe impl Pod for ProgramHeader {}

unsafe impl Zeroable for Header {}
unsafe impl Pod for Header {}

