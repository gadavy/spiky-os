use bootloader_api::info;
use spin::Mutex;

use crate::framebuffer::{Framebuffer, PixelFormat};

pub static DISPLAY: Mutex<Option<Framebuffer>> = Mutex::new(None);

pub fn init(info: info::FrameBufferInfo, buf: &'static mut [u8]) {
    let px_format = match info.pixel_format {
        info::PixelFormat::Rgb => PixelFormat::Rgb,
        info::PixelFormat::Bgr => PixelFormat::Bgr,
        fmt => unimplemented!("unsupported pixel format {:?}", fmt),
    };

    DISPLAY.lock().replace(Framebuffer::new(
        info.bytes_per_pixel,
        info.width,
        info.height,
        info.stride,
        px_format,
        buf,
    ));
}
