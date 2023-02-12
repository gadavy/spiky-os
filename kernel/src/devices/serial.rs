use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

pub static COM1: Mutex<SerialPort> = Mutex::new(SerialPort::empty(0x3F8));
pub static COM2: Mutex<SerialPort> = Mutex::new(SerialPort::empty(0x2F8));

pub struct SerialPort {
    data: Port<u8>,
    int_en: PortWriteOnly<u8>,
    fifo_ctrl: PortWriteOnly<u8>,
    line_ctrl: PortWriteOnly<u8>,
    modem_ctrl: PortWriteOnly<u8>,
    line_sts: PortReadOnly<u8>,
}

impl SerialPort {
    const fn empty(base: u16) -> Self {
        Self {
            data: Port::new(base),
            int_en: PortWriteOnly::new(base + 1),
            fifo_ctrl: PortWriteOnly::new(base + 2),
            line_ctrl: PortWriteOnly::new(base + 3),
            modem_ctrl: PortWriteOnly::new(base + 4),
            line_sts: PortReadOnly::new(base + 5),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.int_en.write(0x00); //     Disable interrupts
            self.line_ctrl.write(0x80); //  Enable DLAB (set baud rate divisor)
            self.data.write(0x03); //       Set divisor to 3 (lo byte) 38400 baud
            self.int_en.write(0x00); //     Set divisor to 3 (hi byte)
            self.line_ctrl.write(0x03); //  8 bits, no parity, one stop bit
            self.fifo_ctrl.write(0xC7); //  Enable FIFO, clear them, with 14-byte threshold
            self.modem_ctrl.write(0x0B); // IRQs enabled, RTS/DSR set
            self.int_en.write(0x01); //     Enable interrupts
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        buf.iter().for_each(|b| self.write_byte(*b));
    }

    fn write_byte(&mut self, byte: u8) {
        unsafe {
            while !self.transit_empty() {}
            self.data.write(byte);
        }
    }

    #[inline]
    unsafe fn transit_empty(&mut self) -> bool {
        self.line_sts.read() & 0x20 != 0
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());

        Ok(())
    }
}
