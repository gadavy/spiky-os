#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
mod framebuffer;
mod gdt;
mod interrupts;

bootloader_api::entry_point!(kernel_entry);

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb = info.framebuffer.as_mut().expect("no framebuffer");

    framebuffer::init(fb.info(), fb.buffer_mut());
    println!("Framebuffer initialized.");

    gdt::init();
    println!("GDT initialized.");

    interrupts::init();
    println!("IDT initialized.");

    drivers::init_default();
    println!("Drivers initialized.");

    interrupts::enable();
    println!("Interrupts enabled.");

    println!("Kernel initialized successfully.\n");

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
