//! Main script.

extern crate piston_window;
extern crate graphics;

mod node;

use piston_window::{
    G2d,
    PistonWindow,
    WindowSettings,
    clear,
};

use graphics::context::Context;

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

        window.draw_2d(
            &event,
            |context, graphics| {

                display_nodes(
                    &nodes,
                    context,
                    graphics,
                );

                clear_screen(graphics);
            }
        );
    }
}

/// Displays all nodes.
///
/// # Arguments:
///
/// * `nodes` - the array of nodes to display
/// * `context` - graphical context from the piston window
/// * `graphics` - 2D graphics from the piston window
fn display_nodes(
    nodes: &[Node; 1],
    context: Context,
    graphics: &mut G2d,
) {

    for node in nodes {
        node.display(
            context,
            graphics,
        );
    }
}

/// Clear the whole screen.
///
/// # Arguments:
///
/// * `graphics` - 2D graphics from the piston window
fn clear_screen(graphics: &mut G2d) {

    const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    clear(
        BLACK_COLOR,
        graphics,
    );
}
