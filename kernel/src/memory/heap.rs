use slab_allocator_rs::LockedHeap;

#[cfg_attr(not(test), global_allocator)]
static HEAP: LockedHeap = LockedHeap::empty();

/// # Safety
///
/// The start address must be valid and the memory in the
/// `[start_addr, start_addr + heap_size)` range must not be used for anything else.
/// This function is unsafe because it can cause undefined behavior if the given address
/// is invalid.
///
/// The provided memory range must be valid for the `'static` lifetime.
pub unsafe fn init(start_addr: usize, size: usize) {
    HEAP.init(start_addr, size);
}
