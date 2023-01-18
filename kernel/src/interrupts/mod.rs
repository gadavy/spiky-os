use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

mod exceptions;

const PIC_1_OFFSET: u8 = 0x20;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    let mut idt = IDT.lock();
    exceptions::init(&mut idt);

    // Setup interrupts.
    idt[InterruptsVector::Timer.into()].set_handler_fn(timer_interrupt_handler);

    unsafe {
        idt.load_unsafe();
        PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InterruptsVector {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl From<InterruptsVector> for u8 {
    fn from(value: InterruptsVector) -> Self {
        value as u8
    }
}

impl From<InterruptsVector> for usize {
    fn from(value: InterruptsVector) -> Self {
        usize::from(value as u8)
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut pics = PICS.lock();
    unsafe { pics.notify_end_of_interrupt(InterruptsVector::Timer.into()) }
}
