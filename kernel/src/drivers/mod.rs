use bootloader_api::info::FrameBufferInfo;

pub mod framebuffer;
pub mod keyboard;
pub mod uart;

pub fn init_framebuffer(info: FrameBufferInfo, buf: &'static mut [u8]) {
    let mut fb = framebuffer::FRAMEBUFFER.lock();
    fb.init(info, buf);
    fb.fill(framebuffer::BLACK);
}

pub fn init_keyboard() {
    keyboard::PC_KEYBOARD.lock().init(default_keyboard_handler);
}

pub fn init_uart() {
    uart::UART.lock().init(uart::COM1_BASE);
}

fn default_keyboard_handler(c: char) {
    log::trace!("new char: {c}");
}
