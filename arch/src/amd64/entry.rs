use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;

use crate::logger;

use super::debug;
use super::devices;
use super::gdt;
use super::idt;
use super::memory;
use super::paging;

bootloader_api::entry_point!(arch_entry, config = &BOOTLOADER_CONFIG);

static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn arch_entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    extern "Rust" {
        fn kernel_main() -> !;
    }

    // Init logging.
    devices::init_serial();
    devices::init_display(info.framebuffer.as_mut());
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
    paging::test();

    gdt::init(0);
    idt::init_bsp();

    memory::init_heap();

    log::debug!("jump to kernel main");

    unsafe { kernel_main() }
}
