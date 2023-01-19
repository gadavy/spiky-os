#![cfg_attr(not(test), no_std)]
#![no_main]

use core::panic::PanicInfo;

mod framebuffer;

bootloader_api::entry_point!(kernel_entry);

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    match info.framebuffer.as_mut() {
        Some(buf) => framebuffer::init(buf.info(), buf.buffer_mut()),
        None => panic!("no framebuffer"),
    }

    println!("framebuffer initialized");

    loop {
        x86_64::instructions::hlt()
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] {info:#?}");

    loop {}
}
