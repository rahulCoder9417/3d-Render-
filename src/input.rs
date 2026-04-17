use sdl3::event::Event;
use sdl3::keyboard::Keycode;

pub struct InputState {
    pub quit: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            quit: false,
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Quit { .. } => self.quit = true,

            Event::KeyDown { keycode: Some(key), repeat: false, .. } => {
                match key {
                    Keycode::Escape => self.quit = true,
                    Keycode::W => self.w = true,
                    Keycode::A => self.a = true,
                    Keycode::S => self.s = true,
                    Keycode::D => self.d = true,
                    _ => {}
                }
            }

            Event::KeyUp { keycode: Some(key), .. } => {
                match key {
                    Keycode::W => self.w = false,
                    Keycode::A => self.a = false,
                    Keycode::S => self.s = false,
                    Keycode::D => self.d = false,
                    _ => {}
                }
            }

            _ => {}
        }
    }
}