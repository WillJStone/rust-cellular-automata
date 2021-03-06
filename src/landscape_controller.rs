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

        let total = cell_array[i][lower_j].is_alive as i8 + 
            cell_array[i][upper_j].is_alive as i8 +
            cell_array[lower_i][j].is_alive as i8 + 
            cell_array[upper_i][j].is_alive as i8 +
            cell_array[lower_i][lower_j].is_alive as i8 +
            cell_array[lower_i][upper_j].is_alive as i8 +
            cell_array[upper_i][lower_j].is_alive as i8 +
            cell_array[upper_i][upper_j].is_alive as i8;

        if cell_array[i][j].is_alive {
            if total < 2 || total > 3 {
                return false
            }
            else {
                return true
            }
        } else {
            if total == 3 {
                return true
            } else {
                return false
            }
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            let size = self.landscape.landscape_size;
            // Clone cellstate just to get an array with the appropriate shape, it will be overwritten
            let mut updated_cells = self.landscape.cells.clone();

            for i in 0..size {
                for j in 0..size{
                    let cell_is_alive = self.update_cell([i, j]);
                    updated_cells[i][j].is_alive = cell_is_alive;
                }
            }

            self.landscape.cells = updated_cells;
        }
    }
}