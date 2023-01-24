use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

mod exceptions;

const PIC_1_OFFSET: u8 = 0x20;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Initializes the interrupt subsystem and sets up an initial IDT.
pub fn init() {
    let mut idt = IDT.lock();
    exceptions::init(&mut idt);

    // Fill all IDT entries with a default unimplemented interrupt handler.
    for entry in idt.slice_mut(32..=255) {
        entry.set_handler_fn(unimplemented_interrupt_handler);
    }

    // Setup interrupts.
    idt[InterruptsVector::Timer.into()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptsVector::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);

    unsafe {
        idt.load_unsafe();
        PICS.lock().initialize();
    };

    log::debug!("IDT initialized");
}

/// Enable interrupts.
pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

/// Run a closure with disabled interrupts.
pub fn without_interrupts<F: Fn() -> R, R>(f: F) -> R {
    x86_64::instructions::interrupts::without_interrupts(f)
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

impl From<InterruptsVector> for Option<u8> {
    fn from(value: InterruptsVector) -> Self {
        Some(value as u8)
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    eoi(InterruptsVector::Timer.into());
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    crate::devices::keyboard::PC_KEYBOARD.lock().read();

    eoi(InterruptsVector::Keyboard.into());
}

extern "x86-interrupt" fn unimplemented_interrupt_handler(_stack_frame: InterruptStackFrame) {
    eoi(None);
}

fn eoi(interrupt_id: Option<u8>) {
    let Some(id) = interrupt_id else { return };

    let mut pics = PICS.lock();
    unsafe { pics.notify_end_of_interrupt(id) }
}
