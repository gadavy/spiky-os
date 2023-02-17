use crate::devices::local_apic;

pub mod exception;
pub mod irq;

#[inline]
fn eoi() {
    local_apic::LOCAL_APIC.end_of_interrupt();
}

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    x86_64::instructions::hlt();
}

/// Enable interrupts.
#[inline]
pub fn enable() {
    x86_64::instructions::interrupts::enable();
}
