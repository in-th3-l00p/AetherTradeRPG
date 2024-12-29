use imgui::{Condition, Ui, WindowFlags};
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::engine::data::events::queue::EventQueue;
use crate::engine::data::map::{Cell, Map};
use crate::engine::rendering::raycaster::raypoint::RayPoint;
use crate::engine::rendering::raycaster::Raycaster;
use crate::engine::scene::Scene;

// fix this bs memory management
pub struct TestScene {
    event_queue: EventQueue,
    map: Map,
    point: RayPoint,
    raycaster: Raycaster
}

impl TestScene {
    pub fn new() -> Self {
        let map = Map::create_test_map();
        let point = map.create_point();
        TestScene {
            event_queue: EventQueue::new(),
            map, point,
            raycaster: Raycaster::new()
        }
    }
}

impl Scene for TestScene {
    fn ui(&mut self, ui: &mut Ui) {
        ui
            .window("mini map")
            .size([408.0, 430.0], Condition::FirstUseEver)
            .flags(WindowFlags::NO_RESIZE)
            .build(|| {
                let mut pos = ui.window_pos();
                pos[0] += 4.0; // offsets for the top-left corner of the window (without borders)
                pos[1] += 20.0;
                let cell_size = (ui.window_size()[0]) / self.map.size.0 as f32;

                for y in 0..self.map.size.1 {
                    for x in 0..self.map.size.0 {
                        match self.map.cells.get(&self.map.data[y][x]).unwrap() {
                            Cell::Wall(r, g, b) => {
                                ui
                                    .get_window_draw_list()
                                    .add_rect_filled_multicolor(
                                        [pos[0] + x as f32 * cell_size, pos[1] + y as f32 * cell_size],
                                        [
                                            pos[0] + x as f32 * cell_size + cell_size,
                                            pos[1] + y as f32 * cell_size + cell_size
                                        ],
                                        [*r as f32, *g as f32, *b as f32],
                                        [*r as f32, *g as f32, *b as f32],
                                        [*r as f32, *g as f32, *b as f32],
                                        [*r as f32, *g as f32, *b as f32],
                                    );
                            }
                            _ => {}
                        }
                    }
                }

                ui
                    .get_window_draw_list()
                    .add_circle(
                        [
                            pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size),
                            pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size)
                        ],
                        5.0,
                        [1.0, 0.0, 0.0]
                    )
                    .build();

                const DIRECTION_MULTIPLIER: f32 = 20.0;
                ui
                    .get_window_draw_list()
                    .add_line(
                        [
                            pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size),
                            pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size)
                        ],
                        [
                            pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size) + self.point.angle.cos() * DIRECTION_MULTIPLIER,
                            pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size) + self.point.angle.sin() * DIRECTION_MULTIPLIER
                        ],
                        [1.0, 0.0, 0.0]
                    )
                    .build();
            });
    }

    fn update(&mut self, event_pump: &EventPump, delta_time: &f32) {
        let keys = event_pump.keyboard_state();
        let velocity: f32 = 0.5 * delta_time;
        let rotate_speed: f32 = 0.02 * delta_time;
        if keys.is_scancode_pressed(Scancode::W)
        {
            self.point.pos.0 += velocity * delta_time * self.point.angle.cos();
            self.point.pos.1 += velocity * delta_time * self.point.angle.sin();
        }
        if keys.is_scancode_pressed(Scancode::A) {
            self.point.rotate(-rotate_speed * delta_time);
        }
        if keys.is_scancode_pressed(Scancode::S)
        {
            self.point.pos.0 -= velocity * delta_time * self.point.angle.cos();
            self.point.pos.1 -= velocity * delta_time * self.point.angle.sin();
        }
        if keys.is_scancode_pressed(Scancode::D) {
            self.point.rotate(rotate_speed * delta_time);
        }
    }

    fn render(&self, _canvas: &mut Canvas<Window>) {
        self.raycaster.render(_canvas, &self.point, &self.map)
    }

    fn events(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }
}