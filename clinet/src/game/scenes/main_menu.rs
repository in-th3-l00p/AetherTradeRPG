use crate::engine::rendering::scene::Scene;
use imgui::{Condition, WindowFlags};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::Pointer;

pub struct MainMenu {

}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
}

impl Scene for MainMenu {
    fn ui(&self, ui: &mut imgui::Ui) {
        let window_size = [400.0, 300.0];
        let display_size = ui.io().display_size;
        let window_pos = [
            (display_size[0] - window_size[0]) * 0.5,
            (display_size[1] - window_size[1]) * 0.5,
        ];

        // Set window size and position
        ui.window("MainMenu")
            .size(window_size, Condition::Always)
            .position(window_pos, Condition::Always)
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_COLLAPSE | WindowFlags::NO_TITLE_BAR)
            .build(|| {
                // Add spacing for centering
                ui.dummy([0.0, 50.0]);

                // Center the title text
                let title = "AetherTradeRPG";
                let text_size = ui.calc_text_size(title);
                ui.set_cursor_pos([(window_size[0] - text_size[0]) * 0.5, ui.cursor_pos()[1]]);
                ui.text(title);

                ui.dummy([0.0, 20.0]); // Space between title and buttons

                // Center the buttons
                let button_size = [150.0, 50.0];
                let button_spacing = 20.0;

                ui.set_cursor_pos([(window_size[0] - button_size[0]) * 0.5, ui.cursor_pos()[1]]);
                ui.button_with_size("Test Scene", button_size);

                ui.dummy([0.0, button_spacing]); // Space between buttons

                ui.set_cursor_pos([(window_size[0] - button_size[0]) * 0.5, ui.cursor_pos()[1]]);
                if ui.button_with_size("Quit", button_size) {
                    std::process::exit(0);
                }
            });
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
    }
}
