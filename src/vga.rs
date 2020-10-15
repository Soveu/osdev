use core::ptr;
use spin;

#[repr(C)]
#[derive(Clone, Copy)]
struct VGACell {
    character:  u8,
    color:      u8,
}

const BLACK: VGACell = VGACell {
    character: 0,
    color: 0x0,
};

pub struct VGA {
    column: usize,
}

impl VGA {
    const fn _vgaptr() -> *mut [[VGACell; 80]; 25] {
        0xB8000 as *mut _
    }
    fn vgaptr(&mut self) -> (&mut usize ,&mut [[VGACell; 80]; 25]) {
        /* SAFETY: we are the only one having this pointer */
        let p = unsafe { &mut *Self::_vgaptr() };
        (&mut self.column, p)
    }

    const fn new() -> Self {
        Self { column: 0 }
    }

    pub fn print(&mut self, s: &[u8]) {
        for x in s.iter().copied() {
            self.vga_print_byte(x, 0xE);
        }
    }
    
    fn vga_print_byte(&mut self, character: u8, color: u8) {
        let (col, vga) = self.vgaptr();

        if character == b'\n' || *col == 80 {
            Self::vga_newline(vga);
            *col = 0;
        }
        if character == b'\n' {
            return;
        }
    
        if !(0x20..0x7F).contains(&character) {
            return self.vga_print_byte(0x41, color);
        }
    
        let ptr = &mut vga[24][*col];
        let c = VGACell {
            character,
            color,
        };
        unsafe { ptr::write_volatile(ptr, c); }
    
        *col += 1;
    }
    
    fn vga_newline(vga: &mut [[VGACell; 80]; 25]) {
        vga.rotate_left(1);
        unsafe {
            ptr::write_volatile(&mut vga[24], [BLACK; 80]);
        }
    }
}

use core::fmt;
impl fmt::Write for VGA {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}

pub static VGA_TEXT: spin::Mutex<VGA> = spin::Mutex::new(VGA::new());

/* From https://github.com/phil-opp/blog_os/blob/post-03/src/vga_buffer.rs */
/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_TEXT.lock().write_fmt(args).unwrap();
}

