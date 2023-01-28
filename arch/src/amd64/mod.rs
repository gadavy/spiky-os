pub mod debug;
pub mod devices;
pub mod entry;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod memory;
pub mod paging;

mod consts {
    pub const KERNEL_BACKUP_STACK_SIZE: usize = 65536; // 64 KB

    pub const KERNEL_PERCPU_OFFSET: u64 = 0xffff_fd80_0000_0000;
    pub const KERNEL_PERCPU_SIZE: u64 = 0x10000;

    pub const KERNEL_HEAP_SIZE: u64 = 1024 * 1024 * 1; // 1 MB
    pub const KERNEL_HEAP_OFFSET: u64 = 0xffff_fe80_0000_0000;
}
