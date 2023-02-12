use bootloader_api::info::FrameBuffer;
use x86_64::VirtAddr;

pub mod acpi;
mod cpu;
pub mod display;
pub mod io_apic;
pub mod local_apic;
pub mod rtc;
pub mod serial;

pub fn init_early(info: Option<&'static mut FrameBuffer>) {
    serial::COM1.lock().init();
    serial::COM2.lock().init();

    if let Some(fb) = info {
        display::DISPLAY.lock().init(fb.info(), fb.buffer_mut());
    };
}

pub fn init(phys_mem_offset: u64, rsdp_addr: Option<u64>) {
    let phys_mem_offset = VirtAddr::new(phys_mem_offset);

    log::trace!("Init Local APIC");
    local_apic::LOCAL_APIC.init(phys_mem_offset);

    if let Some(rsdp_addr) = rsdp_addr {
        log::trace!("Parse ACPI");
        acpi::ACPI.write().init(phys_mem_offset, rsdp_addr);

        let acpi_info = acpi::ACPI.read();

        if let Some((apic, bsp)) = acpi_info
            .apic
            .as_ref()
            .zip(acpi_info.boot_processor.as_ref())
        {
            log::trace!("Init IO APIC");

            let bsp_apic_id = u8::try_from(bsp.local_apic_id).unwrap();
            io_apic::IO_APICS.init(phys_mem_offset, bsp_apic_id, apic);
        }

        if let Some(century) = acpi_info.century_reg {
            log::trace!("Init RTC");
            rtc::RTC.lock().init(century)
        }
    }
}
