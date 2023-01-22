use bootloader_api::info::PixelFormat;

pub const BLACK: Color = Color::new(0x0000_0000);
pub const WHITE: Color = Color::new(0x00ff_ffff);
pub const RED: Color = Color::new(0x00ff_4757);
pub const ORANGE: Color = Color::new(0x00ff_8200);
pub const GREEN: Color = Color::new(0x0046_c93a);

#[derive(Copy, Clone)]
pub struct Color {
    channel: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(value: u32) -> Self {
        Self {
            channel: (value >> 24) as u8,
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
        }
    }

    pub fn to_bytes(self, format: PixelFormat) -> [u8; 4] {
        match format {
            PixelFormat::Rgb => [self.r, self.g, self.b, self.channel],
            PixelFormat::Bgr => [self.b, self.g, self.r, self.channel],
            PixelFormat::U8 => [self.gray(), self.gray(), self.gray(), 0],
            format => unimplemented!("unimplemented pixel format {format:?}"),
        }
    }

    pub fn intensity(mut self, value: u8) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        fn calculate(value: u8, intensity: u8) -> u8 {
            ((u16::from(value) * u16::from(intensity)) / u16::from(u8::MAX)) as u8
        }

        self.r = calculate(self.r, value);
        self.g = calculate(self.g, value);
        self.b = calculate(self.b, value);

        self
    }

    #[inline]
    fn gray(self) -> u8 {
        self.r / 3 + self.b / 3 + self.g / 3
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color::new(value)
    }
}
