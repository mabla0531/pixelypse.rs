use sfml::{graphics::RenderWindow, window::Key};

pub trait State {
    //fn mouse_event(&self);
    fn keypress_event(&mut self, key: Key);
    fn keyrelease_event(&mut self, key: Key);
    fn update(&mut self);
    fn render(&mut self, window: &mut RenderWindow);
}
