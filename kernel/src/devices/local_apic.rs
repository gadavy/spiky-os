use core::cell::UnsafeCell;

use x2apic::lapic;
use x86_64::structures::paging::{Page, PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::KERNEL_PAGE_MAPPER;

pub static LOCAL_APIC: LocalApic = LocalApic::empty();

pub struct LocalApic {
    inner: UnsafeCell<Option<lapic::LocalApic>>,
}

impl LocalApic {
    const fn empty() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub(super) fn init(&self, phys_mem_offset: VirtAddr) {
        let apic_phys_addr = PhysAddr::new(unsafe { lapic::xapic_base() });
        let apic_virt_addr = phys_mem_offset + apic_phys_addr.as_u64();

        if !super::cpu::has_x2apic() {
            unsafe { map_memory(apic_phys_addr, apic_virt_addr) };
        }

        let lapic = lapic::LocalApicBuilder::new()
            .timer_vector(48)
            .error_vector(49)
            .spurious_vector(50)
            .set_xapic_base(apic_virt_addr.as_u64())
            .build()
            .unwrap_or_else(|err| panic!("build Local APIC: {}", err));

        unsafe { self.inner.get().replace(Some(lapic)) };

        self.init_ap();
    }

    pub(super) fn init_ap(&self) {
        unsafe {
            if let Some(Some(inner)) = self.inner.get().as_mut() {
                inner.enable();
            }
        }
    }

    /// Signals end-of-interrupt.
    pub fn end_of_interrupt(&self) {
        unsafe {
            if let Some(Some(inner)) = self.inner.get().as_mut() {
                inner.end_of_interrupt();
            }
        }
    }

    /// Sends an INIT IPI to the processors in dest
    pub unsafe fn send_init_ipi(&self, dest: u32) {
        if let Some(Some(inner)) = self.inner.get().as_mut() {
            inner.send_init_ipi(dest);
        }
    }

    /// Sends a start-up IPI to the processors in dest.
    pub unsafe fn send_start_ipi(&self, vector: u8, dest: u32) {
        if let Some(Some(inner)) = self.inner.get().as_mut() {
            inner.send_sipi(vector, dest);
        }
    }
}

unsafe impl Sync for LocalApic {}

unsafe fn map_memory(phys_addr: PhysAddr, virt_addr: VirtAddr) {
    let mut mapper = KERNEL_PAGE_MAPPER.lock();

    let page = Page::containing_address(virt_addr);
    let frame = PhysFrame::containing_address(phys_addr);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    if mapper.translate(virt_addr).is_none() {
        mapper.map_phys(page, frame, flags).unwrap().flush();
    }
}
