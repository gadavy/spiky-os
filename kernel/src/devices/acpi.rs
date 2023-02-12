use alloc::vec::Vec;
use core::ptr::NonNull;

use acpi::fadt::Fadt;
use acpi::platform::interrupt::Apic;
use acpi::platform::Processor;
use acpi::sdt::Signature;
use acpi::{AcpiTables, InterruptModel, PhysicalMapping};
use spin::RwLock;
use x86_64::VirtAddr;

pub static ACPI: RwLock<AcpiInfo> = RwLock::new(AcpiInfo::empty());

pub struct AcpiInfo {
    pub apic: Option<Apic>,
    pub boot_processor: Option<Processor>,
    pub ap_processors: Vec<Processor>,
    pub century_reg: Option<u8>,
}

impl AcpiInfo {
    const fn empty() -> Self {
        Self {
            apic: None,
            boot_processor: None,
            ap_processors: Vec::new(),
            century_reg: None,
        }
    }

    pub fn init(&mut self, phys_mem_offset: VirtAddr, rsdp_addr: u64) {
        log::trace!("Parse ACPI tables...");

        let handler = AcpiMemoryMapper::new(phys_mem_offset);
        let tables = unsafe {
            AcpiTables::from_rsdp(handler, rsdp_addr as usize).expect("AcpiTables should be parsed")
        };

        if let Ok(info) = tables.platform_info() {
            if let InterruptModel::Apic(apic) = info.interrupt_model {
                self.apic.replace(apic);
            }

            if let Some(pi) = info.processor_info {
                self.boot_processor.replace(pi.boot_processor);
                self.ap_processors = pi.application_processors;
            }
        }

        if let Ok(Some(fadt)) = unsafe { tables.get_sdt::<Fadt>(Signature::FADT) } {
            self.century_reg.replace(fadt.century);
        }
    }
}

#[derive(Copy, Clone)]
struct AcpiMemoryMapper {
    phys_mem_offset: VirtAddr,
}

impl AcpiMemoryMapper {
    fn new(phys_mem_offset: VirtAddr) -> Self {
        Self { phys_mem_offset }
    }
}

impl acpi::AcpiHandler for AcpiMemoryMapper {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let virt_addr = self.phys_mem_offset + physical_address;

        PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr()).unwrap(),
            size,
            size,
            *self,
        )
    }

    fn unmap_physical_region<T>(_: &PhysicalMapping<Self, T>) {}
}
