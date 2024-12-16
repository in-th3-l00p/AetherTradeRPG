use sdl2::video::{GLContext, GLProfile, SwapInterval, Window};
use imgui_glow_renderer::{glow::{self}, AutoRenderer};
use imgui_glow_renderer::glow::HasContext;
use imgui_sdl2_support::SdlPlatform;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::{EventPump, Sdl};

// default settings
// todo: put stuff like this in a config file
const WINDOW_TITLE: &str = "lol";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

// the "Rendering" submodule
// handles the entire sdl3 rendering
pub struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    imgui: imgui::Context,
    imgui_platform: SdlPlatform,
    imgui_renderer: AutoRenderer,
}

// function for creating a glow context from sdl
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(
            |s| window.subsystem().gl_get_proc_address(s) as _
        )
    }
}

impl Renderer {
    pub fn new(sdl_context: Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        // gl stuff
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        let canvas = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build().unwrap()
            .into_canvas()
            .build().unwrap();
        canvas
            .window()
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        let gl = glow_context(&canvas.window());

        // initializing imgui
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let mut platform = SdlPlatform::new(&mut imgui);
        let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();

        Renderer {
            canvas: canvas,
            sdl_context,
            imgui,
            imgui_platform: platform,
            imgui_renderer: renderer,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.imgui_platform.handle_event(&mut self.imgui, event);
    }

    // runs in the game loop
    pub fn update(&mut self, event_pump: &EventPump) {
        // imgui
        self.imgui_platform.prepare_frame(
            &mut self.imgui,
            &self.canvas.window(),
            event_pump
        );
        let ui = self.imgui.frame();
        ui.show_demo_window(&mut true);
        let draw_data = self.imgui.render();

        // clear
        unsafe { self.imgui_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(Rect::from((100, 100, 100, 100))).unwrap();
        self.imgui_renderer.render(draw_data).unwrap();
        self.canvas.present();
    }
}