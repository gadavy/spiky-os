use crate::drivers::framebuffer::{Color, Point, WHITE};
use crate::drivers::uart;
use crate::interrupts::without_interrupts;
use bootloader_api::info::FrameBufferInfo;
use log::{Level, LevelFilter, Metadata, Record};
use spin::Mutex;

mod font;

static KERNEL_LOGGER: KernelLogger = KernelLogger::new();

pub fn init(info: FrameBufferInfo) {
    KERNEL_LOGGER.init(info);

    log::set_logger(&KERNEL_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
}

struct KernelLogger {
    inner: Mutex<Option<InnerKernelLogger>>,
}

impl KernelLogger {
    const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    fn init(&self, info: FrameBufferInfo) {
        self.inner
            .lock()
            .replace(InnerKernelLogger::new(info.width, info.height));
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        use core::fmt::Write;

        without_interrupts(|| {
            if let Some(w) = self.inner.lock().as_mut() {
                writeln!(w.with_color(record.level().into()), "{}", record.args()).unwrap();
            }

            writeln!(uart::UART.lock(), "[{}] {}", record.level(), record.args()).unwrap();
        });
    }

    fn flush(&self) {}
}

struct InnerKernelLogger {
    current_x: usize,
    current_y: usize,
    max_x: usize,
    max_y: usize,
    color: Color,
}

impl InnerKernelLogger {
    fn new(width: usize, height: usize) -> Self {
        Self {
            current_x: 0,
            current_y: 0,
            max_x: width,
            max_y: height,
            color: WHITE,
        }
    }

    fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;

        self
    }

    fn print_string(&mut self, s: &str) {
        let mut fb = crate::drivers::framebuffer::FRAMEBUFFER.lock();

        for character in s.chars() {
            if character == '\n' {
                self.newline();

                continue;
            }

            if self.current_x >= self.max_x {
                self.newline();
            }

            if self.current_y >= self.max_y {
                fb.fill(self.color);

                self.current_x = 0;
                self.current_y = 0;
            }

            for (y, row) in font::FONT[character as usize].iter().enumerate() {
                for (x, intensity) in row.iter().enumerate() {
                    fb.fill_pixel(
                        Point::new(self.current_x + x, self.current_y + y),
                        self.color.intensity(*intensity),
                    );
                }
            }

            self.current_x += font::CHARACTER_WIDTH;
        }
    }

    fn newline(&mut self) {
        self.current_x = 0;
        self.current_y += font::CHARACTER_HEIGHT;
    }
}

impl core::fmt::Write for InnerKernelLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_string(s);

        Ok(())
    }
}

const DANGER: Color = Color::new(0x00ff_4757);
const WARNING: Color = Color::new(0x00ff_8200);
const SUCCESS: Color = Color::new(0x0046_c93a);

impl From<Level> for Color {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => DANGER,
            Level::Warn => WARNING,
            Level::Info => SUCCESS,
            _ => WHITE,
        }
    }
}
