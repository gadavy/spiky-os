#![no_std]
#![no_main]

use arch::interrupts;

#[arch::kernel_entry]
fn main() -> ! {
    log::info!("kernel main");

    loop {
        interrupts::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {
        interrupts::hlt();
    }
}
