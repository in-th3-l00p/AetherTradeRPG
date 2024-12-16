use crate::engine::rendering::scene::Scene;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::Pointer;
use imgui::Condition::FirstUseEver;

pub struct MainMenu {

}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
}

impl Scene for MainMenu {
    fn ui(&self, ui: &mut imgui::Ui) {
        ui
            .window("hello")
            .size([300.0, 100.0], FirstUseEver)
            .build(|| {
                ui.text("Hello World!");
            });
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
    }
}
