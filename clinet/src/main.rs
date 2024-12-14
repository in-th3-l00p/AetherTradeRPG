use crate::engine::Engine;

mod engine;

fn main() {
    let engine = Engine::new();
    engine.start();
}
