use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};
use x86_64::structures::paging::{Page, PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::KERNEL_MAPPER;

pub static mut LOCAL_APIC: Option<LocalApic> = None;

pub fn init(phys_mem_offset: u64) {
    let apic_phys_addr = PhysAddr::new(unsafe { xapic_base() });
    let apic_virt_addr = VirtAddr::new(apic_phys_addr.as_u64()) + phys_mem_offset;

    if !super::cpu::has_x2apic() {
        unsafe { map_memory(apic_phys_addr, apic_virt_addr) };
    }

    let lapic = LocalApicBuilder::new()
        .timer_vector(48)
        .error_vector(49)
        .spurious_vector(50)
        .set_xapic_base(apic_virt_addr.as_u64())
        .build()
        .unwrap_or_else(|err| panic!("build Local APIC: {}", err));

    unsafe { LOCAL_APIC.replace(lapic) };

    init_ap();
}

pub fn init_ap() {
    unsafe {
        LOCAL_APIC
            .as_mut()
            .expect("expected initialized Local APIC")
            .enable()
    };
}

unsafe fn map_memory(phys_addr: PhysAddr, virt_addr: VirtAddr) {
    let mut mapper_guard = KERNEL_MAPPER.lock();
    let mapper = mapper_guard.as_mut().expect("expected initialized mapper");

    let page = Page::containing_address(virt_addr);
    let frame = PhysFrame::containing_address(phys_addr);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    if mapper.translate(virt_addr).is_none() {
        mapper.map_phys(page, frame, flags).unwrap().flush();
    }
}
