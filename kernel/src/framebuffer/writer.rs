use bootloader_api::info::{FrameBufferInfo, PixelFormat};

use crate::framebuffer::color::{Color, BLACK, WHITE};
use crate::framebuffer::fonts::{CHARACTER_HEIGHT, CHARACTER_WIDTH, FONT_BASIC};

pub struct Writer {
    info: FrameBufferInfo, // TODO: replace to fields???
    buf: &'static mut [u8],

    bg_color: Color,
    fg_color: Color,

    max_x: usize,
    max_y: usize,

    current_x: usize,
    current_y: usize,
}

impl Writer {
    pub fn new(info: FrameBufferInfo, buf: &'static mut [u8]) -> Self {
        let max_x = info.width / CHARACTER_WIDTH;
        let max_y = info.height / CHARACTER_HEIGHT;

        Self {
            info,
            buf,
            bg_color: BLACK,
            fg_color: WHITE,
            current_x: 0,
            current_y: 0,
            max_x,
            max_y,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.info.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.info.height
    }

    #[inline]
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x <= self.width() && y <= self.height()
    }

    pub fn clean(&mut self) {
        self.buf.fill(0);
        self.current_x = 0;
        self.current_y = 0;
    }

    pub fn draw(&mut self, x: usize, y: usize, color: Color) {
        if !self.contains(x, y) {
            return;
        }

        let bpp = self.info.bytes_per_pixel;
        let byte_start = (y * self.info.stride + x) * bpp;
        let byte_end = byte_start + bpp;

        let buf = &mut self.buf[byte_start..byte_end];

        match self.info.pixel_format {
            PixelFormat::Rgb => buf.copy_from_slice(&color.rgb_bytes()[..bpp]),
            PixelFormat::Bgr => buf.copy_from_slice(&color.bgr_bytes()[..bpp]),
            PixelFormat::U8 => buf.fill(color.gray()),
            format => {
                unimplemented!("pixel format {:?} unimplemented", format)
            }
        }
    }

    fn print_char(&mut self, character: char, column: usize, line: usize) {
        let start_x = column * CHARACTER_WIDTH;
        let start_y = line * CHARACTER_HEIGHT;

        // print from the offset within the framebuffer
        let mut offset_x = 0;
        let mut offset_y = 0;

        loop {
            let coord_x = start_x + offset_x;
            let coord_y = start_y + offset_y;

            if self.contains(coord_x, coord_y) {
                let pixel = if offset_x >= 1 {
                    // leave 1 pixel gap between two characters
                    let index = offset_x - 1;
                    let char_font = FONT_BASIC[character as usize][offset_y];

                    if get_bit(char_font, index) != 0 {
                        self.fg_color
                    } else {
                        self.bg_color
                    }
                } else {
                    self.bg_color
                };

                self.draw(coord_x, coord_y, pixel);
            }

            offset_x += 1;
            if offset_x == CHARACTER_WIDTH || start_x + offset_x == self.width() {
                offset_y += 1;
                if offset_y == CHARACTER_HEIGHT || start_y + offset_y == self.height() {
                    return;
                }
                offset_x = 0;
            }
        }
    }

    fn newline(&mut self) {
        self.current_x = 0;
        self.current_y += 1;
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.newline();
                continue;
            }

            if c == '\t' {
                self.current_x += 4;
                continue;
            }

            if self.current_x >= self.max_x {
                self.newline();
            }

            if self.current_y >= self.max_y {
                self.clean();
            }

            self.print_char(c, self.current_x, self.current_y);

            self.current_x += 1;
        }

        Ok(())
    }
}

fn get_bit(char_font: u8, i: usize) -> u8 {
    char_font & (0x80 >> i)
}
