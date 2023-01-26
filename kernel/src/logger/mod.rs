use core::fmt::Write;
use log::{Level, LevelFilter, Metadata, Record};
use spin::Mutex;

use crate::idt::without_interrupts;

mod display;
mod serial;

static KERNEL_LOGGER: LockedKernelLogger = LockedKernelLogger::new();

pub fn init() {
    log::set_logger(&KERNEL_LOGGER).expect("set logger failed");
    log::set_max_level(LevelFilter::Trace);
}

struct LockedKernelLogger(Mutex<KernelLogger>);

impl LockedKernelLogger {
    const fn new() -> Self {
        Self(Mutex::new(KernelLogger::new()))
    }
}

impl log::Log for LockedKernelLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        without_interrupts(|| {
            writeln!(
                self.0.lock().with_level(record.level()),
                "{}",
                record.args()
            )
            .unwrap();
        });
    }

    fn flush(&self) {}
}

trait Sink: Write {
    fn with_level(&mut self, level: Level) -> &mut Self;
}

struct KernelLogger {
    serial: serial::Logger,
    display: display::Logger,
}

impl KernelLogger {
    const fn new() -> Self {
        Self {
            serial: serial::Logger::new(),
            display: display::Logger::new(),
        }
    }
}

impl Sink for KernelLogger {
    fn with_level(&mut self, level: Level) -> &mut Self {
        self.serial.with_level(level);
        self.display.with_level(level);

        self
    }
}

impl Write for KernelLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.serial.write_str(s)?;
        self.display.write_str(s)
    }
}
