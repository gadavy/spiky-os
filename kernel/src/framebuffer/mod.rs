use bootloader_api::info::FrameBufferInfo;
use core::fmt;
use spin::Mutex;

use crate::interrupts;
use writer::Writer;

pub mod color;
mod fonts;
mod writer;

pub static WRITER: Mutex<Option<Writer>> = Mutex::new(None);

pub fn init(info: FrameBufferInfo, buf: &'static mut [u8]) {
    let mut writer = Writer::new(info, buf);
    writer.clear();

    WRITER.lock().replace(writer);
}

pub fn clear() {
    WRITER.lock().as_mut().map(Writer::clear);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().as_mut().map(|w| w.write_fmt(args));
    });
}
