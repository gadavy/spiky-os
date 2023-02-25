use bootloader_api::info::MemoryRegions;
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

use frame_allocator::FrameAllocator;
use mapper::KernelMapper;

mod frame_allocator;
mod heap;
mod mapper;

pub static KERNEL_PAGE_MAPPER: Mutex<KernelMapper> = Mutex::new(KernelMapper::empty());
pub static KERNEL_FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::empty());

pub fn init(phys_offset: u64, regions: &'static MemoryRegions) {
    log::trace!("Init KernelMapper and FrameAllocator");

    let phys_offset = VirtAddr::new(phys_offset);
    let page_table = unsafe { active_level_4_table(phys_offset) };

    KERNEL_FRAME_ALLOCATOR.lock().init(phys_offset, regions);
    KERNEL_PAGE_MAPPER
        .lock()
        .init(phys_offset, page_table, &KERNEL_FRAME_ALLOCATOR);
}

pub fn init_heap() {
    heap::init();
}

unsafe fn active_level_4_table(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_offset + phys.as_u64();
    let page_table_ptr = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
