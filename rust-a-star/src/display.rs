//! The trait for displaying purposes.

use piston_window::G2d;
use graphics::context::Context;

pub trait Display {

    fn display(
        &self,
        context: Context,
        graphics: &mut G2d,
    );
}
