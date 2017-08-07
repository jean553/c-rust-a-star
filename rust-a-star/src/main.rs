//! Main script.

extern crate piston_window;
extern crate graphics;

use std::mem;

use piston_window::{
    G2d,
    PistonWindow,
    WindowSettings,
    clear,
    MouseCursorEvent,
    MouseButton,
    PressEvent,
    Button,
};

use graphics::context::Context;

mod node;

use node::{
    DIMENSION,
    Node,
};

fn main() {

    const WINDOW_WIDTH: u32 = 250;
    const WINDOW_HEIGHT: u32 = 250;

    let mut window: PistonWindow = WindowSettings::new(
        "A* algorithm simulation",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ]
    )
    .build()
    .unwrap();

    const NODES_AMOUNT: usize = 25;
    let mut nodes: [Node; NODES_AMOUNT] = unsafe { mem::uninitialized() };

    {
        const DEFAULT_POSITION: f64 = 0.0;
        let mut horizontal_position: f64 = DEFAULT_POSITION;
        let mut vertical_position: f64 = DEFAULT_POSITION;

        for index in 0..NODES_AMOUNT {

            const NODES_PER_ROW: usize = 5;
            if index % NODES_PER_ROW == 0 && index != 0 {
                horizontal_position = DEFAULT_POSITION;
                vertical_position += DIMENSION;
            }

            nodes[index] = Node::new(
                true,
                horizontal_position,
                vertical_position,
            );


            horizontal_position += DIMENSION;
        }
    }

    let mut index: usize = 0;

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

        if let Some(position) = event.mouse_cursor_args() {

            index = 0;

            let mut horizontal_position: f64 = 50.0;
            while horizontal_position < position[0] {
                horizontal_position += DIMENSION;
                index += 1;
            }

            let mut vertical_position: f64 = 50.0;
            while vertical_position < position[1] {
                vertical_position += DIMENSION;
                index += 5;
            }
        }

        if let Some(button) = event.press_args() {

            if button == Button::Mouse(MouseButton::Left) {
                nodes[index].switch();
            }
        }
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
    nodes: &[Node; 25],
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
