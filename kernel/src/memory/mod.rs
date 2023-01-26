use bootloader_api::info::MemoryRegions;
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::mapper::{MapToError, MapperFlush};
use x86_64::structures::paging::{
    Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

use bump::BumpFrameAllocator;

mod bump;
mod heap;

pub static MEMORY_MAPPER: Mutex<Option<MemoryMapper>> = Mutex::new(None);

pub fn init(phys_mem_offset: u64, memory_regions: &'static MemoryRegions) {
    let mem_size: u64 = memory_regions.iter().map(|r| r.end - r.start).sum();
    log::debug!("Available memory {} MB", mem_size >> 20);

    let mut mapper = unsafe { new_mapper(phys_mem_offset) };
    let mut frame_allocator = BumpFrameAllocator::init(memory_regions);

    heap::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    MEMORY_MAPPER
        .lock()
        .replace(MemoryMapper::new(mapper, frame_allocator));

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

pub struct MemoryMapper {
    mapper: OffsetPageTable<'static>,
    frame_allocator: BumpFrameAllocator,
}

impl MemoryMapper {
    fn new(mapper: OffsetPageTable<'static>, frame_allocator: BumpFrameAllocator) -> Self {
        Self {
            mapper,
            frame_allocator,
        }
    }

    pub fn map_phys(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let page = Page::containing_address(virt);
        let frame = PhysFrame::containing_address(phys);

        unsafe {
            self.mapper
                .map_to(page, frame, flags, &mut self.frame_allocator)
        }
    }
}
