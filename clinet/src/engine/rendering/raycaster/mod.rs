use crate::engine::data::map::Map;
use crate::engine::rendering::raycaster::raypoint::RayPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod raypoint;

pub struct Raycaster {}

impl Raycaster {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self, _canvas: &mut Canvas<Window>,
        _point: &RayPoint,
        _map: &Map)
    {}
}
