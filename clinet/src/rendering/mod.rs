use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

// default settings
// todo: put stuff like this in a config file
const WINDOW_TITLE: &str = "lol";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

// the "Rendering" submodule
// handles the entire sdl3 rendering
pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(sdl_context: &sdl3::Sdl) -> Self {
        Renderer {
            canvas: sdl_context
                .video()
                .unwrap()
                .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
                .position_centered()
                .resizable()
                .build().unwrap()
                .into_canvas()
        }
    }

    // runs in the game loop
    pub fn update(&mut self) {
        // clearing
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // rendering objects
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)).unwrap();

        // updating
        self.canvas.present();
    }
}