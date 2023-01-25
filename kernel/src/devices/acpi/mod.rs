use core::ptr::NonNull;

use acpi::fadt::Fadt;
use acpi::madt::Madt;
use acpi::sdt::Signature;
use acpi::{AcpiTables, HpetInfo, PhysicalMapping};
use spin::Once;
use x86_64::VirtAddr;

static HPET: Once<HpetInfo> = Once::new();
pub static MADT: Once<MadtInfo> = Once::new();
pub static FADT: Once<FadtInfo> = Once::new();

pub fn init(phys_mem_offset: u64, rsdp_address: u64) {
    log::debug!("start parsing ACPI tables");

    let handler = AcpiMemoryMapper::new(phys_mem_offset);
    let tables = match unsafe { AcpiTables::from_rsdp(handler, rsdp_address as usize) } {
        Ok(tables) => tables,
        Err(e) => {
            log::error!("parse ACPI tables failed: {e:?}");
            return;
        }
    };

    if let Ok(hpet) = HpetInfo::new(&tables) {
        HPET.call_once(|| hpet);
    }

    if let Ok(Some(madt)) = unsafe { tables.get_sdt::<Madt>(Signature::MADT) } {
        if let Ok(res) = madt.parse_interrupt_model() {
            MADT.call_once(|| res.into());
        }
    }

    if let Ok(Some(fadt)) = unsafe { tables.get_sdt::<Fadt>(Signature::FADT) } {
        FADT.call_once(|| fadt.into());
    }
}

pub struct FadtInfo {
    pub century: u8,
}

impl From<PhysicalMapping<AcpiMemoryMapper, Fadt>> for FadtInfo {
    fn from(value: PhysicalMapping<AcpiMemoryMapper, Fadt>) -> Self {
        Self {
            century: value.century,
        }
    }
}

pub struct MadtInfo {
    pub interrupt_mode: acpi::InterruptModel,
    pub processor_info: Option<acpi::platform::ProcessorInfo>,
}

impl From<(acpi::InterruptModel, Option<acpi::platform::ProcessorInfo>)> for MadtInfo {
    fn from(value: (acpi::InterruptModel, Option<acpi::platform::ProcessorInfo>)) -> Self {
        Self {
            interrupt_mode: value.0,
            processor_info: value.1,
        }
    }
}

#[derive(Copy, Clone)]
struct AcpiMemoryMapper {
    phys_mem_offset: u64,
}

impl AcpiMemoryMapper {
    fn new(phys_mem_offset: u64) -> Self {
        Self { phys_mem_offset }
    }
}

impl acpi::AcpiHandler for AcpiMemoryMapper {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let phys_mem_offset = VirtAddr::new(self.phys_mem_offset);
        let virtual_address = phys_mem_offset + physical_address;

        PhysicalMapping::new(
            usize::from(physical_address),
            NonNull::new(virtual_address.as_mut_ptr()).unwrap(),
            size,
            size,
            *self,
        )
    }

    fn unmap_physical_region<T>(_: &PhysicalMapping<Self, T>) {}
}
