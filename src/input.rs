use sdl3::event::Event;
use sdl3::keyboard::Keycode;

pub struct InputState {
    pub quit: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,         // move camera up (Space)
    pub down: bool,       // move camera down (LShift)
    pub pitch_up: bool,   // tilt view up (Up arrow)
    pub pitch_down: bool, // tilt view down (Down arrow)
}

impl InputState {
    pub fn new() -> Self {
        Self {
            quit: false,
            w: false,
            a: false,
            s: false,
            d: false,
            left: false,
            right: false,
            up: false,
            down: false,
            pitch_up: false,
            pitch_down: false,
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
                    Keycode::Left => self.left = true,
                    Keycode::Right => self.right = true,
                    Keycode::Space => self.up = true,
                    Keycode::LShift => self.down = true,
                    Keycode::Up => self.pitch_up = true,
                    Keycode::Down => self.pitch_down = true,
                    _ => {}
                }
            }

            Event::KeyUp { keycode: Some(key), .. } => {
                match key {
                    Keycode::W => self.w = false,
                    Keycode::A => self.a = false,
                    Keycode::S => self.s = false,
                    Keycode::D => self.d = false,
                    Keycode::Left => self.left = false,
                    Keycode::Right => self.right = false,
                    Keycode::Space => self.up = false,
                    Keycode::LShift => self.down = false,
                    Keycode::Up => self.pitch_up = false,
                    Keycode::Down => self.pitch_down = false,
                    _ => {}
                }
            }

            _ => {}
        }
    }
}