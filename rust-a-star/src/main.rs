//! Main script.

extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

fn main() {

    const WINDOW_WIDTH: u32 = 600;
    const WINDOW_HEIGHT: u32 = 400;

    let mut window: PistonWindow = WindowSettings::new(
        "A* algorithm simulation",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ]
    )
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        /* TODO: to define, simply used to maintain the window open */
    }
}
