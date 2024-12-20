use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::engine::data::map::Map;
use crate::engine::rendering::raycaster::point::Point;

pub mod point;

pub struct Raycaster<'game> {
    point: &'game Point,
    map: &'game Map
}

impl<'game> Raycaster<'game> {
    pub fn new(point: &'game Point, map: &'game Map) -> Self {
        Raycaster { point, map }
    }

    fn update(&mut self, _delta_time: &f32) {

    }

    pub fn render(&self, _canvas: &mut Canvas<Window>) {

    }
}