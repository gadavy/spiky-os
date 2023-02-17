#![feature(thread_local)]
#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use bootloader_api::info::TlsTemplate;
use spin::Once;

mod debug;
mod devices;
mod gdt;
mod idt;
mod interrupts;
mod logger;
mod memory;
mod paging;
mod prelude;

static PHYS_MEMORY_OFFSET: Once<u64> = Once::new();
static TLS_TEMPLATE: Once<TlsTemplate> = Once::new();

pub fn entry(info: &'static mut bootloader_api::BootInfo) -> ! {
    // Init logging.
    devices::init_early(info.framebuffer.as_mut());
    logger::init(debug::write_log);

    let phys_mem_offset = info
        .physical_memory_offset
        .into_option()
        .expect("Physical memory offset not specified in boot information");

    let tls_template = info
        .tls_template
        .into_option()
        .expect("TLS template not specified in boot information");

    PHYS_MEMORY_OFFSET.call_once(|| phys_mem_offset);
    TLS_TEMPLATE.call_once(|| tls_template);

    // Init GDT and IDT early before TLS initialized.
    gdt::init_early();
    idt::init_early();

    // Init memory and TLS.
    memory::init(phys_mem_offset, &info.memory_regions);
    paging::init(0, tls_template);

    // Init GDT and IDT with TLS.
    gdt::init();
    idt::init_bsp(phys_mem_offset);

    // Init kernel heap.
    memory::init_heap();

    devices::init(phys_mem_offset, info.rsdp_addr.into_option());

    interrupts::enable();

    log::info!("Spiky OS started...");

    loop {
        interrupts::hlt();
    }
}

fn ap_entry(cpu_id: u64) -> ! {
    log::info!("AP CORE_{cpu_id} starting...");

    let phys_mem_offset = PHYS_MEMORY_OFFSET
        .get()
        .copied()
        .expect("Physical memory offset should be initialized");

    let tls_template = TLS_TEMPLATE
        .get()
        .copied()
        .expect("TLS template should be initialized");

    // Init GDT and IDT early before TLS initialized.
    gdt::init_early();
    idt::init_early();

    // Init TLS.
    paging::init(cpu_id, tls_template);

    // Init GDT and IDT with TLS.
    gdt::init();
    idt::init_ap(phys_mem_offset);

    devices::init_ap();

    loop {
        interrupts::hlt();
    }
}
