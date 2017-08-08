//! The module that handles the pin.

use piston_window::{
    PistonWindow,
    Texture,
    TextureSettings,
    Flip,
    G2d,
};

use graphics::context::Context;

use gfx_device_gl::Resources;

use display::Display;

pub struct Pin {
    texture: Texture<Resources>,
    horizontal_position: f64,
    vertical_position: f64,
}

impl Display for Pin {

    /// Displays the pin.
    ///
    /// # Arguments:
    ///
    /// * `context` - the context of the piston window
    /// * `graphics` - 2D graphics from the piston window
    ///
    /// TODO: to define
    fn display(
        &self,
        context: Context,
        graphics: &mut G2d,
    ) {
    }
}

impl Pin {

    /// Creates a new pin.
    ///
    /// # Arguments:
    ///
    /// * `window` - the piston window to use
    /// * `horizontal_position` - the horizontal position of the pin
    /// * `vertical_position` - the vertical position of the pin
    ///
    /// # Returns:
    ///
    /// the new created pin
    pub fn new(
        window: &mut PistonWindow,
        horizontal_position: f64,
        vertical_position: f64,
    ) -> Pin {
        Pin {
            texture: Texture::from_path(
                &mut window.factory,
                "res/pin.png",
                Flip::None,
                &TextureSettings::new(),
            ).unwrap(),
            horizontal_position: horizontal_position,
            vertical_position: vertical_position,
        }
    }
}
