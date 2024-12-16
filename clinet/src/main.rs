use crate::engine::Engine;

mod engine;
mod rendering;

fn main() {
    let mut engine = Engine::new();
    engine.start();
}