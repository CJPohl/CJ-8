# CJ-8

## What is CJ-8?

CJ-8 is based on the Chip-8, a "fantasy" video game console conceived in the 1970s which never had its own hardware release. Instead, it exists as its own interpreted programming language run on a Chip-8 virtual machine. Because of its simplicity in design and small number of cpu instructions, it was designed to allow video games to be programmed more easily. Over the years it has been ported to various systems where ROMs and new versions of the interpreter still get developed to this day.

## Goals for this implementation

- **Understanding CPU Architecture**: While studying Computer Architecture on my free time, I found that an applied project like this allowed for a deeper understanding of the subject.

- **Delve Into Lower Level Programming**: It is no lie that programming in languages like C, C++, or Rust takes a certain amount more attention to detail than its higher level counterparts. Given I've had experience programming in C already, it wasn't difficult to conceptualize how Rust implements topics such as pointers and manual memory management. The Rust compiler is my new best friend as runtime bugs are caught during the compilation stage and it is very noticiable that developer friendliness is one of the major advantages of coding with Rust.

- **Modular Rust Development**: Whilst this being one of my early Rust projects, creating an API that was both modular and reusable was one of the core thoughts I had while developing the project. Each core component of the platform the interpreter runs on exists as its own module and can easily be expanded to add more features to the base Chip-8 functionality.

- **Graphics Programming and Accessing Computer Hardware**: I am not much of a graphics programmer (yet), but I've quite a lot of interest in this field as with video game programming. Using the SDL2 crate bindings for Rust, very little is abstracted away as still a significant amount of code is required to do trivial things such as opening a window on your desktop.

## Getting Started

For those who aren't Rust programmers and/or not familiar with the Chip-8, I recommend checking both [this](https://doc.rust-lang.org/book/) and [this](https://en.wikipedia.org/wiki/CHIP-8) to gain more understanding of this project.

This repo doesn't contain any ROMs (Read Only Memory) which are the instructions to run your desired program for the CJ-8. Check out [this](https://github.com/kripod/chip8-roms) for example ROMs to use with the CJ-8 emulator. Simply download the ROM of choice and add it to the project directory.

### Clone the Repo

```shell
// Clone the repo

git clone https://github.com/CJPohl/CJ-8.git

cd cj-8
```

### Running the CJ-8

```shell
// Run the emulator with required arguments

cargo run {WINDOW_SCALE} {PATH_TO_ROM}

// For example

cargo run 20 ./pong.ch8

// This will initiate a CJ-8 window of 64x32 * (WINDOW_SCALE) and load target ROM from path {PATH_TO_ROM} to memory
```

## Other

### Dependencies

CJ-8 was developed primarily to be not be depended on many 3rd party libraries besides the necessities. But below are the required dependencies for CJ-8 to compile properly on your system.

- [sdl2](https://github.com/Rust-SDL2/rust-sdl2): Sdl2 bindings for Rust
- [rand](https://github.com/rust-random/rand): Random number generator and arithmitic
- [sdl](https://www.libsdl.org/): The orignal SDL required to be installed on your system for cargo to compile

### License

CJ-8 is based upon a CPU architecture and interpretated language that I did not come up with. CJ-8 of course is competely free and open source with an MIT License.

### Further Reading

I would like to say thanks to Tobias V. Langhoff with his amazing write-up [here](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/). It's a detailed run down of the architecture and the system without spoiling the answers to coding the opcodes. It's language agnostic so for anyone interested in writing their own CHIP-8 emulator can start here!

For a more detailed documentation for the Chip-8, [this](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) resource is handy.
