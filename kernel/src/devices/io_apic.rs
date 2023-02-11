use alloc::vec::Vec;

use acpi::platform::interrupt;
use acpi::platform::interrupt::{Polarity, TriggerMode};
use x2apic::ioapic::{IoApic, IrqFlags, RedirectionTableEntry};
use x86_64::structures::paging::{Page, PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::KERNEL_MAPPER;

static mut IO_APICS: Vec<IoApicWrapper> = Vec::new();

pub fn init(phys_mem_offset: u64) {
    let acpi = super::acpi::ACPI.read();
    let Some(apic) = acpi.apic() else { return };
    let Some(info) = acpi.madt() else { return };

    let bsp_apic_id = info
        .processor_info
        .as_ref()
        .unwrap()
        .boot_processor
        .local_apic_id as u8;

    for io_apic in &apic.io_apics {
        unsafe { init_io_apic(VirtAddr::new(phys_mem_offset), io_apic) };
    }

    //Map the legacy PC-compatible IRQs (0-15).
    for legacy_irq in 0..=15 {
        unsafe { init_override(legacy_irq, bsp_apic_id, &apic.interrupt_source_overrides) };
    }
}

unsafe fn init_io_apic(phys_mem_offset: VirtAddr, info: &interrupt::IoApic) {
    let phys_addr = PhysAddr::new(u64::from(info.address));
    let virt_addr = phys_mem_offset + phys_addr.as_u64();

    map_memory(phys_addr, virt_addr);

    IO_APICS.push(IoApicWrapper::new(
        virt_addr,
        u8::try_from(info.global_system_interrupt_base).unwrap(),
    ));
}

unsafe fn init_override(
    irq: u8,
    bsp_apic_id: u8,
    overrides: &[interrupt::InterruptSourceOverride],
) {
    let Some((gci, trigger_mode, polarity )) = prepare_override(irq, overrides) else { return };

    let mut entry = RedirectionTableEntry::default();
    entry.set_vector(32 + irq);
    entry.set_dest(bsp_apic_id);
    entry.set_flags(irq_frags(trigger_mode, polarity));

    if let Some(io_apic) = IO_APICS
        .iter_mut()
        .find(|io| gci >= io.gsi_start && gci <= io.gsi_end)
    {
        io_apic.io_apic.set_table_entry(gci, entry)
    }
}

fn prepare_override(
    irq: u8,
    overrides: &[interrupt::InterruptSourceOverride],
) -> Option<(u8, &TriggerMode, &Polarity)> {
    if let Some(value) = overrides.iter().find(|ov| ov.isa_source == irq) {
        Some((
            u8::try_from(value.global_system_interrupt).unwrap(),
            &value.trigger_mode,
            &value.polarity,
        ))
    } else if overrides
        .iter()
        .any(|ov| ov.global_system_interrupt == u32::from(irq) && ov.isa_source != irq)
        && !overrides.iter().any(|ov| ov.isa_source == irq)
    {
        None
    } else {
        Some((irq, &TriggerMode::SameAsBus, &Polarity::SameAsBus))
    }
}

fn irq_frags(trigger_mode: &TriggerMode, polarity: &Polarity) -> IrqFlags {
    match (trigger_mode, polarity) {
        (TriggerMode::Level, Polarity::SameAsBus | Polarity::ActiveHigh) => {
            IrqFlags::LEVEL_TRIGGERED
        }
        (TriggerMode::Edge | TriggerMode::SameAsBus, Polarity::ActiveLow) => IrqFlags::LOW_ACTIVE,
        (TriggerMode::Level, Polarity::ActiveLow) => {
            IrqFlags::LEVEL_TRIGGERED | IrqFlags::LOW_ACTIVE
        }
        (_, _) => IrqFlags::empty(),
    }
}

unsafe fn map_memory(phys_addr: PhysAddr, virt_addr: VirtAddr) {
    let page = Page::containing_address(virt_addr);
    let frame = PhysFrame::containing_address(phys_addr);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;

    let mut mapper_guard = KERNEL_MAPPER.lock();
    let mapper = mapper_guard.as_mut().expect("expected initialized mapper");

    if mapper.translate(page.start_address()).is_none() {
        mapper
            .map_phys(page, frame, flags)
            .expect("failed to map I/O APIC")
            .flush();
    }
}

struct IoApicWrapper {
    io_apic: IoApic,
    gsi_start: u8,
    gsi_end: u8,
}

impl IoApicWrapper {
    unsafe fn new(base_addr: VirtAddr, gsi_start: u8) -> Self {
        let mut io_apic = IoApic::new(base_addr.as_u64());
        let gsi_end = gsi_start + io_apic.max_table_entry();

        Self {
            io_apic,
            gsi_start,
            gsi_end,
        }
    }
}
