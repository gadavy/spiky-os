pub const PAGE_SHIFT: u64 = 12;
pub const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
pub const PAGE_OFFSET_MASK: u64 = PAGE_SIZE - 1;

pub const KERNEL_BACKUP_STACK_SIZE: u64 = 65536; // 64 KB
pub const KERNEL_BACKUP_STACK_INDEX: u16 = 0;

pub const KERNEL_PERCPU_SIZE: u64 = 0x20000;
pub const KERNEL_PERCPU_OFFSET: u64 = 0xffff_fd80_0000_0000;

pub const KERNEL_HEAP_SIZE: u64 = 1024 * 1024; // 1 MB
pub const KERNEL_HEAP_OFFSET: u64 = 0xffff_fe80_0000_0000;

pub const TRAMPOLINE: u64 = 0x8000;
pub static TRAMPOLINE_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/trampoline"));
