use x86_64::structures::idt::InterruptStackFrame;

use crate::devices;

pub extern "x86-interrupt" fn pit_stack(_stack: InterruptStackFrame) {
    eoi(0);
}

pub extern "x86-interrupt" fn keyboard(_stack: InterruptStackFrame) {
    devices::keyboard::PC_KEYBOARD.lock().read();

    eoi(1);
}

pub extern "x86-interrupt" fn cascade(_stack: InterruptStackFrame) {
    eoi(2);
}

pub extern "x86-interrupt" fn com2(_stack: InterruptStackFrame) {
    eoi(3);
}

pub extern "x86-interrupt" fn com1(_stack: InterruptStackFrame) {
    eoi(4);
}

pub extern "x86-interrupt" fn lpt2(_stack: InterruptStackFrame) {
    eoi(5);
}

pub extern "x86-interrupt" fn floppy(_stack: InterruptStackFrame) {
    eoi(6);
}

pub extern "x86-interrupt" fn lpt1(_stack: InterruptStackFrame) {
    eoi(7);
}

pub extern "x86-interrupt" fn rtc(_stack: InterruptStackFrame) {
    eoi(8);
}

pub extern "x86-interrupt" fn pci1(_stack: InterruptStackFrame) {
    eoi(9);
}

pub extern "x86-interrupt" fn pci2(_stack: InterruptStackFrame) {
    eoi(10);
}

pub extern "x86-interrupt" fn pci3(_stack: InterruptStackFrame) {
    eoi(11);
}

pub extern "x86-interrupt" fn mouse(_stack: InterruptStackFrame) {
    eoi(12);
}

pub extern "x86-interrupt" fn fpu(_stack: InterruptStackFrame) {
    eoi(13);
}

pub extern "x86-interrupt" fn ata1(_stack: InterruptStackFrame) {
    eoi(14);
}

pub extern "x86-interrupt" fn ata2(_stack: InterruptStackFrame) {
    eoi(15);
}

pub extern "x86-interrupt" fn lapic_timer(_stack: InterruptStackFrame) {
    eoi(16);
}

pub extern "x86-interrupt" fn lapic_error(_stack: InterruptStackFrame) {
    eoi(17);
}

pub extern "x86-interrupt" fn lapic_spurious(_stack: InterruptStackFrame) {
    eoi(18);
}

pub extern "x86-interrupt" fn unimplemented(_stack: InterruptStackFrame) {
    if let Some(lapic) = devices::lapic::LOCAL_APIC.lock().as_mut() {
        unsafe { lapic.end_of_interrupt() };
    }
}

fn eoi(irq: u8) {
    if let Some(lapic) = devices::lapic::LOCAL_APIC.lock().as_mut() {
        unsafe { lapic.end_of_interrupt() };
    } else {
        unsafe {
            devices::pic::PICS
                .lock()
                .notify_end_of_interrupt(irq + 0x20)
        };
    }
}
