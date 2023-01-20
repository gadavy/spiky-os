#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
mod framebuffer;
mod gdt;
mod interrupts;
mod logger;

bootloader_api::entry_point!(kernel_entry);

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    logger::init();
    let fb = info.framebuffer.as_mut().expect("no framebuffer");

    framebuffer::init(fb.info(), fb.buffer_mut());
    log::debug!("Framebuffer initialized.");

    gdt::init();
    log::debug!("GDT initialized.");

    interrupts::init();
    log::debug!("IDT initialized.");

    drivers::init_default();
    log::debug!("Drivers initialized.");

    interrupts::enable();
    log::debug!("Interrupts enabled.");

    log::debug!("Kernel initialized successfully.\n");

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
