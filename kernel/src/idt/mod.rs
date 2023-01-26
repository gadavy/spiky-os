use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

mod exceptions;
mod irq;

pub static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

/// Initializes the interrupt subsystem and sets up an initial IDT.
pub fn init() {
    let mut idt = IDT.lock();
    exceptions::init(&mut idt);

    // Setup interrupts.
    idt[32].set_handler_fn(irq::pit_stack);
    idt[33].set_handler_fn(irq::keyboard);
    idt[34].set_handler_fn(irq::cascade);
    idt[35].set_handler_fn(irq::com2);
    idt[36].set_handler_fn(irq::com1);
    idt[37].set_handler_fn(irq::lpt2);
    idt[38].set_handler_fn(irq::floppy);
    idt[39].set_handler_fn(irq::lpt1);
    idt[40].set_handler_fn(irq::rtc);
    idt[41].set_handler_fn(irq::pci1);
    idt[42].set_handler_fn(irq::pci2);
    idt[43].set_handler_fn(irq::pci3);
    idt[44].set_handler_fn(irq::mouse);
    idt[45].set_handler_fn(irq::fpu);
    idt[46].set_handler_fn(irq::ata1);
    idt[47].set_handler_fn(irq::ata2);
    idt[48].set_handler_fn(irq::lapic_timer);
    idt[49].set_handler_fn(irq::lapic_error);
    idt[50].set_handler_fn(irq::lapic_spurious);

    // Fill all IDT entries with a default unimplemented interrupt handler.
    for entry in idt.slice_mut(32..=255) {
        if entry.handler_addr().as_u64() == 0 {
            entry.set_handler_fn(irq::unimplemented);
        }
    }

    unsafe {
        idt.load_unsafe();
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
    Timer = 32,
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
