use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::PrivilegeLevel;

use crate::gdt::DOUBLE_FAULT_IST_INDEX;

pub fn init(idt: &mut InterruptDescriptorTable) {
    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
    idt.breakpoint
        .set_handler_fn(breakpoint_handler)
        .set_present(true)
        .set_privilege_level(PrivilegeLevel::Ring3);

    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded
        .set_handler_fn(bound_range_exceeded_handler);
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
    idt.device_not_available
        .set_handler_fn(device_not_available_handler);
    let double_fault = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe { double_fault.set_stack_index(DOUBLE_FAULT_IST_INDEX) };

    // reserved: 0x09 coprocessor segment overrun exception
    idt.invalid_tss.set_handler_fn(invalid_tss_handler);
    idt.segment_not_present
        .set_handler_fn(segment_not_present_handler);
    idt.stack_segment_fault
        .set_handler_fn(stack_segment_fault_handler);
    idt.general_protection_fault
        .set_handler_fn(general_protection_fault_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    // reserved: 0x0F
    idt.x87_floating_point
        .set_handler_fn(x87_floating_point_handler);
    idt.alignment_check.set_handler_fn(alignment_check_handler);
    idt.machine_check.set_handler_fn(machine_check_handler);
    idt.simd_floating_point
        .set_handler_fn(simd_floating_point_handler);
    idt.virtualization.set_handler_fn(virtualization_handler);
    // reserved: 0x15 - 0x1C
    idt.vmm_communication_exception
        .set_handler_fn(vmm_communication_exception_handler);
    idt.security_exception
        .set_handler_fn(security_exception_handler);
}

extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: DIVIDE ERROR\n{:#X?}", frame);
}

extern "x86-interrupt" fn debug_handler(frame: InterruptStackFrame) {
    log::warn!("\nEXCEPTION: DEBUG EXCEPTION\n{:#X?}", frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

extern "x86-interrupt" fn nmi_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: NON-MASKABLE INTERRUPT\n{:#X?}", frame);
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    log::warn!("\nEXCEPTION: BREAKPOINT\n{:#X?}", frame);
    // don't halt here, this isn't a fatal/permanent failure, just a brief pause.
}

extern "x86-interrupt" fn overflow_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: OVERFLOW\n{:#X?}", frame);
}

extern "x86-interrupt" fn bound_range_exceeded_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: BOUND RANGE EXCEEDED\n{:#X?}", frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: INVALID OPCODE\n{:#X?}", frame);
}

extern "x86-interrupt" fn device_not_available_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: DEVICE NOT AVAILABLE\n{:#X?}", frame);
}

pub extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, err: u64) -> ! {
    panic!(
        "\nEXCEPTION: DOUBLE FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn invalid_tss_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: INVALID TSS\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn segment_not_present_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: SEGMENT NOT PRESENT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn stack_segment_fault_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: STACK SEGMENT FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: GENERAL PROTECTION FAULT\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, err: PageFaultErrorCode) {
    panic!(
        "\nEXCEPTION: PAGE FAULT while accessing {:#x}\n\
        error code: {:?}\n{:#X?}",
        Cr2::read_raw(),
        err,
        frame
    );
}

extern "x86-interrupt" fn x87_floating_point_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: x87 FLOATING POINT\n{:#X?}", frame);
}

extern "x86-interrupt" fn alignment_check_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: ALIGNMENT CHECK\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn machine_check_handler(frame: InterruptStackFrame) -> ! {
    panic!("\nEXCEPTION: MACHINE CHECK\n{:#X?}", frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: SIMD FLOATING POINT\n{:#X?}", frame);
}

extern "x86-interrupt" fn virtualization_handler(frame: InterruptStackFrame) {
    panic!("\nEXCEPTION: VIRTUALIZATION\n{:#X?}", frame);
}

extern "x86-interrupt" fn vmm_communication_exception_handler(
    frame: InterruptStackFrame,
    err: u64,
) {
    panic!(
        "\nEXCEPTION: VMM COMMUNICATION EXCEPTION\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}

extern "x86-interrupt" fn security_exception_handler(frame: InterruptStackFrame, err: u64) {
    panic!(
        "\nEXCEPTION: SECURITY EXCEPTION\n{:#X?}\nError code: {:#b}",
        frame, err
    );
}
