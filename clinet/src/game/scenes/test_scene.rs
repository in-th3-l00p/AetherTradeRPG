use crate::engine::data::events::queue::EventQueue;
use crate::engine::scene::Scene;

pub struct TestScene {
    event_queue: EventQueue,
}

impl TestScene {
    pub fn new() -> Self {
        TestScene {
            event_queue: EventQueue::new(),
        }
    }
}

impl Scene for TestScene {
    fn events(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }
}