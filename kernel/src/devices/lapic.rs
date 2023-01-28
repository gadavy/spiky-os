use spin::Mutex;
use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};
use x86_64::structures::paging::mapper::TranslateResult;
use x86_64::structures::paging::PageTableFlags;
use x86_64::{PhysAddr, VirtAddr};

pub static LOCAL_APIC: Mutex<Option<LocalApic>> = Mutex::new(None);

pub fn init(phys_mem_offset: u64) {
    super::pic::disable();
    create_lapic(phys_mem_offset);

    unsafe { LOCAL_APIC.lock().as_mut().unwrap().enable() }
}

fn create_lapic(phys_mem_offset: u64) {
    let apic_phys_addr = PhysAddr::new(unsafe { xapic_base() });
    let apic_virt_addr = VirtAddr::new(apic_phys_addr.as_u64()) + phys_mem_offset;

    let mut binding = crate::memory::MEMORY_MAPPER.lock();
    let Some(mapper) = binding.as_mut() else { return };

    if let Ok((_, flusher)) = mapper.unmap_phys(apic_virt_addr) {
        flusher.flush();
    }

    mapper
        .map_phys(
            apic_virt_addr,
            apic_phys_addr,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        )
        .expect("failed to map local APIC memory")
        .flush();

    let lapic = LocalApicBuilder::new()
        .timer_vector(48)
        .error_vector(49)
        .spurious_vector(50)
        .set_xapic_base(apic_virt_addr.as_u64())
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    LOCAL_APIC.lock().replace(lapic);
}
