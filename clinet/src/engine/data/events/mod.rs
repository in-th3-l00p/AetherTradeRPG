pub mod queue;

use crate::engine::scene::Scene;

/**
    Event driven programming used by different modules of the engine
*/
pub enum Event {
    SceneChange(Box<dyn Scene>)
}
