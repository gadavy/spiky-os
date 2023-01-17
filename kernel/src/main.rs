#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![no_main]

use core::panic::PanicInfo;

mod framebuffer;
mod gdt;
mod interrupts;

bootloader_api::entry_point!(entry);

fn entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    match info.framebuffer.as_mut() {
        Some(buf) => framebuffer::init(buf.info(), buf.buffer_mut()),
        None => panic!("no framebuffer"),
    }

    println!("Framebuffer initialized");

    gdt::init();
    println!("GDT initialized");

    interrupts::init(gdt::DOUBLE_FAULT_IST_INDEX);
    println!("IDT initialized");

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    loop {
        x86_64::instructions::hlt()
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    loop {}
}
