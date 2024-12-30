use std::f32::consts::PI;
use crate::engine::data::map::Map;
use crate::engine::rendering::raycaster::raypoint::RayPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod raypoint;

pub struct Raycaster {
    fov: f32,
    render_distance: f32,
}

// it made me angry to write (f32, f32) so many times
type Vec2 = (f32, f32);

impl Raycaster {
    pub fn new(fov: f32, render_distance: f32) -> Self {
        Self { fov, render_distance, }
    }

    pub fn clamp_angle(mut angle: f32) -> f32 {
        while angle < 0f32 {
            angle += 2f32 * PI;
        }
        while angle >= 2f32 * PI {
            angle -= 2f32 * PI;
        }
        angle
    }

    // calculates what it should render using the DDA algorithm
    pub fn calculate(&self, map: &Map, point: &RayPoint, screen_width: u32) -> (f32, f32) {
        for i in (1u32 .. screen_width + 1) {
            // calculating the current ray direction
            let current_ray: f32 = (i as f32 / (screen_width as f32)) * self.fov; // the current ray offset
            let current_angle: f32 = Self::clamp_angle(point.angle - self.fov / 2f32 + current_ray);
            let current_direction = (
                current_angle.cos(),
                current_angle.sin()
            );

            // calculate the offset where the player is
            let map_cell = (
                (point.pos.0 / map.cell_size).floor(),
                (point.pos.1 / map.cell_size).floor()
            );

            // calculate the first line hits
            let mut horizontal_side: Vec2 = (
                (
                    map_cell.0 +
                    if current_direction.0 > 0f32 { 1f32 } else { 0f32 }
                ) *
                    map.cell_size - point.pos.0,
                0_f32
            );
            horizontal_side.1 = current_angle.tan() * horizontal_side.0;
            let mut vertical_side: Vec2 = (
                0_f32,
                (
                    map_cell.1 +
                    if current_direction.1 > 0f32 { 1f32 } else { 0f32 }
                ) *
                    map.cell_size - point.pos.1
            );
            vertical_side.0 = vertical_side.1 / current_angle.tan();

            // rayzz
            let horizontal_ray = (
                point.pos.0 + horizontal_side.0,
                point.pos.1 + horizontal_side.1
            );
            let vertical_ray = (
                point.pos.0 + vertical_side.0,
                point.pos.1 + vertical_side.1
            );

            let hit = true;
            while !hit {

            }

            if i == screen_width / 2 {
                return vertical_ray;
            }
        }

        (0f32, 0f32)
    }

    pub fn render(
        &self, _canvas: &mut Canvas<Window>,
        _point: &RayPoint,
        _map: &Map)
    {

    }
}
