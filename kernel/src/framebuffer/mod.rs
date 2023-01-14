use bootloader_api::info::FrameBufferInfo;
use spin::Mutex;

pub mod color;
mod font;
mod printer;

pub use printer::Printer;

pub struct Framebuffer {
    info: FrameBufferInfo, // TODO: replace to fields???
    buf: &'static mut [u8],
}

impl Framebuffer {
    pub fn new(info: FrameBufferInfo, buf: &'static mut [u8]) -> Self {
        Self { info, buf }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.info.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.info.height
    }

    pub fn feel<T: color::Color>(&mut self, color: T) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.draw(x, y, color)
            }
        }
    }

    pub fn draw<T: color::Color>(&mut self, x: usize, y: usize, color: T) {
        if !self.contains(x, y) {
            return;
        }

        let range = self.buffer_range(x, y);
        let buf = &mut self.buf[range];

        color.write_to_slice(self.info.pixel_format, self.info.bytes_per_pixel, buf);
    }

    #[inline]
    fn contains(&self, x: usize, y: usize) -> bool {
        x <= self.width() && y <= self.height()
    }

    fn buffer_range(&self, x: usize, y: usize) -> core::ops::Range<usize> {
        let pixel_offset = y * self.info.stride + x;
        let byte_start = pixel_offset * self.info.bytes_per_pixel;
        let byte_end = byte_start + self.info.bytes_per_pixel;

        byte_start..byte_end
    }
}
