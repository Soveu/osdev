#![allow(dead_code)]
/* Code from flower-os */
//! Thanks to https://en.wikibooks.org/wiki/Serial_Programming/8250_UART_Programming and OSDev wiki

use core::marker::PhantomData;

/// Read a single byte from the port.
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nostack, nomem));
    result
}

/// Write a single byte to the port.
#[inline(always)]
pub unsafe fn outb(value: u8, port: u16) {
    asm!("out dx, al", in("al") value, in("dx") port, options(nostack, nomem))
}

/// Nice little type that allows us to specify the size of the value read without using inb
/// directly.
pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        inb(port)
    }
    unsafe fn port_out(port: u16, value: u8) {
        outb(value, port);
    }
}

/// An `InOut`sized port. This could be any of the type implementors for `InOut`.
#[derive(Debug)]
pub struct Port<T: InOut> {
    /// Port address.
    port: u16,

    /// Zero-byte placeholder.  This is only here so that we can have a
    /// type parameter `T` without a compiler error.
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Create a port which can handle values of `T` size.
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }

    /// Read a value from `self.port`.
    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    /// Write a value to `self.port`.
    pub fn write(&mut self, value: T) {
        unsafe {
            T::port_out(self.port, value);
        }
    }
}

use bitflags::bitflags;
use core::fmt::{self, Write};

pub const PORT_1_ADDR: u16 = 0x3f8;
pub const PORT_2_ADDR: u16 = 0x2f8;
pub const MAX_BAUD: u32 = 115200;

pub struct SerialPort {
    initialized: bool,
    data: Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
    line_status: Port<u8>,
    _modem_status: Port<u8>,
    _scratch: Port<u8>,
}

impl SerialPort {
    pub const unsafe fn new(port_base: u16) -> SerialPort {
        SerialPort {
            initialized: false,
            data: Port::new(port_base),
            interrupt_enable: Port::new(port_base + 1),
            fifo_control: Port::new(port_base + 2),
            line_control: Port::new(port_base + 3),
            modem_control: Port::new(port_base + 4),
            line_status: Port::new(port_base + 5),
            _modem_status: Port::new(port_base + 6),
            _scratch: Port::new(port_base + 7),
        }
    }

    /// Initializes the serial port
    pub fn init(&mut self, baud: u32, enable_irqs: bool) -> Result<(), InvalidBaudrate> {
        let divisor = MAX_BAUD / baud;
        if MAX_BAUD / divisor != baud {
            return Err(InvalidBaudrate(baud));
        }

        // Disable interrupts
        self.interrupt_enable.write(0);

        // Enable DLAB - data port & interrupt enable will temporarily become DLAB lsb & msb
        self.line_control.write(1 << 7);

        // Write divisor
        self.data.write((divisor & 0xFF) as u8);
        self.interrupt_enable.write((divisor >> 8) as u8);

        // 8 bits, no parity byte, one stop bit
        self.line_control.write(0b111);

        //             Flags: Enable     & Reset tx/rx & 64byte buf & trigger level 56bytes
        let fifo_flags = (0b1 << 0) | (0b11 << 1) | (0b1 << 5) | (0b11 << 6);
        self.fifo_control.write(fifo_flags);

        //  Request To Send & Data Terminal Ready
        let mut modem_ctrl_flags = 0b1 << 0;

        if enable_irqs {
            modem_ctrl_flags |= 0b1 << 3; // Aux output 2 (enable IRQs, practically)
        }

        self.modem_control.write(modem_ctrl_flags);

        self.initialized = true;
        Ok(())
    }

    /// Returns the line status. Panics if not initialized.
    pub fn status(&mut self) -> LineStatus {
        if self.initialized {
            LineStatus::from_bits_truncate(self.line_status.read())
        } else {
            panic!("Serial port not initialized");
        }
    }

    /// Attempts to read one byte of data, returning none if no byte was waiting.
    pub fn try_read(&mut self) -> Option<u8> {
        if self.status().contains(LineStatus::DATA_READY) {
            Some(self.data.read())
        } else {
            None
        }
    }

    /// Attempts to write one byte of data, returning whether it could.
    pub fn try_write(&mut self, data: u8) -> bool {
        if self
            .status()
            .contains(LineStatus::TRANSMITTER_HOLDING_REGISTER_EMPTY)
        {
            self.data.write(data);
            true
        } else {
            false
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            while !self.try_write(byte) {}
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidBaudrate(u32);

bitflags! {
    pub struct LineStatus: u8 {
        const DATA_READY = 1 << 0;
        const OVERRUN_ERROR = 1 << 1;
        const PARITY_ERROR = 1 << 2;
        const FRAMING_ERROR = 1 << 3;
        const BREAK_INDICATOR = 1 << 4;
        const TRANSMITTER_HOLDING_REGISTER_EMPTY = 1 << 5;
        const TRANSMITTER_EMPTY = 1 << 6;
        const IMPENDING_ERROR = 1 << 7;
    }
}
