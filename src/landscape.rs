extern crate rand;


const SIZE: usize = 100;


pub struct Landscape {
    pub landscape_size: usize,
    pub cells: [[bool; SIZE]; SIZE],
}


impl Landscape {
    pub fn new() -> Landscape {
        let mut landscape = Landscape {
            cells: [[false; SIZE]; SIZE],
            landscape_size: SIZE,
        };
        
        for i in 0..SIZE {
            for j in 0..SIZE {
                let r = rand::random::<f64>();
                if r > 0.93 {
                    landscape.cells[i][j] = true;
                }
            }
        }

        landscape
    }

    pub fn toggle_state(&mut self, index: [usize; 2]) {
        self.cells[index[1]][index[0]] = !self.cells[index[1]][index[0]];
    }
}