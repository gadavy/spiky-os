use bootloader_api::info::PixelFormat;

pub const BLACK: Color = Color::new(0x0000_0000);
pub const WHITE: Color = Color::new(0x00ff_ffff);
pub const RED: Color = Color::new(0x00ff_4757);
pub const ORANGE: Color = Color::new(0x00ff_8200);
pub const GREEN: Color = Color::new(0x0046_c93a);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    channel: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates new color from u32 representation (hex).
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(value: u32) -> Self {
        Self {
            channel: (value >> 24) as u8,
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
        }
    }

    /// Converts `Color` to formatted bytes array.
    pub fn to_bytes(self, format: PixelFormat) -> [u8; 4] {
        match format {
            PixelFormat::Rgb => [self.r, self.g, self.b, self.channel],
            PixelFormat::Bgr => [self.b, self.g, self.r, self.channel],
            PixelFormat::U8 => [self.gray(), self.gray(), self.gray(), 0],
            format => unimplemented!("unimplemented pixel format {format:?}"),
        }
    }

    /// Returns a new color with the given intensity.
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

#[cfg(test)]
mod tests {
    use super::{Color, PixelFormat};

    #[test]
    fn color_from() {
        let cases = [
            (0x00000000, (0x00, 0x00, 0x00, 0x00)),
            (0xff000000, (0xff, 0x00, 0x00, 0x00)),
            (0x00ff0000, (0x00, 0xff, 0x00, 0x00)),
            (0x0000ff00, (0x00, 0x00, 0xff, 0x00)),
            (0x000000ff, (0x00, 0x00, 0x00, 0xff)),
        ];

        for (value, (channel, r, g, b)) in cases {
            assert_eq!(Color::from(value), Color { channel, r, g, b });
        }
    }

    #[test]
    fn color_to_bytes() {
        let c = Color::new(0x11223344);
        assert_eq!(c.to_bytes(PixelFormat::Rgb), [0x22, 0x33, 0x44, 0x11]);
        assert_eq!(c.to_bytes(PixelFormat::Bgr), [0x44, 0x33, 0x22, 0x11]);
        assert_eq!(c.to_bytes(PixelFormat::U8), [0x32, 0x32, 0x32, 0x0]);
    }

    #[test]
    #[should_panic]
    fn color_to_bytes_panic() {
        Color::new(0x00ffffff).to_bytes(PixelFormat::Unknown {
            red_position: 0,
            green_position: 0,
            blue_position: 0,
        });
    }

    #[test]
    fn color_intensity() {
        let c = Color::new(0x007f7f7f);
        assert_eq!(c.intensity(0xff), 0x007f7f7f.into());
        assert_eq!(c.intensity(0x7f), 0x003f3f3f.into());
        assert_eq!(c.intensity(0x00), 0x00000000.into());
    }
}
