//! Main script.

extern crate piston_window;
extern crate graphics;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Rectangle,
};

use graphics::rectangle;

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

    const NODES_AMOUNT_ON_MAP: usize = 25;
    let nodes = [
        Rectangle {
            color: [0.0, 0.0, 0.3, 1.0], /* blue */
            shape: rectangle::Shape::Square,
            border: None,
        }; NODES_AMOUNT_ON_MAP
    ];

    while let Some(event) = window.next() {
        /* TODO: to define, simply used to maintain the window open */
    }
}
