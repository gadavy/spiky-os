mod heap;

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};

pub fn init(phys_mem_offset: u64, memory_regions: &'static MemoryRegions) {
    let mem_size: u64 = memory_regions.iter().map(|r| r.end - r.start).sum();
    log::debug!("Available memory {} MB", mem_size >> 20);

    let mut mapper = unsafe { new_mapper(phys_mem_offset) };
    let mut frame_allocator = BootInfoFrameAllocator::init(memory_regions);

    heap::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    log::debug!("Heap initialized");
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

/// A `FrameAllocator` that returns usable frames from the bootloader's memory map.
struct BootInfoFrameAllocator {
    regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a `KernelFrameAllocator` from the passed `MemoryRegions`.
    fn init(regions: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator { regions, next: 0 }
    }

    /// Returns an iterator over the usable frames specified in the memory regions.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .map(|r| r.start..r.end)
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
