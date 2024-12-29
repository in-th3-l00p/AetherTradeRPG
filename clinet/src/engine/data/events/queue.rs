use crate::engine::data::events::Event;
use std::collections::LinkedList;

pub struct EventQueue(LinkedList<Event>);

impl Iterator for EventQueue {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn push(&mut self, event: Event) {
        self.0.push_back(event);
    }

    #[allow(unused)]
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
}
