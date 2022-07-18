use rand::Rng;
use crate::units::{au::AU, ku::KU};

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
            sound_timer: 60,
            stack: [0x000; 16],
            sp: 0,
            memory: [0x0; 4096],
            gfx: [[0x000; 64]; 32],
            draw_flag: false,
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
    pub fn emulate_cycle(&mut self, audio_unit: &mut AU, keys: &[bool; 16], keyboard_unit: &KU) {
        // Fetch
        let index = self.pc as usize;
        self.opcode = u16::from(self.memory[index]) << 8 | u16::from(self.memory[index + 1]);
        let op_index = self.opcode as usize;
        let x_reg: usize = (op_index & 0x0F00) >> 8;
        let y_reg: usize = (op_index & 0x00F0) >> 4;
        let vx = self.v[x_reg] as u16;
        let vy = self.v[y_reg] as u16;
        let nn = (self.opcode & 0x00FF) as u16;

        println!("Executing: {:#x}", self.opcode);

        // Decode
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x00FF {
                    // Clears the screen
                    0x00E0 => {
                        // self.gfx = [[0x0000; 64]; 32];
                        for y in 0..32 {
                            for x in 0..64 {
                                self.gfx[y][x] = 0;
                            }
                        }
                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    // Returns from subroutine
                    0x00EE => {
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
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            // Skips the next instruction if VX == NN
            0x3000 => {
                if self.v[x_reg] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX != NN
            0x4000 => {
                if self.v[x_reg] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Skips the next instruction if VX == VY
            0x5000 => {
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            // Sets VX to NN
            0x6000 => {
                self.v[x_reg] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            // // Adds NN to VX
            0x7000 => {
                self.v[x_reg] = (vx + nn) as u8;
                self.pc += 2;
            }
            0x8000 => {
                match self.opcode & 0x000F {
                    // Sets VX to the value of VY
                    0x0000 => {
                        self.v[x_reg] = self.v[y_reg];
                        self.pc += 2;
                    }
                    // Sets VX to VX or VY
                    0x0001 => {
                        self.v[x_reg] = self.v[x_reg] | self.v[y_reg]; 
                        self.pc += 2;
                    }
                    // Sets VX to VX and VY
                    0x0002 => {
                        self.v[x_reg] = self.v[x_reg] & self.v[y_reg]; 
                        self.pc += 2;
                    }
                    // Sets VX to VX xor VY
                    0x0003 => {
                        self.v[x_reg] = self.v[x_reg] ^ self.v[y_reg]; 
                        self.pc += 2;
                    }
                    // Adds the value of register VY to VX
                    0x0004 => {
                        // If sum is larger than 255 set carry flag
                        if vy
                            > (0xFF - vx)
                        {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        let sum = (vx + vy) as u8;
                        self.v[x_reg] = sum;
                        self.pc += 2;
                    }
                    // VY is subtracted from VX and VF is set to 0 when there is a borrow and 1 when there is not
                    0x0005 => {
                        if vx > vy {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        
                        self.v[x_reg] = self.v[x_reg].wrapping_sub(self.v[y_reg]);
                        self.pc += 2;
                    }
                    // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
                    0x0006 => {
                        self.v[0xF] = self.v[x_reg] & 1;
                        self.v[x_reg] >>= 1;
                        self.pc += 2;
                    }
                    // Sets VX to VY - VX and VF is set to 0 when there is a borrow and 1 when there is not
                    0x0007 => {
                        if self.v[y_reg] > self.v[x_reg] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        let difference = vy - vx;
                        self.v[x_reg] = difference as u8;
                        self.pc += 2;
                    }
                    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                    0x000E => {
                        self.v[0xF] = (self.v[x_reg] & 0b10000000) >> 7;
                        self.v[x_reg] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        panic!("ERROR: Unknown opcode: {:#X}", self.opcode);
                    }
                }
            }
            // // Skips the next instruction if VX != VY
            0x9000 => {
                if vx != vy {
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
            // Jumps to the address NNN plus V0
            0xB000 => {
                self.pc = (self.opcode & 0x0FFF) + self.v[0] as u16;
            }
            // Sets VX to equal a random number & NN
            0xC000 => {
                let mut rng = rand::thread_rng();
                let number: u8 = rng.gen::<u8>();
                self.v[x_reg] = number & (self.opcode & 0x00FF) as u8;
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
                    let y = (self.v[y_reg] + byte as u8) % 32;
                    for bit in 0..8 {
                        let x = (self.v[x_reg] + bit) % 64;
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
                        if keys[vx as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0x00A1 => {
                        if !keys[vx as usize] {
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
                    // Sets VX to the value of the delay timer
                    0x0007 => {
                        self.v[x_reg] = self.delay_timer;
                        self.pc += 2;
                    }
                    // Blocks and then put key value into VX
                    0x000A => {
                        if keyboard_unit.kd==false {
                            self.pc -= 2;
                        } else {
                            for (i, key) in keys.iter().enumerate() {
                               if *key {
                                self.v[x_reg] = i as u8;
                               }
                            }
                            self.pc += 2;
                        }
                    }
                    // Sets the delay timer to VX
                    0x0015 => {
                        self.delay_timer = self.v[x_reg];
                        self.pc += 2;
                    }
                    // Sets the sound timer to VX
                    0x0018 => {
                        self.sound_timer = self.v[x_reg];
                        self.pc += 2;
                    }
                    // Adds VX to I without VF being affected
                    0x001E => {
                        self.i += vx as usize;
                        self.pc += 2;
                    }
                    // Sets i to the location of the sprite for the character in VX
                    0x0029 => {
                        self.i = vx as usize;
                        self.pc += 2;
                    }
                    // Stores binary decimal representation of VX at address i, i + 1, and i + 2
                    0x0033 => {
                        self.memory[self.i] = self.v[x_reg] / 100;
                        self.memory[self.i + 1] = (self.v[x_reg] / 10) % 10;
                        self.memory[self.i + 2] = (self.v[x_reg] % 100) % 10;
                        self.pc += 2;
                    }
                    // Dump values from V0 to VX into memory starting at address i  with + 1 offset with i left unmodified
                    0x0055 => {
                        for (i_offset, v) in self.v[0..x_reg + 1].iter().enumerate() {
                            self.memory[self.i + i_offset] = *v;
                        }
                        self.pc += 2;
                    }
                    // Fills values from VO to VX with values from memory starting at address i with + 1 offest with i left unmodified
                    0x0065 => {
                        let x = x_reg as u8;
                        for i_offset in 0..=x {
                            self.v[i_offset as usize] = self.memory[self.i + i_offset as usize];
                        }
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
            audio_unit.device.resume();
            self.sound_timer -= 1;
        } else {
            audio_unit.device.pause();
        }

        // Print timer state to console
        println!(
            "Delay Timer: {}\nSound Timer: {}",
            self.delay_timer, self.sound_timer
        );
    }
}