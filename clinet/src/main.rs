use crate::engine::Engine;

mod engine;
mod rendering;

fn main() {
    let engine = Engine::new();
    engine.start();
}
