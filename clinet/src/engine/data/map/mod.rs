use std::collections::BTreeMap;
use crate::engine::rendering::raycaster::raypoint::RayPoint;

pub enum Cell {
    Empty,
    Wall(u8, u8, u8),
}

pub struct Map {
    pub size: (usize, usize),
    pub data: Vec<Vec<u32>>,
    pub cells: BTreeMap<u32, Cell>,

    // related to the gameplay
    pub cell_size: f32,
    pub spawn_points: ((f32, f32), f32), // position, angle

    // used by the iterator
    current: (usize, usize),
}

impl Map {
    pub fn new(size: (usize, usize)) -> Map {
        Map {
            size,
            data: vec![vec![0; size.1]; size.0],
            cells: BTreeMap::from([ (0, Cell::Empty) ]),
            cell_size: 16f32,
            spawn_points: ((20f32, 20f32), 0f32),
            current: (0, 0),
        }
    }

    // creates a point on the map based on the spawn point
    pub fn create_point(&self) -> RayPoint {
        RayPoint::new(
            self.spawn_points.0,
            self.spawn_points.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let map = Map::new((3, 3));
        assert_eq!(map.size, (3, 3));
        assert_eq!(map.data.len(), 3);
        assert_eq!(map.data.get(0).unwrap().len(), 3);
    }
}