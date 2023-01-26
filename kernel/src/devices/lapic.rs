use spin::Mutex;
use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};
use x86_64::structures::paging::PageTableFlags;
use x86_64::{PhysAddr, VirtAddr};

pub static LOCAL_APIC: Mutex<Option<LocalApic>> = Mutex::new(None);

pub fn init(phys_mem_offset: u64) {
    create_lapic(phys_mem_offset);

    unsafe { LOCAL_APIC.lock().as_mut().unwrap().enable() }

    super::pic::disable();
}

fn create_lapic(phys_mem_offset: u64) {
    let apic_phys_addr = unsafe { xapic_base() };
    let apic_virt_addr = apic_phys_addr + phys_mem_offset;

    // TODO: need fix memory mapping.
    //  * maybe send mapper by link
    //  * or map memory in main function
    crate::memory::MEMORY_MAPPER
        .lock()
        .as_mut()
        .unwrap()
        .map_phys(
            VirtAddr::new(apic_virt_addr),
            PhysAddr::new(apic_phys_addr),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        )
        .expect("failed to map local APIC memory")
        .flush();

    let lapic = LocalApicBuilder::new()
        .timer_vector(48)
        .error_vector(49)
        .spurious_vector(50)
        .set_xapic_base(apic_virt_addr)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    LOCAL_APIC.lock().replace(lapic);
}
