use sdl3::event::Event;
use sdl3::event::Event::Window;
use sdl3::keyboard::Keycode;
use crate::engine::container::ECSContainer;
use crate::rendering::Renderer;

mod container;

pub struct Engine {
    sdl_context: sdl3::Sdl,
    ecs: ECSContainer,
    renderer: Renderer,

    running: bool,
}

impl Engine {
    pub fn new() -> Engine {
        let sdl_context = sdl3::init().unwrap();
        let engine = Engine {
            ecs: ECSContainer::new(),
            renderer: Renderer::new(&sdl_context),
            sdl_context,
            running: true,
        };

        engine
    }

    pub fn start(&mut self) {
        while (self.running) {
            for event in self.sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                        break;
                    }
                    _ => {}
                }
            }

            self.renderer.update();
        }
    }
}