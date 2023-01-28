use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

/// A `BumpAllocator` that returns usable frames from the bootloader's memory map.
pub struct BumpAllocator {
    regions: &'static [MemoryRegion],
    next: usize,
}

impl BumpAllocator {
    /// Create a `BumpFrameAllocator` from the passed `MemoryRegions`.
    pub fn init(regions: &'static MemoryRegions) -> Self {
        Self { regions, next: 0 }
    }

    /// Returns an iterator over the usable frames specified in the memory regions.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .map(|r| r.start..r.end)
            .flat_map(|r| r.step_by(Size4KiB::SIZE as usize))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BumpAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

impl FrameDeallocator<Size4KiB> for BumpAllocator {
    unsafe fn deallocate_frame(&mut self, _: PhysFrame) {
        unimplemented!("BumpFrameAllocator::deallocate_frame not implemented")
    }
}
