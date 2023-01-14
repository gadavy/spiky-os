#![cfg_attr(not(test), no_std)]
#![no_main]

mod framebuffer;

use crate::framebuffer::color::BLACK;
use crate::framebuffer::{color, Framebuffer};
use bootloader_api::BootloaderConfig;
use core::panic::PanicInfo;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.frame_buffer.minimum_framebuffer_height = Some(720);
    config
};

bootloader_api::entry_point!(entry, config = &BOOTLOADER_CONFIG);

fn entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buf = framebuffer.buffer_mut();

        let mut f_buffer = Framebuffer::new(info, buf);

        f_buffer.feel(BLACK);

        let mut printer = framebuffer::Printer::new(&mut f_buffer);

        for (i, c) in "hello world!".chars().enumerate() {
            printer.print_char(c, i, 0);
        }

        for (i, c) in "kernel started".chars().enumerate() {
            printer.print_char(c, i, 1);
        }
    } else {
        panic!("no framebuffer")
    }

    loop {
        x86_64::instructions::hlt()
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
