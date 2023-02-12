use core::ops::Range;

use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};

use crate::consts::*;

pub struct BuddyFrameAllocator {
    entries: &'static mut [BuddyEntry],
}

impl BuddyFrameAllocator {
    pub fn new(phys_mem_offset: u64, regions: &[MemoryRegion]) -> Self {
        let phys_mem_offset = VirtAddr::new(phys_mem_offset);

        // First we need to find a MemoryRegion for buddy table.
        let mut table_addr = VirtAddr::zero();
        let mut region_idx = 0;
        let mut region_offset = 0;

        for (idx, region) in Self::usable_regions(regions).enumerate() {
            if region.end - region.start >= PAGE_SIZE {
                table_addr = phys_mem_offset + region.start;
                region_idx = idx;
                region_offset = PAGE_SIZE;

                break;
            };
        }

        assert!(
            !table_addr.is_null(),
            "MemoryRegion with size {PAGE_SIZE} not found",
        );

        // Allocate buddy table and zero it.
        let entries = unsafe {
            core::slice::from_raw_parts_mut(
                table_addr.as_mut_ptr::<BuddyEntry>(),
                (PAGE_SIZE / BuddyEntry::SIZE) as usize,
            )
        };

        for entry in entries.iter_mut() {
            *entry = BuddyEntry::empty();
        }

        // Add regions to buddy table combining areas when possible.
        for (idx, region) in Self::usable_regions(regions).enumerate() {
            let mut region_start = PhysAddr::new(region.start);
            let mut region_size = region.end - region.start;

            if idx == region_idx {
                if region_offset == region_size {
                    continue;
                } else {
                    region_start += region_offset;
                    region_size -= region_offset;
                }
            }

            for entry in entries.iter_mut() {
                if region_start + region_size == entry.start_phys {
                    entry.start_phys = region_start;
                    entry.start_virt = phys_mem_offset + region_start.as_u64();
                    entry.size += region_size;

                    break;
                } else if region_start == entry.start_phys + entry.size {
                    entry.size += region_size;

                    break;
                } else if entry.size == 0 {
                    entry.start_phys = region_start;
                    entry.start_virt = phys_mem_offset + region_start.as_u64();
                    entry.size = region_size;

                    break;
                };
            }
        }

        // Allocate buddy maps.
        for entry in entries.iter_mut() {
            for page in 0..entry.total_pages() {
                entry.set_page_usage(page, page < entry.usage_pages());
            }

            entry.used = entry.usage_pages();
            entry.skip = entry.usage_pages();
        }

        Self { entries }
    }

    pub fn allocate_frames_range(&mut self, length: u64) -> Option<PhysFrame> {
        self.entries
            .iter_mut()
            .find_map(|e| e.allocate_range(length))
            .map(PhysFrame::containing_address)
    }

    pub unsafe fn deallocate_frames_range(&mut self, frame: PhysFrame, length: u64) {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.contains_addr(frame.start_address()))
        {
            e.deallocate_range(frame.start_address(), length);
        }
    }

    pub fn free_pages(&self) -> u64 {
        self.entries.iter().map(|e| e.total_pages() - e.used).sum()
    }

    fn usable_regions(regions: &[MemoryRegion]) -> impl Iterator<Item = &MemoryRegion> {
        regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
    }
}

unsafe impl FrameAllocator<Size4KiB> for BuddyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.allocate_frames_range(1)
    }
}

impl FrameDeallocator<Size4KiB> for BuddyFrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame) {
        self.deallocate_frames_range(frame, 1);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Buddy entry
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
struct BuddyEntry {
    start_phys: PhysAddr,
    start_virt: VirtAddr,
    size: u64,
    skip: u64,
    used: u64,
}

impl BuddyEntry {
    const SIZE: u64 = core::mem::size_of::<Self>() as u64;

    fn empty() -> Self {
        Self {
            start_phys: PhysAddr::zero(),
            start_virt: VirtAddr::zero(),
            size: 0,
            skip: 0,
            used: 0,
        }
    }

    #[inline]
    fn total_pages(&self) -> u64 {
        self.size >> PAGE_SHIFT
    }

    #[inline]
    fn usage_pages(&self) -> u64 {
        (self.total_pages() + PAGE_OFFSET_MASK) >> PAGE_SHIFT
    }

    fn contains_addr(&self, addr: PhysAddr) -> bool {
        self.start_phys <= addr && addr <= self.start_phys + self.size
    }

