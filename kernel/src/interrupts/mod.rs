use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init(gdt_stack_index: u16) {
    let mut idt = IDT.lock();

    idt.breakpoint.set_handler_fn(breakpoint_handler);

    let double_fault = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe { double_fault.set_stack_index(gdt_stack_index) };

    unsafe {
        idt.load_unsafe();
        PICS.lock().initialize();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT\n{:#X?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _code: u64) -> ! {
    panic!("\nEXCEPTION: DOUBLE FAULT\n{:#X?}", stack_frame);
}
