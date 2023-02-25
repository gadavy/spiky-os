use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::{PrivilegeLevel, VirtAddr};

use crate::memory::KERNEL_FRAME_ALLOCATOR;

use crate::interrupts::{exception, irq};
use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Early idt
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut EARLY_IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_early() {
    log::trace!("init early IDT");

    unsafe {
        EARLY_IDT
            .double_fault
            .set_handler_fn(exception::double_fault);
        EARLY_IDT
            .segment_not_present
            .set_handler_fn(exception::segment_not_present);
        EARLY_IDT
            .general_protection_fault
            .set_handler_fn(exception::general_protection_fault);
        EARLY_IDT.page_fault.set_handler_fn(exception::page_fault);
        EARLY_IDT
            .alignment_check
            .set_handler_fn(exception::alignment_check);

        EARLY_IDT.load();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Global thread locals instances
////////////////////////////////////////////////////////////////////////////////////////////////////

#[thread_local]
static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// Initializes a BSP IDT.
pub fn init_bsp(phys_offset: u64) {
    log::trace!("Init BSP IDT");

    unsafe { init_generic(true, VirtAddr::new(phys_offset), &mut IDT) };
}

/// Initializes a BSP IDT.
pub fn init_ap(phys_offset: u64) {
    log::trace!("Init AP IDT");

    unsafe { init_generic(false, VirtAddr::new(phys_offset), &mut IDT) };
}

unsafe fn init_generic(is_bsp: bool, phys_offset: VirtAddr, idt: &mut InterruptDescriptorTable) {
    // Allocate 64 KiB of stack space for the backup stack.
    let page_count = KERNEL_BACKUP_STACK_SIZE / PAGE_SIZE;

    let frame = KERNEL_FRAME_ALLOCATOR
        .lock()
        .allocate_frames_range(page_count)
        .expect("failed to allocate pages for backup interrupt stack");

    let stack_start = phys_offset + frame.start_address().as_u64();
    let stack_end = stack_start + KERNEL_BACKUP_STACK_SIZE;

    super::gdt::TSS.interrupt_stack_table[usize::from(KERNEL_BACKUP_STACK_INDEX)] = stack_end;

    // Set up exceptions
    idt.divide_error.set_handler_fn(exception::divide_error);
    idt.debug.set_handler_fn(exception::debug);
    idt.non_maskable_interrupt
        .set_handler_fn(exception::nmi)
        .set_stack_index(KERNEL_BACKUP_STACK_INDEX);
    idt.breakpoint
        .set_handler_fn(exception::breakpoint)
        .set_present(true)
        .set_privilege_level(PrivilegeLevel::Ring3);

    idt.overflow.set_handler_fn(exception::overflow);
    idt.bound_range_exceeded
        .set_handler_fn(exception::bound_range_exceeded);
    idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
    idt.device_not_available
        .set_handler_fn(exception::device_not_available);
    idt.double_fault
        .set_handler_fn(exception::double_fault)
        .set_stack_index(KERNEL_BACKUP_STACK_INDEX);

    idt.invalid_tss.set_handler_fn(exception::invalid_tss);
    idt.segment_not_present
        .set_handler_fn(exception::segment_not_present);
    idt.stack_segment_fault
        .set_handler_fn(exception::stack_segment_fault);
    idt.general_protection_fault
        .set_handler_fn(exception::general_protection_fault);
    idt.page_fault.set_handler_fn(exception::page_fault);
    idt.x87_floating_point
        .set_handler_fn(exception::x87_floating_point);
    idt.alignment_check
        .set_handler_fn(exception::alignment_check);
    idt.machine_check
        .set_handler_fn(exception::machine_check)
        .set_stack_index(KERNEL_BACKUP_STACK_INDEX);
    idt.simd_floating_point
        .set_handler_fn(exception::simd_floating_point);
    idt.virtualization.set_handler_fn(exception::virtualization);
    idt.vmm_communication_exception
        .set_handler_fn(exception::vmm_communication_exception);
    idt.security_exception
        .set_handler_fn(exception::security_exception);

    if is_bsp {
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
    } else {
        idt[49].set_handler_fn(irq::lapic_error);
    }

    // Fill empty IDT entries with a default unimplemented interrupt handler.
    for entry in idt.slice_mut(32..=255) {
        if entry.handler_addr().as_u64() == 0 {
            entry.set_handler_fn(irq::unimplemented);
        }
    }

    unsafe { IDT.load() }
}
