use super::PixelFormat;

pub const WHITE: Color = Color::new(0x00ff_ffff);

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
            assert_eq!(Color::new(value), Color { channel, r, g, b });
        }
    }

    #[test]
    fn color_to_bytes() {
        let c = Color::new(0x11223344);
        assert_eq!(c.to_bytes(PixelFormat::Rgb), [0x22, 0x33, 0x44, 0x11]);
        assert_eq!(c.to_bytes(PixelFormat::Bgr), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn color_intensity() {
        let c = Color::new(0x007f7f7f);
        assert_eq!(c.intensity(0xff), Color::new(0x007f7f7f));
        assert_eq!(c.intensity(0x7f), Color::new(0x003f3f3f));
        assert_eq!(c.intensity(0x00), Color::new(0x00000000));
    }
}
