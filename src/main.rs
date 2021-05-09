extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

pub use crate::landscape::Landscape;
pub use crate::landscape_controller::LandscapeController;
pub use crate::landscape_view::{LandscapeView, LandscapeViewSettings};

mod landscape;
mod landscape_controller;
mod landscape_view;


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Cellular Automata", [400; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
    let event_settings: EventSettings = EventSettings::new()
        .max_fps(25)
        .ups(5);
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let mut landscape_controller = LandscapeController::new(Landscape::new());
    let landscape_view = LandscapeView::new(LandscapeViewSettings::new());

    while let Some(e) = events.next(&mut window) {
        landscape_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                landscape_view.draw(&landscape_controller, &c, g);
            });
        }
    }
}
