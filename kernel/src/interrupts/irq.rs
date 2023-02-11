use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;

use super::eoi;

pub extern "x86-interrupt" fn pit_stack(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn keyboard(_stack: InterruptStackFrame) {
    let _: u8 = unsafe { PortReadOnly::new(0x60).read() };

    log::debug!("keyboard interrupt!");

    eoi();
}

pub extern "x86-interrupt" fn cascade(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn com2(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn com1(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn lpt2(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn floppy(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn lpt1(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn rtc(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn pci1(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn pci2(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn pci3(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn mouse(_stack: InterruptStackFrame) {
    let _: u8 = unsafe { PortReadOnly::new(0x60).read() };

    log::debug!("mouse interrupt!");

    eoi();
}

pub extern "x86-interrupt" fn fpu(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn ata1(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn ata2(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn lapic_timer(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn lapic_error(_stack: InterruptStackFrame) {
    eoi();
}

pub extern "x86-interrupt" fn unimplemented(_stack: InterruptStackFrame) {
    eoi();
}
