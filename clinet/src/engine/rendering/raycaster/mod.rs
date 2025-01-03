use std::f32::consts::PI;
use sdl2::pixels::Color;
use crate::engine::data::map::{Cell, Map};
use crate::engine::rendering::raycaster::raypoint::RayPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod raypoint;
mod rayhit;

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
        while angle > 2f32 * PI {
            angle -= 2f32 * PI;
        }
        angle
    }
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        map: &Map,
        point: &RayPoint,
    ) -> Vec<Vec2> {
        let mut rays = vec![];
        let screen_width = canvas.window().size().0;
        for i in 1u32 .. screen_width + 1 {
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

            // calculating dem deltas
            // ich habe paranoide schizofrenie
            let mut horizontal_delta: Vec2 = (
                if current_direction.0 > 0f32 { map.cell_size } else { -map.cell_size },
                0f32
            );
            horizontal_delta.1 = current_angle.tan() * horizontal_delta.0;
            let mut vertical_delta: Vec2 = (
                0f32,
                if current_direction.1 > 0f32 { map.cell_size } else { -map.cell_size }
            );
            vertical_delta.0 = vertical_delta.1 / current_angle.tan();

            // rayzz
            let mut horizontal_ray = (
                point.pos.0 + horizontal_side.0,
                point.pos.1 + horizontal_side.1
            );
            let mut vertical_ray = (
                point.pos.0 + vertical_side.0,
                point.pos.1 + vertical_side.1,
            );

            let offset = (
                if current_direction.0 >= 0f32 { 0f32 } else { -1f32 },
                if current_direction.1 >= 0f32 { 0f32 } else { -1f32 },
            );

            let mut horizontal_hit: Option<(usize, usize)> = None;
            let mut lol = 0;
            while horizontal_hit.is_none() && lol < 5 {
                let map_hit = (
                    ((horizontal_ray.0 + offset.0) / map.cell_size).floor() as i32,
                    (horizontal_ray.1 / map.cell_size).floor() as i32,
                );
                if
                    map_hit.0 >= 0i32 && map_hit.0 < map.size.0 as i32 &&
                    map_hit.1 >= 0i32 && map_hit.1 < map.size.1 as i32
                {
                    let cell = map.data[map_hit.1 as usize][map_hit.0 as usize];
                    if map.cells[&cell] != Cell::Empty {
                        horizontal_hit = Option::from(
                            (map_hit.0 as usize, map_hit.1 as usize)
                        );
                        break;
                    }
                }

                horizontal_ray = (
                    horizontal_ray.0 + horizontal_delta.0,
                    horizontal_ray.1 + horizontal_delta.1,
                );
                lol += 1;
            }

            let mut vertical_hit: Option<(usize, usize)> = None;
            let mut lol = 0;
            while vertical_hit.is_none() && lol < 5 {
                let map_hit = (
                    (vertical_ray.0 / map.cell_size).floor() as i32,
                    ((vertical_ray.1 + offset.1) / map.cell_size).floor() as i32,
                );

                if
                    map_hit.0 >= 0i32 && map_hit.0 < map.size.0 as i32 &&
                    map_hit.1 >= 0i32 && map_hit.1 < map.size.1 as i32
                {
                    let cell = map.data[map_hit.1 as usize][map_hit.0 as usize];
                    if map.cells[&cell] != Cell::Empty {
                        vertical_hit = Option::from(
                            (map_hit.0 as usize, map_hit.1 as usize)
                        );
                        break;
                    }
                }

                vertical_ray = (
                    vertical_ray.0 + vertical_delta.0,
                    vertical_ray.1 + vertical_delta.1,
                );
                lol += 1;
            }

            let horizontal_dist =
                (point.pos.0 - horizontal_ray.0) * (point.pos.0 - horizontal_ray.0) +
                (point.pos.1 - horizontal_ray.1) * (point.pos.1 - horizontal_ray.1);

            let vertical_dist =
                (point.pos.0 - vertical_ray.0) * (point.pos.0 - vertical_ray.0) +
                (point.pos.1 - vertical_ray.1) * (point.pos.1 - vertical_ray.1);

            // rendering
            let mut distance = horizontal_dist;
            if horizontal_dist < vertical_dist {
                rays.push(horizontal_ray);
            } else {
                rays.push(vertical_ray);
                distance = vertical_dist;
            }
            distance *= (point.angle - current_angle).cos();
            let height = canvas.window().size().1 as f32 / distance * map.cell_size;
            canvas.set_draw_color(Color::WHITE);
            canvas
                .draw_line(
                    (i as i32, (canvas.window().size().1 as f32 / 2f32 - height / 2f32) as i32),
                    (i as i32, (canvas.window().size().1 as f32 / 2f32 + height / 2f32) as i32)
                )
                .unwrap();
        }

        rays
    }
}
