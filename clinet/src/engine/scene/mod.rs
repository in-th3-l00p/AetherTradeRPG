use crate::engine::data::events::queue::EventQueue;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Scene {
    fn handle_sdl_event(&mut self, _event: &Event) {}
    fn ui(&mut self, _ui: &mut imgui::Ui) { }
    fn update(&mut self, _event_pump: &EventPump, _delta_time: &f32) {}
    fn render(&mut self, _canvas: &mut Canvas<Window>) {}
    fn events(&mut self) -> &mut EventQueue;
}