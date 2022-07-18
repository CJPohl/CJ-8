use sdl2::{Sdl, render::Canvas, pixels::Color, rect::Rect};

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

                self.canvas.set_draw_color(GU::color(col));
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, scale, scale));
            }
        }
        self.canvas.present();
    }

    fn color(i: u16) -> Color {
        if i == 0 {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        }
    }
}