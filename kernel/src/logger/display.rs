use core::fmt::Write;

use log::Level;

use crate::devices::framebuffer::{Color, Point, BLACK, GREEN, ORANGE, RED, WHITE};

const CHARACTER_WIDTH: usize = 9;
const CHARACTER_HEIGHT: usize = 16;

/// `RobotoMono` bitmap.
static FONT: [[[u8; CHARACTER_WIDTH]; CHARACTER_HEIGHT]; 256] =
    include!("../../../assets/roboto-mono-bitmap.txt");

pub struct Logger {
    current_x: usize,
    current_y: usize,
    color: Color,
}

impl Logger {
    pub const fn new() -> Self {
        Self {
            current_x: 0,
            current_y: 0,
            color: WHITE,
        }
    }

    fn draw_text(&mut self, s: &str) {
        let mut fb = crate::devices::framebuffer::FRAMEBUFFER.lock();

        for character in s.chars() {
            if character == '\n' {
                self.newline();
                continue;
            }

            if self.current_x >= fb.width() {
                self.newline();
            }

            if self.current_y >= fb.height() {
                fb.fill(BLACK);
                self.current_x = 0;
                self.current_y = 0;
            }

            for (y, row) in FONT[character as usize].iter().enumerate() {
                for (x, intensity) in row.iter().enumerate() {
                    fb.fill_pixel(
                        Point::new(self.current_x + x, self.current_y + y),
                        self.color.intensity(*intensity),
                    );
                }
            }

            self.current_x += CHARACTER_WIDTH;
        }
    }

    fn newline(&mut self) {
        self.current_x = 0;
        self.current_y += CHARACTER_HEIGHT;
    }
}

impl super::Sink for Logger {
    fn with_level(&mut self, level: Level) -> &mut Self {
        self.color = match level {
            Level::Error => RED,
            Level::Warn => ORANGE,
            Level::Info => GREEN,
            _ => WHITE,
        };

        self
    }
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.draw_text(s);

        Ok(())
    }
}
