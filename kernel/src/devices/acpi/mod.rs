use core::ptr::NonNull;

use acpi::fadt::Fadt;
use acpi::madt::Madt;
use acpi::sdt::Signature;
use acpi::{AcpiTables, HpetInfo, PhysicalMapping};
use spin::RwLock;
use x86_64::VirtAddr;

pub static ACPI: RwLock<AcpiInfo> = RwLock::new(AcpiInfo::empty());

pub struct AcpiInfo {
    /// Contains fixed hardware details, such as the addresses of the hardware register blocks.
    fadt: Option<FadtInfo>,

    /// Contains information about platform `InterruptModel` and `Processor`.
    madt: Option<MadtInfo>,

    /// Contains information about the High Precision Event Timer (HPET).
    hpet: Option<HpetInfo>,
}

impl AcpiInfo {
    const fn empty() -> Self {
        Self {
            fadt: None,
            madt: None,
            hpet: None,
        }
    }

    pub fn init(&mut self, phys_mem_offset: u64, rsdp_addr: u64) {
        log::debug!("Parse ACPI tables...");

        let handler = AcpiMemoryMapper::new(phys_mem_offset);
        let tables = match unsafe { AcpiTables::from_rsdp(handler, rsdp_addr as usize) } {
            Ok(tables) => tables,
            Err(e) => {
                log::warn!(">>> parsing failed: {e:?}");
                return;
            }
        };

        if let Ok(hpet) = HpetInfo::new(&tables) {
            self.hpet.replace(hpet);
        } else {
            log::warn!(">>> HPET not found");
        }

        if let Ok(Some(madt)) = unsafe { tables.get_sdt::<Madt>(Signature::MADT) } {
            if let Ok((im, pi)) = madt.parse_interrupt_model() {
                self.madt.replace(MadtInfo {
                    interrupt_mode: im,
                    processor_info: pi,
                });
            }
        } else {
            log::warn!(">>> MADT not found");
        }

        if let Ok(Some(fadt)) = unsafe { tables.get_sdt::<Fadt>(Signature::FADT) } {
            self.fadt.replace(FadtInfo {
                century_reg: fadt.century,
            });
        } else {
            log::warn!(">>> FADT not found");
        }
    }

    pub fn hpet(&self) -> Option<&HpetInfo> {
        self.hpet.as_ref()
    }

    pub fn madt(&self) -> Option<&MadtInfo> {
        self.madt.as_ref()
    }

    pub fn fadt(&self) -> Option<&FadtInfo> {
        self.fadt.as_ref()
    }
}

/// Contains fixed hardware details, such as the addresses of the hardware register blocks.
pub struct FadtInfo {
    pub century_reg: u8,
}

/// Contains information about platform `InterruptModel` and `Processor`.
pub struct MadtInfo {
    pub interrupt_mode: acpi::InterruptModel,
    pub processor_info: Option<acpi::platform::ProcessorInfo>,
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
