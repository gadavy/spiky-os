use crate::framebuffer::color::Color;
use log::{LevelFilter, Metadata, Record};

static KERNEL_LOGGER: KernelLogger = KernelLogger;

const SUCCESS: Color = Color::new(0x0046_c93a);
const WARNING: Color = Color::new(0x00ff_8200);
const DANGER: Color = Color::new(0x00ff_4757);

pub fn init() {
    log::set_logger(&KERNEL_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
}

pub struct KernelLogger;

impl log::Log for KernelLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        crate::println!("[{:5}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
