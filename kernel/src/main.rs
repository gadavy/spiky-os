#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;
use kernel::drivers;
use kernel::gdt;
use kernel::interrupts;
use kernel::logger;
use kernel::memory;

#[cfg(not(test))]
bootloader_api::entry_point!(kernel_entry, config = &BOOTLOADER_CONFIG);

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn kernel_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb = info.framebuffer.as_mut().expect("no framebuffer");
    let phys_mem_offset = *info.physical_memory_offset.as_mut().expect("no mem offset");

    // Init logger.
    logger::init();

    // Init base drivers.
    drivers::init_framebuffer(fb.info(), fb.buffer_mut());
    drivers::init_uart();

    // Init global descriptor table.
    gdt::init();

    // Init interrupts.
    interrupts::init();

    // Init other drivers.
    drivers::init_keyboard();

    // Init memory.
    memory::init(phys_mem_offset, &info.memory_regions);

    // Enable interrupts.
    interrupts::enable();
    log::debug!("Interrupts enabled");

    log::debug!("Kernel initialized successfully");

    loop {
        x86_64::instructions::hlt();
    }
}
