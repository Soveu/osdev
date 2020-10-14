use core::ptr;
use spin;

#[repr(C)]
#[derive(Clone, Copy)]
struct VGACell {
    character:  u8,
    color:      u8,
}

#[allow(non_snake_case)]
const fn VGAPTR() -> *mut [[VGACell; 80]; 25] {
    0xB8000 as *mut _
}

static COLUMN: spin::Mutex<usize> = spin::Mutex::new(0);
const BLACK: VGACell = VGACell {
    character: 0,
    color: 0x0,
};

type VGA = [[VGACell; 80]; 25];

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Message = 0xE,
    Error = 0xC,
}

pub fn print(s: &[u8], color: Color) {
    let mut col = COLUMN.lock();
    let vga = unsafe { &mut *VGAPTR() };

    for x in s.iter().copied() {
        vga_print_byte(vga, &mut *col, x, color as u8);
    }
}

fn vga_print_byte(vga: &mut VGA, col: &mut usize, character: u8, color: u8) {
    if character == b'\n' || *col == 80 {
        vga_newline(vga);
        *col = 0;
    }
    if character == b'\n' {
        return;
    }

    /*
    if !(0x20..0x7F).contains(&character) && character != b'\n' {
        return vga_print_byte(vga, col, 0x41, color);
    }
    */

    let ptr = &mut vga[24][*col];
    let c = VGACell {
        character,
        color,
    };
    unsafe { ptr::write_volatile(ptr, c); }

    *col += 1;
}

fn vga_newline(vga: &mut VGA) {
    for i in 1..vga.len() {
        vga[i-1] = vga[i];
    }

    vga[24] = [BLACK; 80];
}

