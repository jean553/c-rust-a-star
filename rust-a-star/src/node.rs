//! The module that handles the nodes.

use piston_window::{
    Rectangle,
    G2d,
};

use graphics::rectangle::Shape;
use graphics::context::Context;

pub const DIMENSION: f64 = 50.0;

pub struct Node {
    surface: Rectangle,
    wall: bool,
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
            wall: wall,
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

        self.surface.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                DIMENSION,
                DIMENSION,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }

    /// Switches the selected node from wall to empty and empty to wall
    pub fn switch(&mut self) {

        self.wall = !self.wall;

        /* FIXME: needs to check why this does not work at all */
        self.surface.color([1.0, 1.0, 1.0, 0.0]);
    }
}

