use bootloader_api::info::FrameBuffer;

pub mod acpi;
mod cpu;
pub mod display;
pub mod io_apic;
pub mod lapic;
pub mod rtc;
pub mod serial;

pub fn init_early(info: Option<&'static mut FrameBuffer>) {
    serial::init();

    if let Some(fb) = info {
        display::init(fb.info(), fb.buffer_mut())
    };
}

pub fn init_acpi(phys_mem_offset: u64, rsdp_addr: u64) {
    acpi::ACPI.write().init(phys_mem_offset, rsdp_addr);
    io_apic::init(phys_mem_offset);
}

pub fn init_local_apic(phys_mem_offset: u64) {
    lapic::init(phys_mem_offset);
}
