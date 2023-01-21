use pc_keyboard::layouts::Us104Key;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub static PC_KEYBOARD: Mutex<KeyboardDriver> = Mutex::new(KeyboardDriver::empty(0x60));

type OnKeyHandler = fn(char) -> ();

pub struct KeyboardDriver {
    decoder: Keyboard<Us104Key, ScancodeSet1>,
    port: Port<u8>,
    handler: Option<OnKeyHandler>,
}

impl KeyboardDriver {
    const fn empty(port: u16) -> Self {
        Self {
            decoder: Keyboard::new(HandleControl::MapLettersToUnicode),
            port: Port::new(port),
            handler: None,
        }
    }

    pub fn init(&mut self, handler: OnKeyHandler) {
        self.handler.replace(handler);
    }

    pub fn read(&mut self) {
        let value = unsafe { self.port.read() };

        let Some(handler) = self.handler else { return };
        let Ok(Some(event)) = self.decoder.add_byte(value) else { return };
        let Some(decoded) = self.decoder.process_keyevent(event) else { return };

        match decoded {
            DecodedKey::Unicode(c) => handler(c),
            DecodedKey::RawKey(_) => {}
        }
    }
}