    fn allocate_range(&mut self, length: u64) -> Option<PhysAddr> {
        if self.size - self.used < length {
            return None;
        }

        let page_range = self.find_free_range(length)?;

        for page in page_range.clone() {
            self.set_page_usage(page, true);

            let offset = page << PAGE_SHIFT;
            let virt_addr = self.start_virt + offset;

            // Zero page.
            unsafe { core::ptr::write_bytes(virt_addr.as_mut_ptr::<u8>(), 0, PAGE_SIZE as usize) };
        }

        if self.skip == page_range.start {
            self.skip += length;
        }

        self.used += length;

        let offset = page_range.start << PAGE_SHIFT;
        let phys_addr = self.start_phys + offset;

        Some(phys_addr)
    }

    fn deallocate_range(&mut self, addr: PhysAddr, length: u64) {
        let start_page = (addr - self.start_phys) >> PAGE_SHIFT;

        for page in start_page..start_page + length {
            assert!(
                self.page_is_used(start_page),
                "tried to free already free page"
            );

            self.set_page_usage(start_page, false);

            if page < self.skip {
                self.skip = page;
            }

            self.used -= 1;
        }
    }

    fn find_free_range(&self, length: u64) -> Option<Range<u64>> {
        let mut free_page = 0;
        let mut free_count = 0;

        for page in self.skip..self.total_pages() {
            if self.page_is_used(page) {
                free_count = 0;
                continue;
            }

            if free_count == 0 {
                free_page = page;
            }

            free_count += 1;

            if free_count == length {
                return Some(free_page..free_page + free_count);
            }
        }

        None
    }

    fn page_is_used(&self, index: u64) -> bool {
        use bit::BitIndex;

        let byte_index = index / 8;
        let bit_index = (index % 8) as usize;

        let addr = self.start_virt + byte_index;

        unsafe { addr.as_ptr::<u8>().read().bit(bit_index) }
    }

