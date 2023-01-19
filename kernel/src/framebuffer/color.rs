pub const BLACK: Color = Color::new(0x000000);
pub const WHITE: Color = Color::new(0xFFFFFF);

#[derive(Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn new(value: u32) -> Self {
        Self {
            red: (value >> 16) as u8,
            green: (value >> 8) as u8,
            blue: value as u8,
        }
    }

    #[inline]
    pub fn red(self) -> u8 {
        self.red
    }

    #[inline]
    pub fn green(self) -> u8 {
        self.green
    }

    #[inline]
    pub fn blue(self) -> u8 {
        self.blue
    }

    #[inline]
    pub fn gray(self) -> u8 {
        self.red / 3 + self.blue / 3 + self.green / 3
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color::new(value)
    }
}
