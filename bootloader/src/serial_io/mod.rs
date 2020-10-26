mod serial;
use serial::SerialPort;

pub struct SerialIO {
    port: SerialPort,
}

impl SerialIO {
    pub fn new() -> Self {
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init(115200, false);
        Self { port }
    }
}

use core::fmt;
impl fmt::Write for SerialIO {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.port.write_str(s)
    }
}
