use x86_64::registers::control::Cr3;
use x86_64::structures::paging::mapper::{MapToError, MapperFlush, TranslateResult};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB, Translate,
};
use x86_64::{PhysAddr, VirtAddr};

use super::BuddyFrameAllocator;

pub struct KernelMapper {
    inner: Option<InnerKernelMapper>,
}

impl KernelMapper {
    pub(super) const fn empty() -> Self {
        Self { inner: None }
    }

    pub(super) unsafe fn init(&mut self, phys_mem_offset: u64, allocator: BuddyFrameAllocator) {
        let phys_mem_offset = VirtAddr::new(phys_mem_offset);
        let (level_4_table_frame, _) = Cr3::read();

        let phys = level_4_table_frame.start_address();
        let virt = phys_mem_offset + phys.as_u64();
        let page_table_ptr = virt.as_mut_ptr();

        let table = OffsetPageTable::new(&mut *page_table_ptr, phys_mem_offset);

        self.inner.replace(InnerKernelMapper { table, allocator });
    }

    pub unsafe fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner_as_mut().map(page, flags)
    }

    pub unsafe fn map_phys(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner_as_mut().map_phys(page, frame, flags)
    }

    pub unsafe fn identity_map(
        &mut self,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner_as_mut().identity_map(frame, flags)
    }

    pub fn allocate_frames_range(&mut self, length: u64) -> Option<PhysFrame> {
        self.inner_as_mut().allocate_frames_range(length)
    }

    pub fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        self.inner.as_ref().unwrap().translate(addr)
    }

    #[inline]
    fn inner_as_mut(&mut self) -> &mut InnerKernelMapper {
        self.inner.as_mut().unwrap()
    }
}

struct InnerKernelMapper {
    table: OffsetPageTable<'static>,
    allocator: BuddyFrameAllocator,
}

impl InnerKernelMapper {
    unsafe fn map(
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

    unsafe fn map_phys(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.table.map_to(page, frame, flags, &mut self.allocator)
    }

    unsafe fn identity_map(
        &mut self,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.table.identity_map(frame, flags, &mut self.allocator)
    }

    fn allocate_frames_range(&mut self, length: u64) -> Option<PhysFrame> {
        self.allocator.allocate_frames_range(length)
    }

    fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        match self.table.translate(addr) {
            TranslateResult::Mapped { frame, .. } => Some(frame.start_address()),
            _ => None,
        }
    }
}
