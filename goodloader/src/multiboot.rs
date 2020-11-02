use core::mem::MaybeUninit;
use core::slice;
use core::str;

/* SAFETY: Pointer needs to point to a valid C string */
unsafe fn strlen(ptr: *const u8) -> usize {
    let mut len = 0usize;
    while *ptr.offset(len as isize) != 0u8 {
        len += 1;
    }
    return len;
}

/* SAFETY: from here now we rely purely on multiboot spec, so you need to be sure
 * that you are being booted by multiboot */

#[repr(C)]
pub struct Module {
    start: *const u8,
    end: *const u8,
    string: *const u8,
    reserved: u32,
}

impl Module {
    pub fn bytes(&self) -> &[u8] {
        let len = unsafe { self.end.offset_from(self.start) };

        if len < 0 {
            return &[];
        }

        return unsafe { slice::from_raw_parts(self.start, len as usize) };
    }

    pub fn name(&self) -> &str {
        if self.string.is_null() {
            return "";
        }

        return unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(self.string, strlen(self.string)))
        };
    }
}

#[repr(C)]
pub struct Info {
    flags: u32,

    mem: MaybeUninit<(u32, u32)>,
    boot_device: MaybeUninit<[u8; 4]>,
    cmdline: MaybeUninit<*const u8>,
    mods: MaybeUninit<(usize, *const Module)>,

    _syms_stuff: MaybeUninit<[u32; 4]>,

    mmap: MaybeUninit<(u32, u32)>,

    _rest: MaybeUninit<[u8; 63]>,
}

impl Info {
    pub fn memory_limits(&self) -> Option<(u32, u32)> {
        if self.flags & (1 << 0) == 0 {
            return None;
        }
        let limits = unsafe { self.mem.assume_init() };
        return Some(limits);
    }

    pub fn cmdline(&self) -> &str {
        if self.flags & (1 << 2) == 0 {
            return "";
        }

        let ptr = unsafe { self.cmdline.assume_init() };
        if ptr.is_null() {
            return "";
        }

        return unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, strlen(ptr))) };
    }

    pub fn modules(&self) -> &[Module] {
        if self.flags & (1 << 3) == 0 {
            return &[];
        }

        let (len, ptr) = unsafe { self.mods.assume_init() };
        if ptr.is_null() {
            return &[];
        }

        return unsafe { slice::from_raw_parts(ptr, len) };
    }

    pub fn memory_map(&self) -> Option<MemoryMapIter> {
        if self.flags & (1 << 6) == 0 {
            return None;
        }

        let (length, addr) = unsafe { self.mmap.assume_init() };
        let iter = MemoryMapIter {
            current: addr,
            end: addr + length,
            _lifetime: core::marker::PhantomData,
        };
        return Some(iter);
    }
}

/* Code comes from multiboot crate */
pub struct MemoryMapIter<'multiboot> {
    current: u32,
    end: u32,
    _lifetime: core::marker::PhantomData<&'multiboot Info>,
}

impl<'multiboot> Iterator for MemoryMapIter<'multiboot> {
    type Item = &'multiboot MemoryEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let ptr = self.current as *const MemoryEntry;
        let entry = unsafe { &*ptr };
        self.current += entry.size + 4;
        return Some(entry);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryEntry {
    size: u32, // note: size is defined at offset -4
    base_addr: [u32; 2],
    length: [u32; 2],
    mtype: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum MemoryEntryType {
    AvaliableRam = 1,
    AcpiInfo = 3,
    HibernationReserved = 4,
    DefectiveRamModule = 5,
    Reserved,
}

impl MemoryEntryType {
    pub fn from_integer(x: u32) -> Self {
        match x {
            1 => Self::AvaliableRam,
            2 => Self::Reserved,
            3 => Self::AcpiInfo,
            4 => Self::HibernationReserved,
            5 => Self::DefectiveRamModule,
            _ => Self::Reserved,
        }
    }
}

impl core::fmt::Debug for MemoryEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let base_addr = self.base_addr[0] as u64 | ((self.base_addr[1] as u64) << 32);
        let length = self.length[0] as u64 | ((self.length[1] as u64) << 32);

        f.write_fmt(format_args!(
            "MemoryEntry {{ base_addr: 0x{:X}, length: 0x{:X}, type: {} = {:?} }}",
            base_addr,
            length,
            self.mtype,
            MemoryEntryType::from_integer(self.mtype),
        ))
    }
}
