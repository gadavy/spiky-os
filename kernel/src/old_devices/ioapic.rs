use acpi::platform::ProcessorState;
use acpi::InterruptModel;
use spin::lock_api::Mutex;
use x2apic::ioapic::{IoApic, IrqFlags, IrqMode, RedirectionTableEntry};

pub static IO_APIC: Mutex<()> = Mutex::new(());

pub fn init() {
    let acpi = super::acpi::ACPI.read();
    let Some(madt) = acpi.madt() else { return };
    let InterruptModel::Apic(apic) = &madt.interrupt_mode else { return };
    let Some(info) = &madt.processor_info else { return };

    log::info!("CPU INFO:");

    log::info!(
        ">>> BOOT_CPU_CORE: local_apic_id={}, is_ap={}",
        info.boot_processor.local_apic_id,
        info.boot_processor.is_ap
    );

    for cpu in &info.application_processors {
        if cpu.state == ProcessorState::Disabled {
            continue;
        }

        log::info!(
            ">>> APP_CPU_CORE: local_apic id={}, is_ap={}",
            cpu.local_apic_id,
            cpu.is_ap
        );
    }

    log::info!("I/O APIC INFO:");

    for io_apic in &apic.io_apics {
        log::info!(
            ">>> I/O APIC: id={}, address={}, interrupt_base={}",
            io_apic.id,
            io_apic.address,
            io_apic.global_system_interrupt_base
        );
    }

    log::info!("APIC interrupt source overrides:");

    for ov in &apic.interrupt_source_overrides {
        log::info!(
            ">>> interrupt overrides: isa source: {}, gsi: {}, polarity: {:?}, trigger_mode: {:?}",
            ov.isa_source,
            ov.global_system_interrupt,
            ov.polarity,
            ov.trigger_mode
        );
    }

    // unsafe {
    //     let mut ioapic = IoApic::new(addr);
    //
    //     ioapic.init(irq_offset);
    //
    //     let mut entry = RedirectionTableEntry::default();
    //     entry.set_mode(IrqMode::Fixed);
    //     entry.set_flags(IrqFlags::LEVEL_TRIGGERED | IrqFlags::LOW_ACTIVE | IrqFlags::MASKED);
    //     entry.set_dest(2); // CPU(s)
    //     ioapic.set_table_entry(34, entry);
    //
    //     ioapic.enable_irq(irq_number);
    // }
}
