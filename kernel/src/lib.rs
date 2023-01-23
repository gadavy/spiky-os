#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod drivers;
pub mod gdt;
pub mod interrupts;
pub mod logger;

#[cfg(not(test))] // TODO: fix later
pub mod memory;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
