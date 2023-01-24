use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use spin::Mutex;

pub use color::*;
pub use geometry::*;

mod color;
mod geometry;

pub static FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer::empty());

pub struct Framebuffer {
    bpp: usize,
    stride: usize,
    format: PixelFormat,
    rect: Rect,

    buf: Option<&'static mut [u8]>,
}

impl Framebuffer {
    const fn empty() -> Self {
        Self {
            bpp: 0,
            stride: 0,
            format: PixelFormat::Rgb,
            rect: Rect::new(0, 0, 1, 1),
            buf: None,
        }
    }

    pub fn init(&mut self, info: FrameBufferInfo, buf: &'static mut [u8]) {
        self.rect = Rect::new(0, 0, info.width, info.height);
        self.bpp = info.bytes_per_pixel;
        self.format = info.pixel_format;
        self.stride = info.stride;

        self.buf.replace(buf);
        self.fill(BLACK);
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.rect.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.rect.height()
    }

    pub fn fill_pixel<P, C>(&mut self, point: P, color: C)
    where
        P: Into<Point>,
        C: Into<Color>,
    {
        let point = point.into();

        if self.buf.is_none() || !self.rect.contains(point) {
            return;
        }

        let color = color.into();

        let buffer = {
            let offset = offset(self.stride, self.bpp, point);
            self.buf.as_mut().unwrap()[offset..offset + self.bpp].as_mut()
        };

        buffer.copy_from_slice(&color.to_bytes(self.format)[..self.bpp]);
    }

    pub fn fill_region<C>(&mut self, rect: Rect, color: C)
    where
        C: Into<Color>,
    {
        if self.buf.is_none() {
            return;
        }

        let Some(intersection) = self.rect.intersection(rect) else { return };
        let color = &color.into().to_bytes(self.format)[..self.bpp];

        let mut offset = offset(self.stride, self.bpp, intersection.min());
        let buffer = self.buf.as_mut().unwrap();

        for _ in 0..intersection.height() {
            for x in 0..intersection.width() {
                let px_start = offset + x * self.bpp;
                let px_end = px_start + self.bpp;

                buffer[px_start..px_end].copy_from_slice(color);
            }

            offset += self.stride * self.bpp;
        }
    }

    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.fill_region(self.rect, color);
    }
}

fn offset(stride: usize, bpp: usize, point: Point) -> usize {
    (point.y() * stride + point.x()) * bpp
}
