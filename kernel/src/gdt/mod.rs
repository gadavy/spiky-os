use spin::{Lazy, Mutex};
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS, DS, ES, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });

        stack_start + STACK_SIZE
    };

    tss
});

static GDT: Mutex<Gdt> = Mutex::new(Gdt::new());

pub fn init() {
    GDT.lock().init(&TSS);
}

pub struct Gdt {
    gdt: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            code_selector: SegmentSelector::NULL,
            data_selector: SegmentSelector::NULL,
            tss_selector: SegmentSelector::NULL,
        }
    }

    pub fn init(&mut self, tss: &'static TaskStateSegment) {
        self.code_selector = self.gdt.add_entry(Descriptor::kernel_code_segment());
        self.data_selector = self.gdt.add_entry(Descriptor::kernel_data_segment());
        self.tss_selector = self.gdt.add_entry(Descriptor::tss_segment(tss));

        unsafe {
            self.gdt.load_unsafe();

            CS::set_reg(self.code_selector);
            SS::set_reg(self.data_selector);
            DS::set_reg(self.data_selector);
            ES::set_reg(self.data_selector);
            load_tss(self.tss_selector);
        }
    }
}
