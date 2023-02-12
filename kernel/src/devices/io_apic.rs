use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use acpi::platform::interrupt::{
    Apic, InterruptSourceOverride, IoApic as IoApicInfo, Polarity, TriggerMode,
};
use x2apic::ioapic;
use x2apic::ioapic::{IrqFlags, RedirectionTableEntry};
use x86_64::structures::paging::{Page, PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::KERNEL_MAPPER;

pub static IO_APICS: IoApics = IoApics::empty();

pub struct IoApics {
    list: UnsafeCell<Vec<IoApic>>,
}

impl IoApics {
    const fn empty() -> Self {
        Self {
            list: UnsafeCell::new(Vec::new()),
        }
    }

    pub(super) fn init(&self, phys_mem_offset: VirtAddr, bsp_apic_id: u8, info: &Apic) {
        for info in &info.io_apics {
            unsafe { self.add_io_apic(phys_mem_offset, info) };
        }

        let overrides = &info.interrupt_source_overrides;

        for legacy_irq in 0..=15 {
            unsafe { self.init_override(legacy_irq, bsp_apic_id, overrides) };
        }
    }

    unsafe fn add_io_apic(&self, phys_mem_offset: VirtAddr, info: &IoApicInfo) {
        let phys_addr = PhysAddr::new(u64::from(info.address));
        let virt_addr = phys_mem_offset + phys_addr.as_u64();

        map_memory(phys_addr, virt_addr);

        if let Some(list) = self.list.get().as_mut() {
            let mut io_apic = ioapic::IoApic::new(virt_addr.as_u64());

            let gsi_start = u8::try_from(info.global_system_interrupt_base).unwrap();
            let gsi_end = gsi_start + io_apic.max_table_entry();

            list.push(IoApic {
                io_apic,
                gsi_start,
                gsi_end,
            })
        }
    }

    unsafe fn init_override(&self, irq: u8, apic_id: u8, overrides: &[InterruptSourceOverride]) {
        let (gci, trigger_mode, polarity) = match prepare_override(irq, overrides) {
            Some((gci, trigger_mode, polarity)) => (gci, trigger_mode, polarity),
            None => return,
        };

        let mut entry = RedirectionTableEntry::default();
        entry.set_vector(32 + irq);
        entry.set_dest(apic_id);
        entry.set_flags(irq_frags(trigger_mode, polarity));

        if let Some(io_apic) = self.find_io_apic(gci) {
            io_apic.set_table_entry(gci, entry)
        }
    }

    unsafe fn find_io_apic(&self, gci: u8) -> Option<&mut IoApic> {
        self.list
            .get()
            .as_mut()?
            .iter_mut()
            .find(|io| gci >= io.gsi_start && gci <= io.gsi_end)
    }
}

unsafe impl Sync for IoApics {}

struct IoApic {
    io_apic: ioapic::IoApic,
    gsi_start: u8,
    gsi_end: u8,
}

impl Deref for IoApic {
    type Target = ioapic::IoApic;

    fn deref(&self) -> &Self::Target {
        &self.io_apic
    }
}

impl DerefMut for IoApic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.io_apic
    }
}

fn prepare_override(
    irq: u8,
    overrides: &[InterruptSourceOverride],
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
