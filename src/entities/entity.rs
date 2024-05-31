use crate::states::game_state::{KeyboardData, MouseData};
use sfml::{graphics::RenderWindow, system::Vector2f};

pub const ENTITY_SIZE: usize = 64;

#[derive(PartialEq)]
pub enum EntityType {
    PLAYER = 0,
    ZOMBIE = 1,
}

#[derive(PartialEq)]
pub enum Behavior {
    STATIC = 0,
    CHASING = 1,
}

pub trait Entity {
    fn move_towards_position(&mut self, position: Vector2f);
    fn move_entity(&mut self, x: f32, y: f32);
    fn get_type(&self) -> EntityType;
    fn get_speed(&self) -> f32;
    fn get_position(&self) -> Vector2f;
    fn update(&mut self, reference_position: Vector2f, key_data: KeyboardData, mouse_data: MouseData);
    fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f);
}