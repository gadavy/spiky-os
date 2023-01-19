use pc_keyboard::layouts::Us104Key;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub const DATA_PORT: Port<u8> = Port::new(0x60);

pub static PC_KEYBOARD: Controller = Controller::new();

type OnKeyHandler = fn(char) -> ();

pub struct Controller {
    inner: Mutex<Option<InnerController>>,
}

impl Controller {
    const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub fn init(&self, handler: OnKeyHandler) {
        self.inner.lock().replace(InnerController::new(handler));
    }

    pub fn read(&self) {
        if let Some(c) = self.inner.lock().as_mut() {
            c.read();
        }
    }
}

struct InnerController {
    decoder: Keyboard<Us104Key, ScancodeSet1>,
    port: Port<u8>,
    handler: OnKeyHandler,
}

impl InnerController {
    fn new(handler: OnKeyHandler) -> Self {
        Self {
            decoder: Keyboard::new(HandleControl::MapLettersToUnicode),
            port: DATA_PORT,
            handler,
        }
    }

    fn read(&mut self) {
        let value = unsafe { self.port.read() };

        let Ok(Some(event)) = self.decoder.add_byte(value) else { return };
        let Some(decoded) = self.decoder.process_keyevent(event) else { return };

        match decoded {
            DecodedKey::Unicode(c) => (self.handler)(c),
            DecodedKey::RawKey(_) => {}
        }
    }
}