    fn set_page_usage(&self, page_index: u64, is_used: bool) {
        use bit::BitIndex;

        let byte_index = page_index / 8;
        let bit_index = (page_index % 8) as usize;

        let addr = self.start_virt + byte_index;

        unsafe {
            let mut value = addr.as_mut_ptr::<u8>().read();
            value.set_bit(bit_index, is_used);
            addr.as_mut_ptr::<u8>().write(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(align(4096))]
    struct TestMemoryArea {
        space: [u8; 4096 * 5],
    }

    impl TestMemoryArea {
        fn new() -> Self {
            Self {
                space: [0; 4096 * 5],
            }
        }

        fn start_virt(&self) -> VirtAddr {
            VirtAddr::from_ptr(&self.space as *const _)
        }

        fn start_phys(&self) -> PhysAddr {
            PhysAddr::new(self.start_virt().as_u64())
        }

        fn len(&self) -> usize {
            self.space.len()
        }

        fn region(&self) -> MemoryRegion {
            MemoryRegion {
                start: self.start_virt().as_u64(),
                end: self.start_virt().as_u64() + self.len() as u64,
                kind: MemoryRegionKind::Usable,
            }
        }

        fn page_usage_area(&self, root: bool) -> &[u8] {
            let addr = self.start_virt() + if root { 4096u64 } else { 0u64 };
            unsafe { core::slice::from_raw_parts(addr.as_ptr::<u8>(), 4096) }
        }
    }

    #[test]
    fn one_memory_region() {
        let mem_area = TestMemoryArea::new();
        let allocator = BuddyFrameAllocator::new(0, &[mem_area.region()]);

        assert_eq!(
            allocator.entries[0],
            BuddyEntry {
                start_phys: mem_area.start_phys() + 4096u64,
                start_virt: mem_area.start_virt() + 4096u64,
                size: 4096 * 4,
                skip: 1,
                used: 1,
            }
        );

        assert_eq!(allocator.entries[1], BuddyEntry::empty())
    }

    #[test]
    fn several_memory_regions() {
        let mem_area1 = TestMemoryArea::new();
        let _ = TestMemoryArea::new(); // unused area
        let mem_area2 = TestMemoryArea::new();
        let allocator = BuddyFrameAllocator::new(0, &[mem_area1.region(), mem_area2.region()]);

        assert_eq!(
            allocator.entries[0..2],
            [
                BuddyEntry {
                    start_phys: mem_area1.start_phys() + 4096u64, // sub buddy entries table size.
                    start_virt: mem_area1.start_virt() + 4096u64, // sub buddy entries table size.
                    size: 4096 * 4,
                    skip: 1,
                    used: 1,
                },
                BuddyEntry {
                    start_phys: mem_area2.start_phys(),
                    start_virt: mem_area2.start_virt(),
                    size: 4096 * 5,
                    skip: 1,
                    used: 1,
                }
            ]
        );

        for entry in allocator.entries[2..].iter() {
            assert_eq!(entry, &BuddyEntry::empty())
        }
    }

    #[test]
    fn combined_memory_regions() {
        let mem_area1 = TestMemoryArea::new();
        let mem_area2 = TestMemoryArea::new();
        let allocator = BuddyFrameAllocator::new(0, &[mem_area1.region(), mem_area2.region()]);

        assert_eq!(
            allocator.entries[0],
            BuddyEntry {
                start_phys: mem_area1.start_phys() + 4096u64, // sub buddy entries table size.
                start_virt: mem_area1.start_virt() + 4096u64, // sub buddy entries table size.
                size: 4096 * 9,
                skip: 1,
                used: 1,
            }
        );

        for entry in allocator.entries[1..].iter() {
            assert_eq!(entry, &BuddyEntry::empty())
        }
    }

    #[test]
    fn allocate_frames() {
        let mem_area = TestMemoryArea::new();
        let mut allocator = BuddyFrameAllocator::new(0, &[mem_area.region()]);

        assert_eq!(mem_area.page_usage_area(true)[0], 0b0000001);

        let frame1 = allocator.allocate_frame().map(PhysFrame::start_address);
        assert_eq!(mem_area.page_usage_area(true)[0], 0b0000011);

        let frame2 = allocator.allocate_frame().map(PhysFrame::start_address);
        assert_eq!(mem_area.page_usage_area(true)[0], 0b0000111);

        let frame3 = allocator.allocate_frame().map(PhysFrame::start_address);
        assert_eq!(mem_area.page_usage_area(true)[0], 0b0001111);

        let frame4 = allocator.allocate_frame().map(PhysFrame::start_address);
        assert_eq!(mem_area.page_usage_area(true)[0], 0b0001111);

        assert_eq!(frame1, Some(mem_area.start_phys() + 4096u64 * 2u64));
        assert_eq!(frame2, Some(mem_area.start_phys() + 4096u64 * 3u64));
        assert_eq!(frame3, Some(mem_area.start_phys() + 4096u64 * 4u64));
        assert_eq!(frame4, None);

        assert!(page_is_zeroed(frame1.unwrap()), "frame 1 not zeroed!");
        assert!(page_is_zeroed(frame2.unwrap()), "frame 2 not zeroed!");
        assert!(page_is_zeroed(frame3.unwrap()), "frame 3 not zeroed!");
    }

    #[test]
    fn deallocate_frames() {
        let mem_area = TestMemoryArea::new();
        let mut allocator = BuddyFrameAllocator::new(0, &[mem_area.region()]);

        assert_eq!(allocator.entries[0].used, 1);
        assert_eq!(allocator.entries[0].skip, 1);

        let frame1 = allocator.allocate_frame().unwrap();
        let frame2 = allocator.allocate_frame().unwrap();
        let frame3 = allocator.allocate_frame().unwrap();

        unsafe {
            assert_eq!(allocator.entries[0].used, 4);
            assert_eq!(allocator.entries[0].skip, 4);
            assert_eq!(mem_area.page_usage_area(true)[0], 0b0001111);

            allocator.deallocate_frame(frame1);

            assert_eq!(allocator.entries[0].used, 3);
            assert_eq!(allocator.entries[0].skip, 1);
            assert_eq!(mem_area.page_usage_area(true)[0], 0b0001101);

            allocator.deallocate_frame(frame2);
            allocator.deallocate_frame(frame3);

            assert_eq!(allocator.entries[0].used, 1);
            assert_eq!(allocator.entries[0].skip, 1);
            assert_eq!(mem_area.page_usage_area(true)[0], 0b0000001);
        }
    }

    fn page_is_zeroed(addr: PhysAddr) -> bool {
        let virt_addr = VirtAddr::new(addr.as_u64());

        unsafe {
            core::slice::from_raw_parts(virt_addr.as_ptr::<u8>(), 4096)
                .iter()
                .find(|b| **b != 0)
                .is_none()
        }
    }
}
