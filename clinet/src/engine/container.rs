use std::collections::BTreeMap;

// required implementation functions for an entity
pub trait Entity {
    fn new() -> Self where Self: Sized;
    fn update(&mut self, delta_time: &f32) where Self: Sized;
    fn render(&self) where Self: Sized;
}

// the ecs itself
pub struct ECSContainer {
    entities: Vec<Box<dyn Entity>>,
    names: BTreeMap<String, usize>
}


impl ECSContainer {
    fn new() -> ECSContainer {
        ECSContainer {
            entities: Vec::new(),
            names: BTreeMap::new(),
        }
    }

    fn add(&mut self, name: &str, entity: Box<dyn Entity>) {
        self.entities.push(entity);
        self.names.insert(
            name.to_string(),
            self.entities.len() - 1
        );
    }

    fn get(&self, name: &str) -> Option<&Box<dyn Entity>> {
        self.entities.get(*self.names.get(name)?)
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(*self.names.get(name)?)
    }

    fn remove(&mut self, name: &str) {
        self.names.remove(name);
    }
}
