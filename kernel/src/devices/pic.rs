use pic8259::ChainedPics;
use spin::Mutex;

const PIC_1_OFFSET: u8 = 0x20;
const PIC_2_OFFSET: u8 = 0xA0;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    unsafe { PICS.lock().initialize() }
}

pub fn disable() {
    unsafe { PICS.lock().disable() }
}
