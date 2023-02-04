use bootloader_api::info::FrameBuffer;

// pub mod acpi;
pub mod display;
pub mod serial;

pub fn init_serial() {
    serial::init();
}

pub fn init_display(info: Option<&'static mut FrameBuffer>) {
    if let Some(fb) = info {
        display::init(fb.info(), fb.buffer_mut());
    }
}
