use crate::devices::lapic;

pub mod exception;
pub mod irq;

fn eoi() {
    unsafe { lapic::LOCAL_APIC.as_mut().unwrap().end_of_interrupt() };
}

/// Halts the CPU until the next interrupt arrives.
pub fn hlt() {
    x86_64::instructions::hlt();
}

pub fn enable() {
    x86_64::instructions::interrupts::enable();
}
