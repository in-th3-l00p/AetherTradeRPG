use crate::engine::data::events::Event;
use crate::engine::scene::Scene;
use crate::game::scenes::main_menu::MainMenu;
use rendering::Renderer;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;

pub mod data;
pub mod rendering;
pub mod scene;

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
            running: false,
            scene: Box::new(MainMenu::new()),
        };

        engine
    }

    fn handle_scene_event(&mut self, event: Event) {
        match (event) {
            Event::SceneChange(scene) => self.scene = scene,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .unwrap();
        let delta_time = 1f32; // todo proper calculation
        while (self.running) {
            for event in event_pump.poll_iter() {
                self.renderer.handle_event(&event);
                self.scene.handle_sdl_event(&event);
                match event {
                    SdlEvent::Quit { .. } |
                    SdlEvent::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                        break;
                    }
                    _ => {}
                }
            }

            self.scene.update(&delta_time);
            self.renderer.update(&event_pump, &mut self.scene);

            // handle scene events
            // todo find out how to use iterators in this
            let mut event = self.scene.events().next();
            while event.is_some() {
                self.handle_scene_event(event.unwrap());
                event = self.scene.events().next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Engine;

    #[test]
    fn new_works() {
        let engine = Engine::new();
        assert!(!engine.running);
    }
}