pub mod exception;
pub mod irq;

fn eoi(_irq: u8) {}

/// Halts the CPU until the next interrupt arrives.
pub fn hlt() {
    x86_64::instructions::hlt();
}
