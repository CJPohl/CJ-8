use std::env;

use cj_8::chip_8::Platform;
use cj_8::chip_8::System;
use cj_8::chip_8::AU;
use cj_8::chip_8::CU;
use cj_8::chip_8::GU;
use cj_8::chip_8::KU;

fn main() {
    // Accept args and throw errors if necessary
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: Failed to parse args");
        panic!("usage: cj-8 resolution-scale path-to-ROM");
    }
    println!("Args accepted");

    let scale = args[1].parse::<u32>().unwrap();

    // Init cartridge unit
    let cartridge = CU::new(&args[2]).unwrap();

    // Init blank slate system
    let mut system = System::new();
    println!("New CJ-8 created with cartridge");

    // Setup render system and input
    const WINDOW_WIDTH: u32 = 64;
    const WINDOW_HEIGHT: u32 = 32;
    let context = Platform::new();
    println!("SDL context created");

    let mut graphical_unit = GU::new(
        &context.context,
        "CJ-8",
        WINDOW_WIDTH * scale,
        WINDOW_HEIGHT * scale,
        // 15,
        // 15,
    );
    let mut keyboard_unit = KU::new(&context.context);
    let audio_unit = AU::new(&context.context);
    graphical_unit.init();
    println!("Front-End Units Initialized");

    // Clear memory and load ROM
    system.init(cartridge.buffer);

    // Emu loop
    let mut quit = false;
    while !quit {
        // Store keypress state
        quit = keyboard_unit.process_input();

        // Program Cycle
        system.emulate_cycle(&audio_unit);

        // Check drawflag
        if system.draw_flag == true {
            // graphical_unit::draw(&system.gfx);
        }
    }
}
