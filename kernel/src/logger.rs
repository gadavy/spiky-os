use log::{LevelFilter, Metadata, Record};

static KERNEL_LOGGER: KernelLogger = KernelLogger;

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
        crate::framebuffer_println!("[{:5}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
