// pub mod graphical_unit {
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

        let mut canvas = window.into_canvas().build().unwrap();

        GU { canvas }
    }
}
// }
