use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};

use crate::memory::KERNEL_MAPPER;
use x86_64::structures::paging::{Page, PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

pub static mut LOCAL_APIC: Option<LocalApic> = None;

pub fn init(phys_mem_offset: u64) {
    let apic_phys_addr = PhysAddr::new(unsafe { xapic_base() });
    let apic_virt_addr = VirtAddr::new(apic_phys_addr.as_u64()) + phys_mem_offset;

    let apic_frame = PhysFrame::containing_address(apic_phys_addr);
    let apic_page = Page::containing_address(apic_virt_addr);

    // TODO: we need to map memory only for xapic.
    unsafe {
        KERNEL_MAPPER
            .lock()
            .as_mut()
            .expect("expected KernelMapper not to None while initializing LAPIC")
            .unmap(apic_page)
            .map(|f| f.flush());

        KERNEL_MAPPER
            .lock()
            .as_mut()
            .expect("expected KernelMapper not to None while initializing LAPIC")
            .map_phys(
                apic_page,
                apic_frame,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            )
            .expect("failed to map local APIC memory")
            .flush();
    }

    let mut lapic = LocalApicBuilder::new()
        .timer_vector(48)
        .error_vector(49)
        .spurious_vector(50)
        .set_xapic_base(apic_virt_addr.as_u64())
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

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
