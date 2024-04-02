use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Key, MouseButton};

use crate::util::Point;

pub trait State {
    //fn mouse_event(&self);
    fn keypress_event(&mut self, key: Key);
    fn keyrelease_event(&mut self, key: Key);

    fn mouse_press_event(&mut self, button: MouseButton);
    fn mouse_release_event(&mut self, button: MouseButton);
    fn mouse_position_event(&mut self, position: Point<f32>);

    fn update(&mut self);
    fn render(&mut self, c: Context, g: &mut GlGraphics);
}
