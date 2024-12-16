use std::collections::BTreeMap;

pub enum Cell {
    Empty,
    Wall(u8, u8, u8),
}

pub struct Map {
    size: (usize, usize),
    data: Vec<Vec<u32>>,
    cells: BTreeMap<u32, Cell>,

    // used by the iterator
    current: (usize, usize),
}

// iterator
impl Iterator for Map {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.data.get(self.current.0)?.get(self.current.1);

        // going to the next element
        if self.current.1 < self.size.1 {
            self.current.1 += 1;
        } else if self.current.0 < self.size.0 {
            self.current.1 = 0;
            self.current.0 += 1;
        }

        cell.copied()
    }
}

impl Map {
    pub fn new(size: (usize, usize)) -> Map {
        Map {
            size,
            data: vec![vec![0; size.1]; size.0],
            cells: BTreeMap::from([ (0, Cell::Wall(255, 255, 255)) ]),
            current: (0, 0),
        }
    }

    pub fn get_cell(&self, cell_id: u32) -> Option<&Cell> {
        self.cells.get(&cell_id)
    }
}