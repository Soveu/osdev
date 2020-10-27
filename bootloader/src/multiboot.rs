use core::mem::MaybeUninit;
use core::slice;
use core::str;

unsafe fn strlen(ptr: *const u8) -> usize {
    let mut len = 0usize;
    while *ptr.offset(len as isize) != 0u8 {
        len += 1;
    }
    return len;
}

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

        return unsafe {
            slice::from_raw_parts(self.start, len as usize)
        };
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

    _rest: MaybeUninit<[u8; 88]>,
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

        return unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(ptr, strlen(ptr)))
        };
    }

    pub fn modules(&self) -> &[Module] {
        if self.flags & (1 << 3) == 0 {
            return &[];
        }

        let (len, ptr) = unsafe { self.mods.assume_init() };
        if ptr.is_null() {
            return &[];
        }

        return unsafe {
            slice::from_raw_parts(ptr, len)
        };
    }
}

