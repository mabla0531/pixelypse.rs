use sfml::{graphics::RenderWindow, system::Vector2f, window::{mouse::Button, Key}};

pub trait State {
    //fn mouse_event(&self);
    fn keypress_event(&mut self, key: Key);
    fn keyrelease_event(&mut self, key: Key);

    fn mouse_press_event(&mut self, button: Button);
    fn mouse_release_event(&mut self, button: Button);
    fn mouse_position_event(&mut self, position: Vector2f);

    fn update(&mut self);
    fn render(&mut self, window: &mut RenderWindow);
}
