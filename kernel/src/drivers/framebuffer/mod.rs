use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use spin::Mutex;

pub use color::*;
pub use geometry::*;

mod color;
mod geometry;

pub static FRAMEBUFFER: Mutex<FramebufferDriver> = Mutex::new(FramebufferDriver::empty());

pub struct FramebufferDriver {
    inner: Option<InnerFrameBuffer>,
}

impl FramebufferDriver {
    const fn empty() -> Self {
        Self { inner: None }
    }

    pub fn init(&mut self, info: FrameBufferInfo, buf: &'static mut [u8]) {
        self.inner.replace(InnerFrameBuffer::new(info, buf));
    }

    pub fn fill_pixel<P, C>(&mut self, point: P, color: C)
    where
        P: Into<Point>,
        C: Into<Color>,
    {
        let Some(inner) = self.inner.as_mut() else { return };

        inner.fill_pixel(point.into(), color.into());
    }

    #[allow(dead_code)] // will be used later.
    pub fn fill_rect<C>(&mut self, rect: Rect, color: C)
    where
        C: Into<Color>,
    {
        let Some(inner) = self.inner.as_mut() else { return };

        inner.fill_region(rect, color.into());
    }

    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        let Some(inner) = self.inner.as_mut() else { return };

        inner.fill_region(inner.rect, color.into());
    }
}

pub struct InnerFrameBuffer {
    bpp: usize,
    stride: usize,
    format: PixelFormat,
    rect: Rect,

    buf: &'static mut [u8],
}

impl InnerFrameBuffer {
    fn new(info: FrameBufferInfo, buf: &'static mut [u8]) -> Self {
        let rect = Rect::new(0, 0, info.width, info.height);

        Self {
            bpp: info.bytes_per_pixel,
            stride: info.stride,
            format: info.pixel_format,
            buf,
            rect,
        }
    }

    fn fill_pixel(&mut self, point: Point, color: Color) {
        if !self.rect.contains(point) {
            return;
        }

        let px_start = (point.y() * self.stride + point.x()) * self.bpp;
        let px_end = px_start + self.bpp;

        self.buf[px_start..px_end].copy_from_slice(&color.to_bytes(self.format)[..self.bpp]);
    }

    fn fill_region(&mut self, rect: Rect, color: Color) {
        let Some(intersection) = self.rect.intersection(rect) else { return };
        let color = &color.to_bytes(self.format)[..self.bpp];
        let mut offset = (intersection.min().y() * self.stride + intersection.min().x()) * self.bpp;

        for _ in 0..intersection.height() {
            for x in 0..intersection.width() {
                let px_start = offset + x * self.bpp;
                let px_end = px_start + self.bpp;

                self.buf[px_start..px_end].copy_from_slice(color);
            }

            offset += self.stride * self.bpp;
        }
    }
}
