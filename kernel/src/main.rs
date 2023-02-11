#![no_std]
#![no_main]

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;

use kernel::*;

bootloader_api::entry_point!(entry, config = &BOOTLOADER_CONFIG);

static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    // Init logging.
    devices::init_early(info.framebuffer.as_mut());
    logger::init(debug::write_log);

    log::info!("Spiky OS starting...");

    let physical_memory_offset = info
        .physical_memory_offset
        .into_option()
        .expect("Physical memory offset not specified in boot information");

    let tls_info = info
        .tls_template
        .into_option()
        .expect("TLS template not specified in boot information");

    // Init GDT and IDT early before TLS initialized.
    gdt::init_early();
    idt::init_early();

    // Init memory and TLS.
    memory::init(physical_memory_offset, &info.memory_regions);
    paging::init(0, tls_info);

    gdt::init(0);
    idt::init_bsp();

    memory::init_heap();

    devices::init_local_apic(physical_memory_offset);

    if let Some(rsdp_addr) = info.rsdp_addr.into_option() {
        log::debug!("try to init acpi and io_apic");

        devices::init_acpi(physical_memory_offset, rsdp_addr);
    }

    interrupts::enable();

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
