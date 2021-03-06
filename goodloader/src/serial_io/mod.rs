mod serial;
use serial::SerialPort;

use spin::{Lazy, Mutex};

pub struct SerialIO {
    port: SerialPort,
}

impl SerialIO {
    pub fn new() -> Self {
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init(115200, false).unwrap();
        Self { port }
    }
}

use core::fmt;
impl fmt::Write for SerialIO {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.port.write_str(s)
    }
}

pub static WRITER: Lazy<Mutex<SerialIO>> = Lazy::new(|| Mutex::new(SerialIO::new()));

/* Code from phil-opp osdev blog */

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial_io::_print(format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
