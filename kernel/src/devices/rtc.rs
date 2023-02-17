#![allow(dead_code)]

use spin::Mutex;
use x86_64::instructions::port::Port;

const CMOS_ADDR: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

pub static RTC: Mutex<Rtc> = Mutex::new(Rtc::new(CMOS_ADDR, CMOS_DATA));

pub struct Rtc {
    addr: Port<u8>,
    data: Port<u8>,

    century_reg: Option<u8>,
}

impl Rtc {
    const fn new(addr: u16, data: u16) -> Self {
        Self {
            addr: Port::new(addr),
            data: Port::new(data),
            century_reg: None,
        }
    }

    pub(super) fn init(&mut self, century_reg: u8) {
        self.century_reg.replace(century_reg);
    }

    pub fn time(&mut self) -> u64 {
        loop {
            unsafe {
                while self.update_in_progress() {}
                let time0 = self.read_unchecked_time();
                while self.update_in_progress() {}
                let time1 = self.read_unchecked_time();

                if time0 == time1 {
                    return time0;
                }
            }
        }
    }

    unsafe fn read_unchecked_time(&mut self) -> u64 {
        let mut second = self.read(0) as usize;
        let mut minute = self.read(2) as usize;
        let mut hour = self.read(4) as usize;
        let mut day = self.read(7) as usize;
        let mut month = self.read(8) as usize;
        let mut year = self.read(9) as usize;
        let mut century = if let Some(reg) = self.century_reg {
            self.read(reg) as usize
        } else {
            20
        };

        let register_b = self.read(0xB);

        if register_b & 4 != 4 {
            second = cvt_bcd(second);
            minute = cvt_bcd(minute);
            hour = cvt_bcd(hour & 0x7F) | (hour & 0x80);
            day = cvt_bcd(day);
            month = cvt_bcd(month);
            year = cvt_bcd(year);
            century = if self.century_reg.is_some() {
                cvt_bcd(century)
            } else {
                century
            };
        }

        if register_b & 2 != 2 || hour & 0x80 == 0x80 {
            hour = ((hour & 0x7F) + 12) % 24;
        }

        year += century * 100;

        // Unix time from clock
        let mut secs: u64 = (year as u64 - 1970) * 31_536_000;
        let mut leap_days = (year as u64 - 1972) / 4 + 1;

        if year % 4 == 0 && month <= 2 {
            leap_days -= 1;
        }

        secs += leap_days * 86_400;

        match month {
            2 => secs += 2_678_400,
            3 => secs += 5_097_600,
            4 => secs += 7_776_000,
            5 => secs += 10_368_000,
            6 => secs += 13_046_400,
            7 => secs += 15_638_400,
            8 => secs += 18_316_800,
            9 => secs += 20_995_200,
            10 => secs += 23_587_200,
            11 => secs += 26_265_600,
            12 => secs += 28_857_600,
            _ => (),
        }

        secs += (day as u64 - 1) * 86_400;
        secs += hour as u64 * 3600;
        secs += minute as u64 * 60;
        secs += second as u64;

        secs
    }

    unsafe fn read(&mut self, reg: u8) -> u8 {
        self.addr.write(reg);
        self.data.read()
    }

    unsafe fn update_in_progress(&mut self) -> bool {
        self.read(0xA) & 80 != 0
    }
}

fn cvt_bcd(value: usize) -> usize {
    (value & 0xF) + ((value / 16) * 10)
}
