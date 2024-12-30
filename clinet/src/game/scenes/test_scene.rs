use crate::engine::data::events::queue::EventQueue;
use crate::engine::data::map::{Cell, Map};
use crate::engine::rendering::raycaster::raypoint::RayPoint;
use crate::engine::rendering::raycaster::Raycaster;
use crate::engine::scene::Scene;
use imgui::{Condition, Ui, WindowFlags};
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

// fix this bs memory management
pub struct TestScene {
    event_queue: EventQueue,
    map: Map,
    point: RayPoint,
    raycaster: Raycaster,
    last_rays: Vec<(f32, f32)>,
}

impl TestScene {
    pub fn new() -> Self {
        let map = Map::create_test_map();
        let point = map.create_point();
        TestScene {
            event_queue: EventQueue::new(),
            map,
            point,
            raycaster: Raycaster::new(90f32.to_radians(), 64f32),
            last_rays: vec![],
        }
    }
}

// ui implementations
impl TestScene {
    fn ui_minimap(&mut self, ui: &mut Ui) {
        ui.window("mini map")
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
                                ui.get_window_draw_list().add_rect_filled_multicolor(
                                    [pos[0] + x as f32 * cell_size, pos[1] + y as f32 * cell_size],
                                    [
                                        pos[0] + x as f32 * cell_size + cell_size,
                                        pos[1] + y as f32 * cell_size + cell_size,
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

                ui.get_window_draw_list()
                    .add_circle(
                        [
                            pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size),
                            pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size),
                        ],
                        5.0,
                        [1.0, 0.0, 0.0],
                    )
                    .build();

                const DIRECTION_MULTIPLIER: f32 = 20.0;
                ui.get_window_draw_list()
                    .add_line(
                        [
                            pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size),
                            pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size),
                        ],
                        [
                            pos[0]
                                + self.point.pos.0 * (cell_size / self.map.cell_size)
                                + self.point.angle.cos() * DIRECTION_MULTIPLIER,
                            pos[1]
                                + self.point.pos.1 * (cell_size / self.map.cell_size)
                                + self.point.angle.sin() * DIRECTION_MULTIPLIER,
                        ],
                        [1.0, 0.0, 0.0],
                    )
                    .build();

                for last_ray in self.last_rays.iter() {
                    ui.get_window_draw_list()
                        .add_line(
                            [
                                pos[0] + self.point.pos.0 * (cell_size / self.map.cell_size),
                                pos[1] + self.point.pos.1 * (cell_size / self.map.cell_size),
                            ],
                            [
                                pos[0] + last_ray.0 * (cell_size / self.map.cell_size),
                                pos[1] + last_ray.1 * (cell_size / self.map.cell_size),
                            ],
                            [1.0, 0.0, 0.0],
                        )
                        .build();
                }
            });
    }

    fn ui_data(&mut self, ui: &mut Ui) {
        ui.window("data")
            .position([500.0, 80.0], Condition::FirstUseEver)
            .size([220.0, 140.0], Condition::FirstUseEver)
            .build(|| {
                let x_str = String::from("Player x: ") + &*self.point.pos.0.to_string();
                let y_str = String::from("Player y: ") + &*self.point.pos.1.to_string();
                let angle_str =
                    String::from("Player angle (rad): ") + &*self.point.angle.to_string();
                let angle_deg_str = String::from("Player angle (deg): ")
                    + &*self.point.angle.to_degrees().to_string();

                let rays_count = String::from("Rays count: ") + &*self.last_rays.len().to_string();

                ui.text(x_str);
                ui.text(y_str);
                ui.text(angle_str);
                ui.text(angle_deg_str);
                ui.text(rays_count);
            });
    }
}

impl Scene for TestScene {
    fn ui(&mut self, ui: &mut Ui) {
        self.ui_minimap(ui);
        self.ui_data(ui);
    }

    fn update(&mut self, event_pump: &EventPump, delta_time: &f32) {
        let keys = event_pump.keyboard_state();
        let velocity: f32 = 0.5 * delta_time;
        let rotate_speed: f32 = 0.02 * delta_time;
        if keys.is_scancode_pressed(Scancode::W) {
            self.point.pos.0 += velocity * delta_time * self.point.angle.cos();
            self.point.pos.1 += velocity * delta_time * self.point.angle.sin();
        }
        if keys.is_scancode_pressed(Scancode::A) {
            self.point.rotate(-rotate_speed * delta_time);
        }
        if keys.is_scancode_pressed(Scancode::S) {
            self.point.pos.0 -= velocity * delta_time * self.point.angle.cos();
            self.point.pos.1 -= velocity * delta_time * self.point.angle.sin();
        }
        if keys.is_scancode_pressed(Scancode::D) {
            self.point.rotate(rotate_speed * delta_time);
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.last_rays = self.raycaster.render(canvas, &self.map, &self.point);
    }

    fn events(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }
}
