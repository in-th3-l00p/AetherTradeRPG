use imgui::{Condition, Ui, WindowFlags};
use crate::engine::data::events::queue::EventQueue;
use crate::engine::data::map::{Cell, Map};
use crate::engine::scene::Scene;

pub struct TestScene {
    event_queue: EventQueue,
    map: Map
}

impl TestScene {
    pub fn new() -> Self {
        TestScene {
            event_queue: EventQueue::new(),
            map: Map::create_test_map()
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
            });
    }

    fn events(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }
}