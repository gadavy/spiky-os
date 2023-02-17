use bootloader_api::info::MemoryRegions;
use spin::Mutex;

use frame_allocator::BuddyFrameAllocator;
use mapper::KernelMapper;

mod frame_allocator;
mod heap;
mod mapper;

pub static KERNEL_MAPPER: Mutex<KernelMapper> = Mutex::new(KernelMapper::empty());

pub fn init(phys_mem_offset: u64, regions: &'static MemoryRegions) {
    log::trace!("Init FrameAllocator");

    let allocator = BuddyFrameAllocator::new(phys_mem_offset, regions);

    log::info!("Available memory {} MB", allocator.free_pages() >> 8);

    unsafe { KERNEL_MAPPER.lock().init(phys_mem_offset, allocator) };
}

pub fn init_heap() {
    heap::init();
}
