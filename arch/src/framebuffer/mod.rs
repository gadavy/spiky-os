pub use color::*;

const CHARACTER_WIDTH: usize = 9;
const CHARACTER_HEIGHT: usize = 16;

/// `RobotoMono` bitmap.
static FONT: [[[u8; CHARACTER_WIDTH]; CHARACTER_HEIGHT]; 256] =
    include!("../../fonts/roboto-mono-bitmap.txt");

mod color;

#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

pub struct Framebuffer {
    bpp: usize,
    width: usize,
    height: usize,
    stride: usize,
    px_format: PixelFormat,

    current_x: usize,
    current_y: usize,
    buf: &'static mut [u8],
}

impl Framebuffer {
    pub fn new(
        bpp: usize,
        width: usize,
        height: usize,
        stride: usize,
        px_format: PixelFormat,
        buf: &'static mut [u8],
    ) -> Self {
        buf.fill(0);

        Self {
            bpp,
            width,
            height,
            stride,
            px_format,
            buf,
            current_x: 0,
            current_y: 0,
        }
    }

    fn draw_char(&mut self, character: char) {
        if character == '\n' {
            self.newline();
            return;
        }

        if self.current_x + CHARACTER_WIDTH >= self.width {
            self.newline();
        }

        if self.current_y + CHARACTER_HEIGHT >= self.height {
            self.clear();
            self.current_x = 0;
            self.current_y = 0;
        }

        let mut offset = (self.current_y * self.stride + self.current_x) * self.bpp;

        for column in FONT[character as usize] {
            for (x, intensity) in column.iter().enumerate() {
                let color = WHITE.intensity(*intensity).to_bytes(self.px_format);

                let px_start = offset + x * self.bpp;
                let px_end = px_start + self.bpp;

                self.buf[px_start..px_end].copy_from_slice(&color[..self.bpp]);
            }

            offset += self.stride * self.bpp;
        }

        self.current_x += CHARACTER_WIDTH;
    }

    fn clear(&mut self) {
        self.buf.fill(0);
    }

    fn newline(&mut self) {
        self.current_x = 0;
        self.current_y += CHARACTER_HEIGHT;
    }
}

impl core::fmt::Write for Framebuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.draw_char(c);
        }

        Ok(())
    }
}
