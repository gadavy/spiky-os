use crate::devices::local_apic;

pub mod exception;
pub mod irq;

fn eoi() {
    local_apic::LOCAL_APIC.end_of_interrupt();
}

/// Halts the CPU until the next interrupt arrives.
pub fn hlt() {
    x86_64::instructions::hlt();
}

/// Enable interrupts.
pub fn enable() {
    x86_64::instructions::interrupts::enable();
}
