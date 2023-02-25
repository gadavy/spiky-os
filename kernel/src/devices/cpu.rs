use core::sync::atomic::{AtomicBool, Ordering};

use acpi::platform::{Processor, ProcessorState};
use raw_cpuid::CpuId;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::ap_entry;
use crate::devices::local_apic;
use crate::memory::{KERNEL_FRAME_ALLOCATOR, KERNEL_PAGE_MAPPER};
use crate::prelude::*;

static AP_READY: AtomicBool = AtomicBool::new(false);

pub fn set_ap_is_ready() {
    AP_READY.store(true, Ordering::SeqCst);
}

pub(super) fn has_x2apic() -> bool {
    let cpuid = CpuId::new();

    match cpuid.get_feature_info() {
        Some(finfo) => finfo.has_x2apic(),
        None => false,
    }
}

pub(super) fn init_ap_cores(phys_mem_offset: VirtAddr, ap_processors: &[Processor]) {
    unsafe { allocate_trampoline(VirtAddr::new(TRAMPOLINE)) };

    let (page_table, _) = Cr3::read();

    for ap in ap_processors {
        if !ap.is_ap || ap.state == ProcessorState::Disabled {
            continue;
        }

        unsafe { start_core(ap, phys_mem_offset, page_table) }
    }
}

fn allocate_ap_stack(phys_mem_offset: VirtAddr, pages_count: u64) -> Option<VirtAddr> {
    let start_frame = KERNEL_FRAME_ALLOCATOR
        .lock()
        .allocate_frames_range(pages_count)?;

    let stack_start = phys_mem_offset + start_frame.start_address().as_u64();
    let stack_end = stack_start + pages_count * PAGE_SIZE;

    Some(stack_end)
}

unsafe fn allocate_trampoline(virt_addr: VirtAddr) {
    let phys_addr = PhysAddr::new(virt_addr.as_u64());

    let frame = PhysFrame::containing_address(phys_addr);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let mut mapper = KERNEL_PAGE_MAPPER.lock();

    assert!(mapper.translate(virt_addr).is_none());

    mapper
        .identity_map(frame, flags)
        .expect("failed to map trampoline")
        .flush();

    // Write trampoline, make sure TRAMPOLINE page is free for use
    for (i, b) in TRAMPOLINE_DATA.iter().enumerate() {
        (TRAMPOLINE as *mut u8).add(i).write_volatile(*b);
    }
}

unsafe fn start_core(ap: &Processor, phys_mem_offset: VirtAddr, page_table: PhysFrame) {
    log::trace!("Ap {ap:?}");

    let stack_end =
        allocate_ap_stack(phys_mem_offset, 64).expect("no more frames in acpi stack_start");

    let ap_ready = (TRAMPOLINE + 8) as *mut u64;
    let ap_cpu_id = ap_ready.offset(1);
    let ap_page_table = ap_ready.offset(2);
    let ap_stack_end = ap_ready.offset(3);
    let ap_code = ap_ready.offset(4);

    ap_ready.write_volatile(0);
    ap_cpu_id.write_volatile(u64::from(ap.local_apic_id));
    ap_page_table.write_volatile(page_table.start_address().as_u64());
    ap_stack_end.write_volatile(stack_end.as_u64());

    #[allow(clippy::fn_to_numeric_cast)]
    ap_code.write_volatile(ap_entry as u64);

    AP_READY.store(false, Ordering::SeqCst);

    let local_apic_id = ap.local_apic_id << 24;

    log::trace!(">> Send init ipi");
    local_apic::LOCAL_APIC.send_init_ipi(local_apic_id);

    log::trace!(">> Send start ipi");
    let ap_segment = (TRAMPOLINE >> 12) & 0xFF;
    local_apic::LOCAL_APIC.send_start_ipi(ap_segment as u8, local_apic_id);

    log::trace!(">> Wait for trampoline ready");
    while unsafe { ap_ready.read_volatile() } == 0 {
        core::hint::spin_loop();
    }

    log::trace!(">> Wait rust code ready");
    while !AP_READY.load(Ordering::SeqCst) {
        core::hint::spin_loop();
    }
}
