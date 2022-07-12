extern crate sdl2;

pub mod chip_8 {
    use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::{self, Color};
    use sdl2::rect::Rect;
    use sdl2::{render::Canvas, EventPump, Sdl};
    use std::fs;

    pub struct System {
        opcode: u16,
        v: [u8; 16],
        i: usize,
        pc: u16,
        delay_timer: u8,
        sound_timer: u8,
        stack: [u16; 16],
        sp: usize,
        memory: [u8; 4096],
        pub gfx: [[u16; 64]; 32],

        pub draw_flag: bool,
        key: [u8; 16],
        font_set: [u8; 80],
    }

    impl System {
        pub fn new() -> System {
            System {
                opcode: 0x000,
                v: [0x0; 16],
                i: 0,
                pc: 0x200,
                delay_timer: 60,
                sound_timer: 1,
                stack: [0x000; 16],
                sp: 0,
                memory: [0x0; 4096],
                gfx: [[0x000; 64]; 32],
                draw_flag: false,
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

        pub fn init(&mut self, buffer: Vec<u8>) {
            // Load fontset
            for (i, font) in self.font_set.iter().enumerate() {
                self.memory[i] = *font;
            }
            println!("Font loaded");

            // Load ROM into memory at address 0x200
            for (i, byte) in buffer.iter().enumerate() {
                self.memory[i + 512] = *byte
            }
            println!("Cartridge loaded successfully");
        }

        // Turn off draw flag
        pub fn falsify_df(&mut self) {
            self.draw_flag = false;
        }

        // Test hardcoded opcodes
        pub fn emulate_cycle(&mut self, audio_unit: &mut AU) {
            // Fetch
            let index = self.pc as usize;
            self.opcode = u16::from(self.memory[index]) << 8 | u16::from(self.memory[index + 1]);
            let op_index = self.opcode as usize;

            println!("Executing: {:#x}", self.opcode);

            // Decode
            match self.opcode & 0xF000 {
                0x0000 => {
                    match self.opcode & 0x000F {
                        // Clears the screen
                        0x0000 => {
                            self.gfx = [[0x0000; 64]; 32];
                            self.pc += 2;
                        }
                        // Returns from subroutine
                        0x000E => {
                            self.sp -= 1;
                            self.pc = self.stack[self.sp];
                        }
                        _ => {
                            panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                        }
                    }
                }
                // Jumps to address NNN
                0x1000 => {
                    self.pc = self.opcode & 0x0FFF;
                }
                // // Calls the subroutine at address NNN
                0x2000 => {
                    self.stack[self.sp] = self.pc;
                    self.sp += 1;
                    self.pc = self.opcode & 0x0FFF;
                }
                // Skips the next instruction if VX == NN
                0x3000 => {
                    if self.v[(op_index & 0x0F00) >> 8] == (self.opcode & 0x00FF) as u8 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                // Skips the next instruction if VX != NN
                0x4000 => {
                    if self.v[(op_index & 0x0F00) >> 8] != (self.opcode & 0x00FF) as u8 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                // Skips the next instruction if VX == VY
                0x5000 => {
                    if self.v[(op_index & 0x0F00) >> 8] == self.v[(op_index & 0x00F0) >> 4] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                // Sets VX to NN
                0x6000 => {
                    self.v[(op_index & 0x0F00) >> 8] = (self.opcode & 0x00FF) as u8;
                    self.pc += 2;
                }
                // // Adds NN to VX
                0x7000 => {
                    let vx = self.v[(op_index & 0x0F00) >> 8] as u16;
                    let nn = (self.opcode & 0x00FF) as u16;
                    self.v[(op_index & 0x0F00) >> 8] = (vx + nn) as u8;
                    self.pc += 2;
                }
                0x8000 => {
                    match self.opcode & 0x0FFF {
                        // Adds the value of register VY to VX
                        0x0004 => {
                            // If sum is larger than 255 set carry flag
                            if self.v[(op_index & 0x00F0) >> 4]
                                > (0xFF - self.v[(op_index & 0x0F00) >> 8])
                            {
                                self.v[0xF] = 1;
                            } else {
                                self.v[0xF] = 0;
                            }
                            self.v[(op_index & 0x0F00) >> 8] += self.v[(op_index & 0x00F0) >> 4];
                            self.pc += 2;
                        }
                        _ => {
                            panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                        }
                    }
                }
                // // Skips the next instruction if VX != VY
                0x9000 => {
                    if self.v[(op_index & 0x0F00) >> 8] != self.v[(op_index & 0x00F0) >> 4] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                // Sets i to the address NNN
                0xA000 => {
                    self.i = (self.opcode & 0x0FFF) as usize;
                    self.pc += 2;
                }
                // Draw a sprite at coord (VX, VY)
                0xD000 => {
                    // Set height
                    let height = self.opcode & 0x000F;

                    // Reset VF
                    self.v[0xF] = 0;

                    // This looping block is heavily inspired by starrhorne's chip-8 impl
                    // Credit due to her
                    for byte in 0..height {
                        let y = (self.v[(op_index & 0x00F0) >> 4 as usize] + byte as u8) % 32;
                        for bit in 0..8 {
                            let x = (self.v[(op_index & 0x0F00) >> 8 as usize] + bit) % 64;
                            let color = (self.memory[self.i + byte as usize] >> (7 - bit)) & 1;
                            self.v[0xF] |= color as u8 & self.gfx[y as usize][x as usize] as u8;
                            self.gfx[y as usize][x as usize] ^= color as u16;
                        }
                    }

                    self.draw_flag = true;
                    self.pc += 2;
                }
                0xE000 => {
                    match self.opcode & 0x00FF {
                        // Skips next instruction if key stored in VX is pressed
                        0x009E => {
                            if self.key[self.v[(op_index & 0x0F00) >> 8] as usize] != 0 {
                                self.pc += 4;
                            } else {
                                self.pc += 2;
                            }
                        }
                        _ => {
                            panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                        }
                    }
                }

                0xF000 => {
                    match self.opcode & 0x00FF {
                        // Sets i to the location of the sprite for the character in VX
                        0x0029 => {
                            self.i = self.v[(op_index & 0x0F00) >> 8] as usize;
                            self.pc += 2;
                        }
                        // Stores binary decimal representation of VX at address i, i + 1, and i + 2
                        0x0033 => {
                            self.memory[self.i] = self.v[(op_index & 0x0F00) >> 8] / 100;
                            self.memory[self.i + 1] = (self.v[(op_index & 0x0F00) >> 8] / 10) % 10;
                            self.memory[self.i + 2] = (self.v[(op_index & 0x0F00) >> 8] % 100) % 10;
                            self.pc += 2;
                        }
                        0x0065 => {
                            // TODO
                            self.pc += 2;
                        }
                        _ => {
                            panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                        }
                    }
                }
                _ => {
                    panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                }
            }

            // Update timers
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    audio_unit.device.resume();
                    self.sound_timer -= 1;
                    audio_unit.device.pause();
                }
            }

            // Print timer state to console
            println!(
                "Delay Timer: {}\nSound Timer: {}",
                self.delay_timer, self.sound_timer
            );
        }
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

    // Cartridge Unit
    pub struct CU {
        pub buffer: Vec<u8>,
    }

    impl CU {
        pub fn new(path: &str) -> Result<CU, ()> {
            let buffer = fs::read(path).expect("ERROR: Unable to locate ROM");
            Ok(CU { buffer })
        }
    }

    // Graphical Unit
    pub struct GU {
        pub canvas: Canvas<sdl2::video::Window>,
    }

    impl GU {
        pub fn new(context: &Sdl, title: &str, window_width: u32, window_height: u32) -> GU {
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
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.present();
        }

        pub fn draw(&mut self, scale: u32, gfx: &[[u16; 64]; 32]) {
            for (y, row) in gfx.iter().enumerate() {
                for (x, &col) in row.iter().enumerate() {
                    let x = (x as u32) * scale;
                    let y = (y as u32) * scale;

                    self.canvas.set_draw_color(color(col));
                    let _ = self
                        .canvas
                        .fill_rect(Rect::new(x as i32, y as i32, scale, scale));
                }
            }
            self.canvas.present();
        }
    }
    fn color(i: u16) -> pixels::Color {
        if i == 0 {
            pixels::Color::RGB(0, 0, 0)
        } else {
            pixels::Color::RGB(255, 255, 255)
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
}
