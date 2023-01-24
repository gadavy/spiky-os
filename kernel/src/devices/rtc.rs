use x86_64::instructions::port::Port;

const CMOS_ADDR: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

pub const NANOS_PER_SEC: u128 = 1_000_000_000;

pub struct Rtc {
    addr: Port<u8>,
    data: Port<u8>,
}

impl Rtc {
    const fn new(addr: u16, data: u16) -> Self {
        Self {
            addr: Port::new(addr),
            data: Port::new(data),
        }
    }

    pub fn time(&mut self) -> u64 {}

    unsafe fn read_time(&mut self) -> u64 {
        let mut second = self.read(0) as usize;
        let mut minute = self.read(2) as usize;
        let mut hour = self.read(4) as usize;
        let mut day = self.read(7) as usize;
        let mut month = self.read(8) as usize;
        let mut year = self.read(9) as usize;
    }

    unsafe fn write(&mut self, reg: u8, value: u8) {
        self.addr.write(reg);
        self.data.write(value);
    }

    unsafe fn read(&mut self, reg: u8) -> u8 {
        self.addr.write(reg);
        self.data.read()
    }

    unsafe fn update_in_progress(&mut self) -> bool {
        self.read(0xA) & 80 != 0
    }
}

#[test]
fn test() {
    assert_eq!(0xA & 0x7F, 0x0A)
}
