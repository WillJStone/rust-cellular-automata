use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::landscape_controller::LandscapeController;


pub struct LandscapeViewSettings {
    pub size: f64,
    pub dead_cell_color: Color,
    pub live_cell_color: Color,
}


impl LandscapeViewSettings {
    pub fn new() -> LandscapeViewSettings {
        LandscapeViewSettings {
            size: 100.0,
            live_cell_color: [0.0, 0.0, 0.0, 1.0],
            dead_cell_color: [1.0, 1.0, 1.0 , 1.0],
        }
    }
}


pub struct LandscapeView {
    pub settings: LandscapeViewSettings,
}


impl LandscapeView {
    pub fn new(landscape_view_settings: LandscapeViewSettings) -> LandscapeView {
        LandscapeView {
            settings: landscape_view_settings,
        }
    }

    pub fn draw<G: Graphics>(&self, controller: &LandscapeController, c: &Context, g: &mut G) {
        use graphics::{Rectangle, Transformed, rectangle};

        let ref settings = self.settings;
        let live_cell = Rectangle::new(settings.live_cell_color);
        let dead_cell = Rectangle::new(settings.dead_cell_color);

        let transform = c
                .transform
                .trans(50.0, 50.0);

        for j in 0..controller.landscape.landscape_size {
            for i in 0..controller.landscape.landscape_size {
                let square = rectangle::square((i * 4) as f64, (j * 4) as f64, 4.0);

                if controller.landscape.cells[j][i] {
                    live_cell.draw(square, &c.draw_state, transform, g);
                } else {
                    dead_cell.draw(square, &c.draw_state, transform, g);
                }
            }
        }
    }
}