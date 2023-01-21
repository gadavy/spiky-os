use spin::Mutex;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

pub const COM1_BASE: u16 = 0x3F8;

pub static UART: Mutex<UartDriver> = Mutex::new(UartDriver::empty());

pub struct UartDriver {
    inner: Mutex<Option<InnerController>>,
}

impl UartDriver {
    const fn empty() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub fn init(&self, base: u16) {
        if let Some(inner) = InnerController::new(base) {
            self.inner.lock().replace(inner);
        } else {
            log::warn!("UART with base `0x{base:02x}` not supported");
        }
    }

    pub fn write(&self, buf: &[u8]) {
        if let Some(inner) = self.inner.lock().as_mut() {
            buf.iter().for_each(|b| inner.write_byte(*b));
        }
    }
}

impl core::fmt::Write for UartDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());

        Ok(())
    }
}

struct InnerController {
    data: Port<u8>,
    line_sts: PortReadOnly<u8>,
}

impl InnerController {
    fn new(base: u16) -> Option<Self> {
        let mut data = Port::<u8>::new(base);
        let mut int_en = PortWriteOnly::<u8>::new(base + 1);
        let mut fifo_ctrl = PortWriteOnly::<u8>::new(base + 2);
        let mut line_ctrl = PortWriteOnly::<u8>::new(base + 3);
        let mut modem_ctrl = PortWriteOnly::<u8>::new(base + 4);
        let line_sts = PortReadOnly::<u8>::new(base + 5);

        unsafe {
            int_en.write(0x00); //     Disable interrupts
            line_ctrl.write(0x80); //  Enable DLAB (set baud rate divisor)
            data.write(0x03); //       Set divisor to 3 (lo byte) 38400 baud
            int_en.write(0x00); //     Set divisor to 3 (hi byte)
            line_ctrl.write(0x03); //  8 bits, no parity, one stop bit
            fifo_ctrl.write(0xC7); //  Enable FIFO, clear them, with 14-byte threshold
            modem_ctrl.write(0x0B); // IRQs enabled, RTS/DSR set
            modem_ctrl.write(0x1E); // Set loopback mode
            data.write(0xAE); //       Send byte 0xAE and check if serial returns same byte

            // Check if serial is faulty (i.e: not same byte as sent)
            if data.read() != 0xAE {
                return None;
            }

            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            modem_ctrl.write(0x0F);
        }

        Some(Self { data, line_sts })
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
