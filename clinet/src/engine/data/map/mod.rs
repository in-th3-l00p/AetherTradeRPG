use std::collections::BTreeMap;

pub enum Cell {
    Empty,
    Wall(u8, u8, u8),
}

pub struct Map {
    pub size: (usize, usize),
    pub data: Vec<Vec<u32>>,
    pub cells: BTreeMap<u32, Cell>,

    // used by the iterator
    current: (usize, usize),
}

impl Map {
    pub fn new(size: (usize, usize)) -> Map {
        Map {
            size,
            data: vec![vec![0; size.1]; size.0],
            cells: BTreeMap::from([ (0, Cell::Empty) ]),
            current: (0, 0),
        }
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