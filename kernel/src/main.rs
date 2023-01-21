#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
mod gdt;
mod interrupts;
mod logger;

bootloader_api::entry_point!(kernel_entry);

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb = info.framebuffer.as_mut().expect("no framebuffer");
    logger::init(fb.info());

    drivers::init_framebuffer(fb.info(), fb.buffer_mut());
    log::debug!("Framebuffer initialized");

    drivers::init_uart();
    log::debug!("UART initialized");

    gdt::init();
    log::debug!("GDT initialized");

    interrupts::init();
    log::debug!("IDT initialized");

    drivers::init_keyboard();
    log::debug!("Keyboard driver initialized");

    interrupts::enable();
    log::debug!("Interrupts enabled");

    log::debug!("Kernel initialized successfully");

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
