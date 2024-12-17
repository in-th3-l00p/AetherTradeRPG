use crate::engine::data::map::{Cell, Map};
use std::collections::BTreeMap;

const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;

pub fn create_test_map() -> Map {
    let mut map = Map::new((MAP_WIDTH, MAP_HEIGHT));
    map.data = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];
    map.cells = BTreeMap::from([
        (0, Cell::Empty),
        (1, Cell::Wall(255, 255, 255))
    ]);
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn creating_test_map_works() {
        let map = create_test_map();

        assert_eq!(map.data.len(), MAP_HEIGHT);
        assert_eq!(map.data[0].len(), MAP_WIDTH);
    }
}