use rendering::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rendering::scene::Scene;
use crate::game::scenes::main_menu::MainMenu;

pub mod container;
pub mod rendering;

pub struct Engine {
    sdl_context: sdl2::Sdl,
    renderer: Renderer,
    running: bool,
    scene: Box<dyn Scene>,
}

impl Engine {
    pub fn new() -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let renderer = Renderer::new(sdl_context.clone());
        let engine = Engine {
            renderer,
            sdl_context,
            running: true,
            scene: Box::new(MainMenu::new()),
        };

        engine
    }

    pub fn start(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        while (self.running) {
            for event in event_pump.poll_iter() {
                self.renderer.handle_event(&event);
                self.scene.handle_event(&event);
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                        break;
                    }
                    _ => {}
                }
            }

            // todo calculate delta time & update the current scene
            self.renderer.update(&event_pump, &self.scene);
        }
    }
}