use log::LevelFilter::Trace;
use log::{Log, Metadata, Record};

static mut KERNEL_LOGGER: KernelLogger = KernelLogger::empty();

pub fn init(write_fn: fn(record: &Record)) {
    unsafe {
        KERNEL_LOGGER.write_fn = write_fn;
        log::set_max_level(Trace);
        log::set_logger(&KERNEL_LOGGER).expect("logger setup failed");
    }
}

struct KernelLogger {
    write_fn: fn(record: &Record),
}

impl KernelLogger {
    const fn empty() -> Self {
        Self { write_fn: |_| {} }
    }
}

impl Log for KernelLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        (self.write_fn)(record);
    }

    fn flush(&self) {}
}
