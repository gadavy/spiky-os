use spin::Mutex;
use x86_64::structures::paging::mapper::{MapToError, MapperFlush};
use x86_64::structures::paging::{
    FrameAllocator as FrameAllocatorImpl, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
    PhysFrame, Size4KiB, Translate,
};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::frame_allocator::FrameAllocator;

pub struct KernelMapper {
    inner: Option<PageMapper>,
}

impl KernelMapper {
    pub(super) const fn empty() -> Self {
        Self { inner: None }
    }

    pub(super) fn init(
        &mut self,
        phys_offset: VirtAddr,
        page_table: &'static mut PageTable,
        allocator: &'static Mutex<FrameAllocator>,
    ) {
        let mapper = PageMapper::new(phys_offset, page_table, allocator);

        self.inner.replace(mapper);
    }

    pub unsafe fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner.as_mut().unwrap().map(page, flags)
    }

    pub unsafe fn map_phys(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner.as_mut().unwrap().map_phys(page, frame, flags)
    }

    pub unsafe fn identity_map(
        &mut self,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.inner.as_mut().unwrap().identity_map(frame, flags)
    }

    pub fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        self.inner.as_ref().unwrap().translate_addr(addr)
    }
}

pub struct PageMapper {
    table: OffsetPageTable<'static>,
    allocator: &'static Mutex<FrameAllocator>,
}

impl PageMapper {
    pub fn new(
        phys_offset: VirtAddr,
        page_table: &'static mut PageTable,
        allocator: &'static Mutex<FrameAllocator>,
    ) -> Self {
        let table = unsafe { OffsetPageTable::new(page_table, phys_offset) };

        Self { table, allocator }
    }

    pub unsafe fn map(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let mut allocator = self.allocator.lock();

        let frame = allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        self.table.map_to(page, frame, flags, &mut *allocator)
    }

    pub unsafe fn map_phys(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let mut allocator = self.allocator.lock();

        self.table.map_to(page, frame, flags, &mut *allocator)
    }

    pub unsafe fn identity_map(
        &mut self,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let mut allocator = self.allocator.lock();

        self.table.identity_map(frame, flags, &mut *allocator)
    }

    pub fn translate_addr(&self, addr: VirtAddr) -> Option<PhysAddr> {
        self.table.translate_addr(addr)
    }
}
