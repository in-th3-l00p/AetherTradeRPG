use std::rc::Rc;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::engine::data::map::Map;
use crate::engine::rendering::raycaster::raypoint::RayPoint;

pub mod raypoint;

pub struct Raycaster {
    point: Rc<RayPoint>,
    map: Rc<Map>
}

impl Raycaster {
    pub fn new(point: Rc<RayPoint>, map: Rc<Map>) -> Self {
        Raycaster { point, map }
    }

    pub fn render(&self, _canvas: &mut Canvas<Window>) {

    }
}