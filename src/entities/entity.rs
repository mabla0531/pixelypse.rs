use sfml::{graphics::{RenderWindow, Texture}, system::Vector2f, SfBox};

pub enum EntityType {
    PLAYER = 0,
    ZOMBIE = 1,
}

#[derive(PartialEq)]
pub enum Behavior {
    STATIC = 0,
    ROAMING = 1,
    CHASING = 2,
}

pub trait Entity {
    fn move_entity(&mut self, x: f32, y: f32);
    fn get_type(&self) -> EntityType;
    fn get_speed(&self) -> f32;
    fn get_position(&self) -> Vector2f;
    fn update(&mut self);
    fn render(&self, window: &mut RenderWindow, texture: &SfBox<Texture>, camera_offset: Vector2f);
}