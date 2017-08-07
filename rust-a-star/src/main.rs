//! Main script.

extern crate piston_window;
extern crate graphics;

mod node;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

use node::Node;

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

    /* TODO: add 25 nodes ( 5 x 5 ) */
    let nodes = [
        Node::new(
            true,
            0.0,
            0.0,
        )
    ];

    while let Some(event) = window.next() {
        /* TODO: to define, simply used to maintain the window open */
    }
}
