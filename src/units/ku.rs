use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

// Keyboard Unit
pub struct KU {
    event_pump: EventPump,
    pub kd: bool,
    pub key_state: Option<u8>,
}

impl KU {
    // Create new keyboard unit instance
    pub fn new(context: &Sdl) -> KU {
        let event_pump = context.event_pump().ok().unwrap();

        KU {
            event_pump,
            kd: false,
            key_state: None,
        }
    }

    pub fn reset_key_down(&mut self) {
        self.kd = false;
    }

    pub fn get_key_state(&self) -> Option<u8> {
        self.key_state
    }

    // Loop over events and process keystrokes
    pub fn process_input(&mut self) -> Result<[bool; 16], ()> {
        for event_type in self.event_pump.poll_iter() {
            self.kd = true;
            match event_type {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err(());
                }
                _ => {}
            }
        }

        let keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                self.kd = true;
                chip_keys[i] = true;
            }
        }

        Ok(chip_keys)
    }
}
