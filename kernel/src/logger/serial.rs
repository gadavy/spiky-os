use core::fmt::Write;
use log::Level;

use crate::devices::uart;

pub struct Logger;

impl Logger {
    pub const fn new() -> Self {
        Self
    }
}

impl super::Sink for Logger {
    fn with_level(&mut self, level: Level) -> &mut Self {
        let color = match level {
            Level::Error => b"\x1b[0031m",
            Level::Warn => b"\x1b[0033m",
            Level::Info => b"\x1b[0032m",
            Level::Debug => b"\x1b[0034m",
            Level::Trace => b"\x1b[0035m",
        };

        let mut w = uart::UART.lock();
        w.write(color);
        w.write(b"[");
        w.write(level.as_str().as_bytes());
        w.write(b"]\x1b[0m\t");

        self
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut w = uart::UART.lock();
        w.write(s.as_bytes());

        Ok(())
    }
}
