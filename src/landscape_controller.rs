use piston::input::GenericEvent;

use crate::Landscape;


pub struct LandscapeController {
    pub landscape: Landscape,
    pub current_index: [usize; 2],
}


impl LandscapeController {
    pub fn new(landscape: Landscape) -> LandscapeController {
        LandscapeController {
            landscape: landscape,
            current_index: [0; 2],
        }
    }

    fn update_cell(&self, cell_index: [usize; 2]) -> bool {
        let i = cell_index[0];
        let j = cell_index[1];
        let cell_array = self.landscape.cells;
        let size = self.landscape.landscape_size;

        let lower_i = if (i as i8 - 1) >= 0 {i - 1} else {size - 1};
        let lower_j = if (j as i8 - 1) >= 0 {j - 1} else {size - 1};
        let upper_i = if (i as i8 + 1) <= (size - 1) as i8 {i + 1} else {0 as usize};
        let upper_j = if (j as i8+ 1) <= (size - 1) as i8 {j + 1} else {0 as usize};

        let total = cell_array[i][lower_j] as i8 + 
            cell_array[i][upper_j] as i8 +
            cell_array[lower_i][j] as i8 + 
            cell_array[upper_i][j] as i8 +
            cell_array[lower_i][lower_j] as i8 +
            cell_array[lower_i][upper_j] as i8 +
            cell_array[upper_i][lower_j] as i8 +
            cell_array[upper_i][upper_j] as i8;

        if cell_array[i][j] {
            if total < 2 || total > 3 {
                return false
            }
        } else {
            if total == 3 {
                return true
            }
        }

        true
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            let size = self.landscape.landscape_size;
            // Clone cellstate just to get an array with the appropriate shape, it will be overwritten
            let mut new_cell_state = self.landscape.cells.clone();

            for i in 0..size {
                for j in 0..size{
                    new_cell_state[i][j] = self.update_cell([i, j]);
                }
            }

            self.landscape.cells = new_cell_state;

        }
    }

    // pub fn event<E: GenericEvent>(&mut self, e: &E) {
    //     if let Some(_) = e.update_args() {
    //         // Turn off previous cell
    //         self.landscape.toggle_state(self.current_index);

    //         if self.current_index[0] < self.landscape.landscape_size - 1 {
    //             self.current_index[0] += 1;
    //         } else {
    //             self.current_index[0] = 0;
    //             if self.current_index[1] < self.landscape.landscape_size - 1 {
    //                 self.current_index[1] += 1;
    //             } else {
    //                 self.current_index[1] = 0;
    //             }
    //         }

    //         // Turn on new cell
    //         self.landscape.toggle_state(self.current_index);

    //         // println!("{:?}", self.landscape.cells);
    //     }
        
    // }
}