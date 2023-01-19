pub mod keyboard;

pub fn init_default() {
    keyboard::PC_KEYBOARD.init(default_keyboard_handler);
}

fn default_keyboard_handler(c: char) {
    match c {
        '\u{000C}' => crate::framebuffer::clear(), // CTRL + L
        c => crate::print!("{c}"),
    }
}
