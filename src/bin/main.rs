use cj_8::chip_8::Platform;
use cj_8::chip_8::System;
use cj_8::chip_8::AU;
use cj_8::chip_8::GU;
use cj_8::chip_8::KU;

fn main() {
    // Init blank slate system
    let mut system = System::new();
    print!("New CJ-8 created...\n");

    // Setup render system and input
    const WINDOW_WIDTH: u32 = 64;
    const WINDOW_HEIGHT: u32 = 32;
    let context = Platform::new();
    println!("SDL context created...");

    let mut graphical_unit = GU::new(
        &context.context,
        "Chip-8 In Rust!",
        WINDOW_WIDTH * 20,
        WINDOW_HEIGHT * 20,
        // 15,
        // 15,
    );
    let mut keyboard_unit = KU::new(&context.context);
    let audio_unit = AU::new(&context.context);
    graphical_unit.init();
    println!("Front-End Units Initialized...");

    // Clear memory and load ROM
    system.init();

    // Emu loop
    let mut quit = false;
    while !quit {
        // Store keypress state
        quit = keyboard_unit.process_input();

        // beep
        audio_unit.device.resume();
    }
}
