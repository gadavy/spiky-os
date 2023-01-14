use super::font::{CHARACTER_HEIGHT, CHARACTER_WIDTH, FONT_BASIC};
use super::Framebuffer;
use crate::framebuffer::color::{Color, BLACK, WAIT};

pub struct Printer<'a> {
    framebuffer: &'a mut Framebuffer,

    position_x: usize,
    position_y: usize,
}

impl<'a> Printer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self {
            framebuffer,
            position_x: 0,
            position_y: 0,
        }
    }

    pub fn print_string(&mut self, s: &str) {
        todo!()
    }

    pub fn print_char(&mut self, character: char, column: usize, line: usize) {
        let start = (
            (column * CHARACTER_WIDTH) as isize,
            (line * CHARACTER_HEIGHT) as isize,
        );

        // print from the offset within the framebuffer
        let (buffer_width, buffer_height) = (self.framebuffer.width(), self.framebuffer.height());
        let off_set_x: usize = if start.0 < 0 { -(start.0) as usize } else { 0 };
        let off_set_y: usize = if start.1 < 0 { -(start.1) as usize } else { 0 };
        let mut j = off_set_x;
        let mut i = off_set_y;
        loop {
            let coordinate = (start.0 + j as isize, start.1 + i as isize);
            if self
                .framebuffer
                .contains(coordinate.0 as usize, coordinate.1 as usize)
            {
                let pixel = if j >= 1 {
                    // leave 1 pixel gap between two characters
                    let index = j - 1;
                    let char_font = FONT_BASIC[character as usize][i];
                    if get_bit(char_font, index) != 0 {
                        WAIT
                    } else {
                        BLACK
                    }
                } else {
                    BLACK
                };

                self.framebuffer
                    .draw(coordinate.0 as usize, coordinate.1 as usize, pixel);
            }
            j += 1;
            if j == CHARACTER_WIDTH || start.0 + j as isize == buffer_width as isize {
                i += 1;
                if i == CHARACTER_HEIGHT || start.1 + i as isize == buffer_height as isize {
                    return;
                }
                j = off_set_x;
            }
        }
    }
}

fn get_bit(char_font: u8, i: usize) -> u8 {
    char_font & (0x80 >> i)
}
