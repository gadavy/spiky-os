use slab_allocator_rs::LockedHeap;
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::VirtAddr;

use crate::consts::*;
use crate::memory::PAGE_MAPPER;

#[cfg_attr(not(test), global_allocator)]
static HEAP: LockedHeap = LockedHeap::empty();

pub fn init() {
    let heap_start = VirtAddr::new(KERNEL_HEAP_OFFSET);
    let heap_end = heap_start + KERNEL_HEAP_SIZE - 1u64;
    let page_range = Page::range_inclusive(
        Page::containing_address(heap_start),
        Page::containing_address(heap_end),
    );

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for page in page_range {
        unsafe {
            PAGE_MAPPER
                .lock()
                .as_mut()
                .expect("failed to get KernelMapper for mapping heap")
                .map(page, flags)
                .expect("failed to map heap memory")
                .flush();
        }
    }

    // Safety: we map memory for stack addresses.
    unsafe { HEAP.init(heap_start.as_u64() as usize, KERNEL_HEAP_SIZE as usize) };
}
