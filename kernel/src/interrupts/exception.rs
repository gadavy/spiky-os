use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn divide_error(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: DIVIDE ERROR\n{:#X?}", frame);
}

pub extern "x86-interrupt" fn debug(frame: InterruptStackFrame) {
    log::warn!("\nEXCEPTION: DEBUG EXCEPTION\n{:#X?}", frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn nmi(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: NON-MASKABLE INTERRUPT\n{:#X?}", frame);
}

pub extern "x86-interrupt" fn breakpoint(frame: InterruptStackFrame) {
    log::warn!("\nEXCEPTION: BREAKPOINT\n{:#X?}", frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn overflow(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: OVERFLOW\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn bound_range_exceeded(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: BOUND RANGE EXCEEDED\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn invalid_opcode(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: INVALID OPCODE\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn device_not_available(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: DEVICE NOT AVAILABLE\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, err: u64) -> ! {
    panic!(
        "\nEXCEPTION: DOUBLE FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn invalid_tss(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: INVALID TSS\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn segment_not_present(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: SEGMENT NOT PRESENT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn stack_segment_fault(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: STACK SEGMENT FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn general_protection_fault(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: GENERAL PROTECTION FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, err: PageFaultErrorCode) {
    panic!(
        "\nEXCEPTION: PAGE FAULT while accessing {:#x}\n\
        error code: {:?}\n{:#X?}",
        Cr2::read_raw(),
        err,
        frame
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn x87_floating_point(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: x87 FLOATING POINT\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn alignment_check(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: ALIGNMENT CHECK\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn machine_check(frame: InterruptStackFrame) -> ! {
    panic!("\nEXCEPTION: MACHINE CHECK\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn simd_floating_point(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: SIMD FLOATING POINT\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn virtualization(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: VIRTUALIZATION\n{:#X?}", frame);
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn vmm_communication_exception(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: VMM COMMUNICATION EXCEPTION\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

/// # Panics
///
/// Will panic if an exception is received.
pub extern "x86-interrupt" fn security_exception(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: SECURITY EXCEPTION\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}
