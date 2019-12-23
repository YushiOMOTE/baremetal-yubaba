use pc_keyboard::{
    layouts, DecodedKey, HandleControl, KeyState, KeyboardLayout, ScancodeSet, ScancodeSet1,
};
use x86_64::instructions::port::Port;

pub struct Keyboard<T, S>
where
    T: KeyboardLayout,
    S: ScancodeSet,
{
    port: Port<u8>,
    kbd: pc_keyboard::Keyboard<T, S>,
    pressed: bool,
}

impl<T, S> Keyboard<T, S>
where
    T: KeyboardLayout,
    S: ScancodeSet,
{
    pub fn new(layout: T, scancodes: S) -> Self {
        Self {
            port: Port::new(0x60),
            kbd: pc_keyboard::Keyboard::new(layout, scancodes, HandleControl::MapLettersToUnicode),
            pressed: false,
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe { self.port.read() }
    }

    pub fn poll_char(&mut self) -> Option<char> {
        let b = self.read_byte();
        let event = self.kbd.add_byte(b).ok()??;

        match (event.state, self.pressed) {
            (KeyState::Down, true) => None,
            (KeyState::Down, false) => {
                self.pressed = true;
                match self.kbd.process_keyevent(event) {
                    Some(DecodedKey::Unicode(c)) => Some(c),
                    _ => None,
                }
            }
            (KeyState::Up, true) => {
                self.pressed = false;
                None
            }
            (KeyState::Up, false) => None,
        }
    }
}

pub type USKeyboard = Keyboard<layouts::Us104Key, ScancodeSet1>;

pub fn new_keyboard_us() -> USKeyboard {
    Keyboard::new(layouts::Us104Key, ScancodeSet1)
}
