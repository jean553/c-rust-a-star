//! Main script.

extern crate piston_window;
extern crate graphics;
extern crate sprite;

use std::mem;
use std::rc::Rc;

use piston_window::{
    G2d,
    PistonWindow,
    WindowSettings,
    clear,
    MouseCursorEvent,
    MouseButton,
    PressEvent,
    Button,
    Texture,
    TextureSettings,
    Flip,
};

use sprite::{
    Scene,
    Sprite,
};

use graphics::context::Context;

mod node;
mod display;

use node::{
    DIMENSION,
    Node,
};

use display::Display;

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
                horizontal_position,
                vertical_position,
            );


            horizontal_position += DIMENSION;
        }
    }

    let pin_surface = Rc::new(Texture::from_path(
        &mut window.factory,
        "res/pin.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap());

    let mut scene: Scene<_> = Scene::new();

    let mut mouse_horizontal_position: f64 = 0.0;
    let mut mouse_vertical_position: f64 = 0.0;

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                clear_screen(graphics);

                display_nodes(
                    &nodes,
                    context,
                    graphics,
                );

                scene.draw(
                    context.transform,
                    graphics,
                );
            }
        );

        if let Some(position) = event.mouse_cursor_args() {
            mouse_horizontal_position = position[0];
            mouse_vertical_position = position[1];
        }

        if let Some(button) = event.press_args() {

            let mut index = 0;

            let mut horizontal_position: f64 = 50.0;
            while horizontal_position < mouse_horizontal_position {
                horizontal_position += DIMENSION;
                index += 1;
            }

            let mut vertical_position: f64 = 50.0;
            while vertical_position < mouse_vertical_position {
                vertical_position += DIMENSION;
                index += 5;
            }

            if button == Button::Mouse(MouseButton::Left) {
                nodes[index].switch();
            }
            else if button == Button::Mouse(MouseButton::Right) {

                let mut sprite = Sprite::from_texture(pin_surface.clone());

                const PIN_OFFSET: f64 = 25.0;
                let pin_horizontal_position: f64 =
                    horizontal_position - PIN_OFFSET;
                let pin_vertical_position: f64 =
                    vertical_position - PIN_OFFSET;

                sprite.set_position(
                    pin_horizontal_position,
                    pin_vertical_position,
                );

                scene.add_child(sprite);
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

        /* graphics::rectangle::Rectangle::color() does nothing,
           so that's why I do not simply update the color
           but use a boolean instead */
        if !node.is_wall() {
            continue;
        }

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
