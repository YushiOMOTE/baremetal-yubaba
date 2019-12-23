use crate::{
    keyboard::{new_keyboard_us, USKeyboard},
    time::sleep,
};
use alloc::string::String;
use fonts::FontReader;
use log::*;
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub struct Console<G> {
    gfx: G,
    px: usize,
    py: usize,
    fonts: FontReader<'static>,
    kbd: USKeyboard,
}

pub fn new_console_640x480() -> Console<Graphics640x480x16> {
    let gfx = Graphics640x480x16::new();
    gfx.set_mode();
    let fonts = FontReader::new(include_bytes!("./font.bin"));
    Console::new(gfx, fonts)
}

impl<G> Console<G>
where
    G: GraphicsWriter<Color16>,
{
    pub fn new(gfx: G, fonts: FontReader<'static>) -> Self {
        Self {
            gfx,
            fonts,
            px: 0,
            py: 0,
            kbd: new_keyboard_us(),
        }
    }

    pub fn reset(&mut self) {
        self.gfx.clear_screen(Color16::Black);
        self.px = 0;
        self.py = 0;
    }

    pub fn putchar(&mut self, ch: char) {
        let (width, height, bitmap) = self
            .fonts
            .get(ch);

        if ch != '\n' {
            for p in bitmap {
                let c = if p.color != 0 {
                    Color16::White
                } else {
                    Color16::Black
                };
                self.gfx.set_pixel(p.x + self.px, p.y + self.py, c);
            }
            info!("ch: {}, width: {}, height: {}", ch, width, height);
        }

        self.px += width + 1;
        if ch == '\n' {
            self.px = 0;
            self.py += 16;
        }
    }

    pub fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.putchar(c);
        }
    }

    pub fn getchar(&mut self) -> char {
        loop {
            if let Some(ch) = self.kbd.poll_char() {
                return ch;
            }
        }
    }

    pub fn readline(&mut self) -> String {
        let mut s = String::new();
        loop {
            let ch = self.getchar();
            if ch == '\n' {
                break;
            }
            self.putchar(ch);
            s.push(ch);
            sleep(1000000);
        }
        s
    }
}
