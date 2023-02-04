use bootloader_api::info::FrameBufferInfo;

pub mod acpi;
pub mod framebuffer;
pub mod ioapic;
pub mod keyboard;
pub mod lapic;
pub mod pic;
pub mod rtc;
pub mod uart;

pub fn init_framebuffer(info: FrameBufferInfo, buf: &'static mut [u8]) {
    framebuffer::FRAMEBUFFER.lock().init(info, buf);

    log::debug!("Framebuffer initialized");
}

pub fn init_keyboard() {
    keyboard::PC_KEYBOARD.lock().init(default_keyboard_handler);

    log::debug!("Keyboard driver initialized");
}

pub fn init_uart() {
    if uart::UART.lock().init(uart::COM1_BASE) {
        log::debug!("UART with base `0x{:02x}` initialized", uart::COM1_BASE);
    } else {
        log::error!("UART with base `0x{:02x}` not supported", uart::COM1_BASE);
    }
}

pub fn init_rtc() {
    if let Some(fadt) = acpi::ACPI.read().fadt() {
        rtc::RTC.lock().init_century_reg(fadt.century_reg);
    } else {
        log::warn!("FADT is empty")
    }
}

pub fn init_acpi(phys_mem_offset: u64, rsdp_addr: u64) {
    acpi::ACPI.write().init(phys_mem_offset, rsdp_addr);
}

fn default_keyboard_handler(c: char) {
    log::trace!("new char: {c}");
}
