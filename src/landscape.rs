extern crate rand;


const SIZE: usize = 100;


#[derive(Clone, Copy)]
pub struct Cell {
    pub is_alive: bool,
    pub recently_alive: bool,
}


pub struct Landscape {
    pub landscape_size: usize,
    pub cells: [[Cell; SIZE]; SIZE],
}


impl Cell {
    fn new() -> Cell {
        Cell {
            is_alive: false,
            recently_alive: false,
        }
    }
}


impl Landscape {
    pub fn new() -> Landscape {
        let dead_cell = Cell::new();
        let mut landscape = Landscape {
            cells: [[dead_cell; SIZE]; SIZE],
            landscape_size: SIZE,
        };
        
        for i in 0..SIZE {
            for j in 0..SIZE {
                let r = rand::random::<f64>();
                if r > 0.93 {
                    landscape.cells[i][j].is_alive = true;
                }
            }
        }

        landscape
    }

    pub fn toggle_state(&mut self, index: [usize; 2]) {
        self.cells[index[1]][index[0]].is_alive = !self.cells[index[1]][index[0]].is_alive;
    }
}