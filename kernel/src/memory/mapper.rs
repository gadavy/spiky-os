use x86_64::registers::control::Cr3;
use x86_64::structures::paging::mapper::{MapToError, MapperFlush, TranslateResult, UnmapError};
use x86_64::structures::paging::{
    FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame,
    Size4KiB, Translate,
};
use x86_64::{PhysAddr, VirtAddr};

use super::BuddyFrameAllocator;

pub struct KernelMapper {
    table: OffsetPageTable<'static>,
    allocator: BuddyFrameAllocator,
}

impl KernelMapper {
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
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let frame = self
            .allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        self.table.map_to(page, frame, flags, &mut self.allocator)
    }

    pub unsafe fn map_phys(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.table.map_to(page, frame, flags, &mut self.allocator)
    }

    pub unsafe fn unmap(
        &mut self,
        page: Page<Size4KiB>,
    ) -> Result<MapperFlush<Size4KiB>, UnmapError> {
        let (frame, flusher) = self.table.unmap(page)?;
        self.allocator.deallocate_frame(frame);

        Ok(flusher)
    }

    pub fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        match self.table.translate(addr) {
            TranslateResult::Mapped { frame, .. } => Some(frame.start_address()),
            TranslateResult::NotMapped => None,
            TranslateResult::InvalidFrameAddress(addr) => panic!("invalid frame addr: {addr:?}"),
        }
    }
}
