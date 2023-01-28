use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::mapper::{MapToError, MapperFlush};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

use bump::BumpAllocator;

use super::consts::*;

// mod buddy;
mod bump;

pub static MEMORY_MANAGER: Mutex<MemoryManager> = Mutex::new(MemoryManager::empty());

pub fn init(phys_mem_offset: u64, regions: &'static MemoryRegions) {
    let mem_size: u64 = regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .map(|r| r.end - r.start)
        .sum();

    log::debug!("Available memory {} MB", mem_size >> 20);

    let mapper = unsafe { new_mapper(phys_mem_offset) };
    let bump_allocator = BumpAllocator::init(regions);

    MEMORY_MANAGER.lock().init(mapper, bump_allocator);
}

pub fn init_heap() {
    let heap_start = VirtAddr::new(KERNEL_HEAP_OFFSET);
    let heap_end = heap_start + KERNEL_HEAP_SIZE - 1u64;
    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);
    let page_range = Page::range_inclusive(heap_start_page, heap_end_page);

    let mut manager = MEMORY_MANAGER.lock();
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for page in page_range {
        manager
            .map(page, flags)
            .expect("failed to map kernel heap")
            .flush();
    }

    // Safety: we map memory for stack addresses.
    unsafe { crate::heap::init(heap_start.as_u64() as usize, KERNEL_HEAP_SIZE as usize) };
}

pub struct MemoryManager {
    mapper: Option<OffsetPageTable<'static>>,
    allocator: Option<BumpAllocator>,
}

impl MemoryManager {
    const fn empty() -> Self {
        Self {
            mapper: None,
            allocator: None,
        }
    }

    fn init(&mut self, mapper: OffsetPageTable<'static>, bump_allocator: BumpAllocator) {
        self.mapper.replace(mapper);
        self.allocator.replace(bump_allocator);
    }

    pub fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let mapper = self
            .mapper
            .as_mut()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let allocator = self
            .allocator
            .as_mut()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let frame = allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        Ok(unsafe { mapper.map_to(page, frame, flags, allocator)? })
    }

    /// Allocates the required number of frames.
    ///
    /// TODO: maybe it's better to use size instead of count?
    pub fn allocate_frames(
        &mut self,
        count: usize,
    ) -> Result<PhysFrame<Size4KiB>, MapToError<Size4KiB>> {
        let allocator = self
            .allocator
            .as_mut()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let first = allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        for _ in 0..count {
            allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
        }

        Ok(first)
    }

    pub fn page_size(&self) -> usize {
        4096
    }

    pub fn phys_to_virt(&self, phys_addr: PhysAddr) -> VirtAddr {
        self.mapper.as_ref().unwrap().phys_offset() + phys_addr.as_u64()
    }
}

unsafe fn new_mapper(phys_mem_offset: u64) -> OffsetPageTable<'static> {
    let phys_mem_offset = VirtAddr::new(phys_mem_offset);
    let level_4_table = active_level_4_table(phys_mem_offset);
    OffsetPageTable::new(level_4_table, phys_mem_offset)
}

unsafe fn active_level_4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
