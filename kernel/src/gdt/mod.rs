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

    log::debug!("GDT initialized");
}

pub struct Gdt {
    gdt: GlobalDescriptorTable,

    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    kernel_tss: SegmentSelector,

    user_data: SegmentSelector,
    user_code: SegmentSelector,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            kernel_code: SegmentSelector::NULL,
            kernel_data: SegmentSelector::NULL,
            kernel_tss: SegmentSelector::NULL,
            user_data: SegmentSelector::NULL,
            user_code: SegmentSelector::NULL,
        }
    }

    pub fn init(&mut self, tss: &'static TaskStateSegment) {
        self.kernel_code = self.gdt.add_entry(Descriptor::kernel_code_segment());
        self.kernel_data = self.gdt.add_entry(Descriptor::kernel_data_segment());
        self.kernel_tss = self.gdt.add_entry(Descriptor::tss_segment(tss));
        self.user_data = self.gdt.add_entry(Descriptor::user_data_segment());
        self.user_code = self.gdt.add_entry(Descriptor::user_code_segment());

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
