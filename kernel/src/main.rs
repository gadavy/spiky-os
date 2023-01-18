#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![no_main]

use core::panic::PanicInfo;

mod framebuffer;
mod gdt;
mod interrupts;

bootloader_api::entry_point!(kernel_entry);

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    kernel_init(info);

    loop {
        x86_64::instructions::hlt()
    }
}

fn kernel_init(info: &'static mut bootloader_api::BootInfo) {
    let fb = match info.framebuffer.as_mut() {
        Some(framebuffer) => framebuffer,
        None => panic!("no framebuffer"),
    };

    framebuffer::init(fb.info(), fb.buffer_mut());
    println!("Framebuffer initialized");

    gdt::init();
    println!("GDT initialized");

    interrupts::init(gdt::DOUBLE_FAULT_IST_INDEX);
    println!("IDT initialized");

    println!("kernel initialized successfully")
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
