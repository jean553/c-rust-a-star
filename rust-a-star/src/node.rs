//! The module that handles the nodes.

use piston_window::G2d;

use graphics::rectangle::{
    Shape,
    Rectangle,
};
use graphics::context::Context;

pub const DIMENSION: f64 = 50.0;

pub struct Node {
    surface: Rectangle,
    wall: bool,
    departure: bool,
    arrival: bool,
    horizontal_position: f64,
    vertical_position: f64,
    heuristic: u8,
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
        horizontal_position: f64,
        vertical_position: f64,
    ) -> Node {

        Node {
            surface: Rectangle {
                color: [0.0, 0.0, 0.3, 1.0], /* blue */
                shape: Shape::Square,
                border: None,
            },
            wall: false,
            departure: false,
            arrival: false,
            horizontal_position: horizontal_position,
            vertical_position: vertical_position,
            heuristic: 0,
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
    }

    /// Indicates if the node is a wall or not
    ///
    /// # Returns:
    ///
    /// boolean indicating if the node is a wall
    pub fn is_wall(&self) -> bool {
        self.wall
    }

    /// Setter for the node heuristic.
    ///
    /// # Arguments:
    ///
    /// * `heuristic` - the expected heuristic to set
    pub fn set_heuristic(
        &mut self,
        heuristic: u8,
    ) {
        self.heuristic = heuristic;
    }
}
