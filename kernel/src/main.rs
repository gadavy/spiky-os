#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;
use kernel::devices;
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
    devices::init_framebuffer(fb.info(), fb.buffer_mut());
    devices::init_uart();

    // Init global descriptor table.
    gdt::init();

    // Init interrupts.
    interrupts::init();

    // Init other drivers.
    devices::init_keyboard();

    // Init memory.
    memory::init(phys_mem_offset, &info.memory_regions);

    // Enable interrupts.
    interrupts::enable();
    log::debug!("Interrupts enabled");

    log::debug!("Kernel initialized successfully");

    let x = alloc::boxed::Box::new(32);
    log::debug!("box ptr={x:p}");

    for _ in 0..1 {
        let mut y = alloc::vec::Vec::new();
        log::debug!("vec ptr={:p}", y.as_slice());

        for i in 0..500 {
            y.push(i);

            if i % 100 == 0 {
                log::debug!("vec ptr={:p}", y.as_slice());
            }
        }
    }

    loop {
        x86_64::instructions::hlt();
    }
}
