#![no_std]
#![no_main]

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;

bootloader_api::entry_point!(kernel::entry, config = &BOOTLOADER_CONFIG);

static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {
        core::hint::spin_loop()
    }
}
