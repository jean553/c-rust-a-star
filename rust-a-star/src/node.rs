//! The module that handles the nodes.

use piston_window::{
    Rectangle,
    G2d,
};

use graphics::rectangle::Shape;
use graphics::context::Context;

pub struct Node {
    surface: Rectangle,
    horizontal_position: f64,
    vertical_position: f64,
}

impl Node {

    /// Creates a new node.
    ///
    /// # Arguments:
    ///
    /// * `wall` - can the AI go through this node
    /// * `horizontal_position` - the horizontal position of the cell
    /// * `vertical_position` - the vertical position of the cell
    pub fn new(
        wall: bool,
        horizontal_position: f64,
        vertical_position: f64,
    ) -> Node {

        let mut color = [0.0, 0.0, 0.3, 1.0]; /* blue */

        if !wall {
            color = [1.0, 1.0, 1.0, 1.0]; /* white */
        }

        Node {
            surface: Rectangle {
                color: color,
                shape: Shape::Square,
                border: None,
            },
            horizontal_position: horizontal_position,
            vertical_position: vertical_position,
        }
    }

    /// Displays the node.
    ///
    /// # Arguments:
    ///
    /// * `context` - the context of the piston window
    /// * `graphics` - 2D graphics from the piston window
    pub fn display(
        &self,
        context: Context,
        graphics: &mut G2d,
    ) {

        const WIDTH: f64 = 50.0;
        const HEIGHT: f64 = 50.0;
        self.surface.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                WIDTH,
                HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}

