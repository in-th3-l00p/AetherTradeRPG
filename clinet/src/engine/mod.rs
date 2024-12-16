use crate::engine::container::ECSContainer;
use crate::rendering::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod container;

pub struct Engine {
    sdl_context: sdl2::Sdl,
    ecs: ECSContainer,
    renderer: Renderer,

    running: bool,
}

impl Engine {
    pub fn new() -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let renderer = Renderer::new(sdl_context.clone());
        let engine = Engine {
            ecs: ECSContainer::new(),
            renderer,
            sdl_context,
            running: true,
        };

        engine
    }

    pub fn start(&mut self) {
        while (self.running) {
            for event in self.sdl_context.event_pump().unwrap().poll_iter() {
                self.renderer.handle_event(&event);
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