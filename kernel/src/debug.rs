use core::fmt::Write;

pub fn write_log(record: &log::Record) {
    write_serial(record);
    write_display(record);
}

fn write_serial(record: &log::Record) {
    let color = match record.level() {
        log::Level::Error => "\x1b[0031m",
        log::Level::Warn => "\x1b[0033m",
        log::Level::Info => "\x1b[0032m",
        log::Level::Debug => "\x1b[0034m",
        log::Level::Trace => "\x1b[0035m",
    };

    let mut writer = super::devices::serial::COM1.lock();

    let _ = writeln!(
        writer,
        "{}[{}]\x1b[0m\t {}",
        color,
        record.level(),
        record.args()
    );
}

fn write_display(record: &log::Record) {
    let mut display = super::devices::display::DISPLAY.lock();

    let _ = writeln!(
        display,
        "[{} {}] {}",
        record.target(),
        record.level(),
        record.args()
    );
}
