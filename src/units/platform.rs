use sdl2::Sdl;

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
