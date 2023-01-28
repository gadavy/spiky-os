use bootloader_api::info::TlsTemplate;
use x86_64::registers::model_specific::FsBase;
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::VirtAddr;

use super::consts::*;

const TLS_ALIGN: u64 = 16;

pub fn init(cpu_id: u64, mut tls: TlsTemplate) {
    log::trace!("Init paging");

    let mut manager = super::memory::MEMORY_MANAGER.lock();

    // In some cases, `mem_size` and `file_size` might not be aligned, so fix this.
    tls.mem_size += tls.mem_size % TLS_ALIGN;
    tls.file_size += tls.file_size % TLS_ALIGN;

    // Map pages per cpu.
    let start = VirtAddr::new(KERNEL_PERCPU_OFFSET + KERNEL_PERCPU_SIZE * cpu_id);
    let end = start + tls.mem_size;

    let start_page = Page::containing_address(start);
    let end_page = Page::containing_address(end);
    let page_range = Page::range_inclusive(start_page, end_page);
    let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::NO_EXECUTE
        | PageTableFlags::GLOBAL;

    for page in page_range {
        manager
            .map(page, flags)
            .expect("failed to allocate page for TLS")
            .flush();
    }

    unsafe {
        // Copy .tdata section.
        core::ptr::copy(
            tls.start_addr as *const u8,
            start.as_u64() as *mut u8,
            tls.file_size as usize,
        );

        // Zeroing .tbss section.
        core::ptr::write_bytes(
            (start.as_u64() + tls.file_size) as *mut u8,
            0,
            (tls.mem_size - tls.file_size) as usize,
        );

        *end.as_mut_ptr() = end.as_u64();
    }

    FsBase::write(end);
}

// Test of zero values in thread BSS
#[thread_local]
static mut TBSS_TEST: u64 = 0;

// Test of non-zero values in thread data.
#[thread_local]
static mut TDATA_TEST: u64 = u64::MAX;

pub fn test() {
    log::trace!("Test paging");

    unsafe {
        assert_eq!(TBSS_TEST, 0);
        assert_eq!(TDATA_TEST, u64::MAX);

        TBSS_TEST += 1;
        TDATA_TEST -= 1;

        assert_eq!(TBSS_TEST, 1);
        assert_eq!(TDATA_TEST, u64::MAX - 1);
    }
}
