use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use spin::Mutex;

pub static DISPLAY: Mutex<Framebuffer> = Mutex::new(Framebuffer::empty());

const CHARACTER_WIDTH: usize = 9;
const CHARACTER_HEIGHT: usize = 16;

/// `RobotoMono` bitmap.
static FONT: [[[u8; CHARACTER_WIDTH]; CHARACTER_HEIGHT]; 256] =
    include!("../../../fonts/roboto-mono-bitmap.txt");

pub struct Framebuffer {
    inner: Option<InnerFramebuffer>,
}

impl Framebuffer {
    pub const fn empty() -> Self {
        Self { inner: None }
    }

    pub(super) fn init(&mut self, info: FrameBufferInfo, buf: &'static mut [u8]) {
        buf.fill(0); // clear screen.

        self.inner.replace(InnerFramebuffer {
            buf,
            info,
            current_x: 0,
            current_y: 0,
        });
    }
}

impl core::fmt::Write for Framebuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Some(inner) = self.inner.as_mut() {
            for c in s.chars() {
                inner.draw_char(c);
            }
        }

        Ok(())
    }
}

pub struct InnerFramebuffer {
    buf: &'static mut [u8],
    info: FrameBufferInfo,

    current_x: usize,
    current_y: usize,
}

impl InnerFramebuffer {
    fn draw_char(&mut self, character: char) {
        if character == '\n' {
            self.newline();
            return;
        }

        if self.current_x + CHARACTER_WIDTH >= self.width() {
            self.newline();
        }

        if self.current_y + CHARACTER_HEIGHT >= self.height() {
            self.clear();
            self.current_x = 0;
            self.current_y = 0;
        }

        let mut offset = (self.current_y * self.stride() + self.current_x) * self.bpp();

        for column in FONT[character as usize] {
            for (x, intensity) in column.iter().enumerate() {
                let px_start = offset + x * self.bpp();
                let px_end = px_start + self.bpp();

                let bytes = &self.color_bytes(WHITE, *intensity)[..self.bpp()];

                self.buf[px_start..px_end].copy_from_slice(bytes);
            }

            offset += self.stride() * self.bpp();
        }

        self.current_x += CHARACTER_WIDTH;
    }

    fn clear(&mut self) {
        self.buf.fill(0);
    }

    fn newline(&mut self) {
        self.current_x = 0;
        self.current_y += CHARACTER_HEIGHT;
    }

    #[inline]
    fn bpp(&self) -> usize {
        self.info.bytes_per_pixel
    }

    #[inline]
    fn stride(&self) -> usize {
        self.info.stride
    }

    #[inline]
    fn width(&self) -> usize {
        self.info.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.info.height
    }

    #[inline]
    fn color_bytes(&self, color: Color, intensity: u8) -> [u8; 4] {
        color.intensity(intensity).to_bytes(self.info.pixel_format)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Colors
////////////////////////////////////////////////////////////////////////////////////////////////////

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
    const fn new(value: u32) -> Self {
        Self {
            channel: (value >> 24) as u8,
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
        }
    }

    /// Converts `Color` to formatted bytes array.
    fn to_bytes(self, format: PixelFormat) -> [u8; 4] {
        match format {
            PixelFormat::Rgb => [self.r, self.g, self.b, self.channel],
            PixelFormat::Bgr => [self.b, self.g, self.r, self.channel],
            PixelFormat::U8 => [self.gray(), self.gray(), self.gray(), 0],
            format => unimplemented!("unimplemented pixel format {format:?}"),
        }
    }

    /// Returns a new color with the given intensity.
    fn intensity(mut self, value: u8) -> Self {
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
        assert_eq!(c.to_bytes(PixelFormat::U8), [0x32, 0x32, 0x32, 0x0]);
    }

    #[test]
    fn color_intensity() {
        let c = Color::new(0x007f7f7f);
        assert_eq!(c.intensity(0xff), Color::new(0x007f7f7f));
        assert_eq!(c.intensity(0x7f), Color::new(0x003f3f3f));
        assert_eq!(c.intensity(0x00), Color::new(0x00000000));
    }
}
