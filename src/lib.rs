extern crate sdl2;

pub mod chip_8 {
    use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::{render::Canvas, EventPump, Sdl};

    pub struct System {
        opcode: u16,
        v: [u8; 16],
        i: u16,
        pc: u16,
        delay_timer: u8,
        sound_timer: u8,
        stack: [u16; 16],
        sp: u16,
        memory: [u8; 4096],
        gfx: [u8; 64 * 32],
        key: [u8; 16],
        font_set: [u8; 80],
    }

    impl System {
        pub fn new() -> System {
            System {
                opcode: 0,
                v: [0x0; 16],
                i: 0,
                pc: 0x200,
                delay_timer: 60,
                sound_timer: 60,
                stack: [0x000; 16],
                sp: 0,
                memory: [0x0; 4096],
                gfx: [0x0; 64 * 32],
                key: [0x0; 16],
                font_set: [
                    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0,
                    0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0,
                    0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40,
                    0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90,
                    0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0,
                    0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0,
                    0x80, 0x80,
                ],
            }
        }

        pub fn init(&mut self) {
            // Load fontset
            for (i, font) in self.font_set.iter().enumerate() {
                self.memory[i] = *font;
            }

            // Load ROM into memory at address 0x200
            // for (i, byte) in buffer.iter().enumerate() {
            //              self.memory[i+512] = *byte
            // }
        }

        pub fn emulate_cycle() {}
    }

    pub struct Platform {
        pub context: Sdl,
    }

    impl Platform {
        pub fn new() -> Platform {
            let sdl_context = sdl2::init().unwrap();
            {
                Platform {
                    context: sdl_context,
                }
            }
        }
    }

    // Graphical Unit
    pub struct GU {
        pub canvas: Canvas<sdl2::video::Window>,
    }

    impl GU {
        pub fn new(
            context: &Sdl,
            title: &str,
            window_width: u32,
            window_height: u32,
            // texture_width: i32,
            // texture_height: i32,
        ) -> GU {
            let video_subsystem = context.video().unwrap();

            let window = video_subsystem
                .window(title, window_width, window_height)
                .position_centered()
                .build()
                .unwrap();

            let canvas = window.into_canvas().build().unwrap();

            GU { canvas }
        }

        pub fn init(&mut self) {
            self.canvas.clear();
            self.canvas.present();
        }
    }

    // Audio Unit
    pub struct AU {
        pub device: AudioDevice<SquareWave>,
    }

    impl AU {
        pub fn new(context: &Sdl) -> AU {
            let audio_subsystem = context.audio().unwrap();

            let desired_spec = AudioSpecDesired {
                freq: Some(44_100),
                channels: Some(1),
                samples: None,
            };

            let device = audio_subsystem
                .open_playback(None, &desired_spec, |spec| {
                    println!("{:?}", spec);

                    // init callback device
                    SquareWave {
                        phase_inc: 440.0 / spec.freq as f32,
                        phase: 0.0,
                        volume: 0.25,
                    }
                })
                .unwrap();

            AU { device }
        }
    }

    pub struct SquareWave {
        phase_inc: f32,
        phase: f32,
        volume: f32,
    }

    impl AudioCallback for SquareWave {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            // gen square wave
            for x in out.iter_mut() {
                *x = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }

    // Keyboard Unit
    pub struct KU {
        event_pump: EventPump,
    }

    impl KU {
        // Create new keyboard unit instance
        pub fn new(context: &Sdl) -> KU {
            let event_pump = context.event_pump().ok().unwrap();

            KU { event_pump }
        }

        // Loop over events and process keystrokes
        pub fn process_input(&mut self) -> bool {
            for event_type in self.event_pump.poll_iter() {
                match event_type {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return true,
                    _ => {}
                }
            }

            false
        }
    }

    // ROM Unit
}
