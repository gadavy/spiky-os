use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Early
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut EARLY_GDT: EarlyGdt = EarlyGdt::empty();

/// Init early GDT, before we have access to thread locals.
pub fn init_early() {
    log::trace!("Init early GDT");

    unsafe { EARLY_GDT.init() };
}

pub struct EarlyGdt {
    gdt: GlobalDescriptorTable,

    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    kernel_tls: SegmentSelector,
}

impl EarlyGdt {
    const fn empty() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            kernel_code: SegmentSelector::NULL,
            kernel_data: SegmentSelector::NULL,
            kernel_tls: SegmentSelector::NULL,
        }
    }

    pub fn init(&mut self) {
        *self = Self::empty();

        self.kernel_code = self.gdt.add_entry(Descriptor::kernel_code_segment());
        self.kernel_data = self.gdt.add_entry(Descriptor::kernel_data_segment());
        self.kernel_tls = self.gdt.add_entry(Descriptor::kernel_data_segment());

        unsafe {
            self.gdt.load_unsafe();

            CS::set_reg(self.kernel_code);
            DS::set_reg(self.kernel_data);
            ES::set_reg(self.kernel_data);
            FS::set_reg(self.kernel_data);
            GS::set_reg(self.kernel_tls);
            SS::set_reg(self.kernel_data);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Thread locals
////////////////////////////////////////////////////////////////////////////////////////////////////

#[thread_local]
static mut BACKUP_STACK: [u8; 256] = [0; 256];

#[thread_local]
pub(crate) static mut TSS: TaskStateSegment = TaskStateSegment::new();

#[thread_local]
static mut GDT: Gdt = Gdt::empty();

/// Init GDT with thread local.
pub fn init() {
    log::trace!("Init GDT");

    unsafe {
        GDT.init();
    }
}

pub struct Gdt {
    gdt: GlobalDescriptorTable,

    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    kernel_tls: SegmentSelector,
    kernel_tss: SegmentSelector,

    user_data: SegmentSelector,
    user_code: SegmentSelector,
}

impl Gdt {
    const fn empty() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            kernel_code: SegmentSelector::NULL,
            kernel_data: SegmentSelector::NULL,
            kernel_tls: SegmentSelector::NULL,
            kernel_tss: SegmentSelector::NULL,
            user_data: SegmentSelector::NULL,
            user_code: SegmentSelector::NULL,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            TSS.privilege_stack_table[usize::from(KERNEL_BACKUP_STACK_INDEX)] = {
                let stack_start = VirtAddr::from_ptr(&BACKUP_STACK);
                stack_start + KERNEL_BACKUP_STACK_SIZE
            };

            self.kernel_code = self.gdt.add_entry(Descriptor::kernel_code_segment());
            self.kernel_data = self.gdt.add_entry(Descriptor::kernel_data_segment());
            self.kernel_tls = self.gdt.add_entry(Descriptor::kernel_data_segment());
            self.kernel_tss = self.gdt.add_entry(Descriptor::tss_segment(&TSS));
            self.user_data = self.gdt.add_entry(Descriptor::user_data_segment());
            self.user_code = self.gdt.add_entry(Descriptor::user_code_segment());
        }

        unsafe {
            self.gdt.load_unsafe();

            CS::set_reg(self.kernel_code);
            DS::set_reg(self.kernel_data);
            ES::set_reg(self.kernel_data);
            SS::set_reg(self.kernel_data);

            load_tss(self.kernel_tss);
        }
    }
}
