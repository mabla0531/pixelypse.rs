use sfml::{graphics::RenderWindow, system::Vector2f};

pub enum EntityType {
    PLAYER = 0,
}

pub trait Entity {
    fn move_entity(&mut self, x: f32, y: f32);
    fn get_type(&self) -> EntityType;
    fn get_speed(&self) -> f32;
    fn get_position(&self) -> Vector2f;
    fn update(&self);
    fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f);
}