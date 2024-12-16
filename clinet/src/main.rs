use crate::engine::Engine;

mod engine;
mod game;

fn main() {
    let mut engine = Engine::new();
    engine.start();
}