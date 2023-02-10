use x86_64::registers::control::Cr3;
use x86_64::structures::paging::mapper::MapperFlush;
use x86_64::structures::paging::{
    FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

use crate::memory::frame_allocator::BuddyFrameAllocator;

pub struct PageMapper {
    table: OffsetPageTable<'static>,
    allocator: BuddyFrameAllocator,
}

impl PageMapper {
    pub unsafe fn new(phys_mem_offset: u64, allocator: BuddyFrameAllocator) -> Self {
        let phys_mem_offset = VirtAddr::new(phys_mem_offset);
        let (level_4_table_frame, _) = Cr3::read();

        let phys = level_4_table_frame.start_address();
        let virt = phys_mem_offset + phys.as_u64();
        let page_table_ptr = virt.as_mut_ptr();

        let table = OffsetPageTable::new(&mut *page_table_ptr, phys_mem_offset);

        Self { table, allocator }
    }

    pub unsafe fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Option<MapperFlush<Size4KiB>> {
        let frame = self.allocator.allocate_frame()?;

        self.table
            .map_to(page, frame, flags, &mut self.allocator)
            .ok()
    }

    pub unsafe fn unmap(&mut self, page: Page<Size4KiB>) -> Option<MapperFlush<Size4KiB>> {
        let (frame, flusher) = self.table.unmap(page).ok()?;
        self.allocator.deallocate_frame(frame);

        Some(flusher)
    }
}
