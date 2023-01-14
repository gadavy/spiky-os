use bootloader_api::info::PixelFormat;

pub trait Color: Copy + Clone {
    fn red(self) -> u8;

    fn green(self) -> u8;

    fn blue(self) -> u8;

    fn gray(self) -> u8 {
        self.red() / 3 + self.blue() / 3 + self.green() / 3
    }

    fn write_to_slice(self, format: PixelFormat, size: usize, buf: &mut [u8]) {
        match (format, size) {
            (PixelFormat::Rgb, 3) => buf.copy_from_slice(&[self.red(), self.green(), self.blue()]),
            (PixelFormat::Bgr, 3) => buf.copy_from_slice(&[self.blue(), self.green(), self.red()]),
            (PixelFormat::U8, 1) => buf.copy_from_slice(&[self.gray()]),
            (format, size) => {
                unimplemented!(
                    "pixel format {:?} with bytes per pixel {} not supported",
                    format,
                    size
                )
            }
        }
    }
}

pub const BLACK: Rgb = Rgb::new(0x000000);
pub const WAIT: Rgb = Rgb::new(0xffffff);

#[derive(Copy, Clone)]
pub struct Rgb([u8; 3]);

impl Rgb {
    pub const fn new(value: u32) -> Self {
        Self([(value >> 16) as u8, (value >> 8) as u8, value as u8])
    }
}

impl Color for Rgb {
    #[inline]
    fn red(self) -> u8 {
        self.0[0]
    }

    #[inline]
    fn green(self) -> u8 {
        self.0[1]
    }

    #[inline]
    fn blue(self) -> u8 {
        self.0[2]
    }
}
