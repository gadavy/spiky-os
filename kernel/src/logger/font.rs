/// The width of a character.
pub const CHARACTER_WIDTH: usize = 9;

/// The height of a character.
pub const CHARACTER_HEIGHT: usize = 16;

/// `RobotoMono` bitmap.
pub static FONT: [[[u8; CHARACTER_WIDTH]; CHARACTER_HEIGHT]; 256] =
    include!("../../assets/roboto-mono-bitmap.txt");
