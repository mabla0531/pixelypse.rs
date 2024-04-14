use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::states::game_state::{KeyboardData, MouseData};

pub const ENTITY_SIZE: usize = 32;

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
    fn move_towards_position(&mut self, position: (f64, f64));
    fn move_entity(&mut self, x: f64, y: f64);
    fn get_type(&self) -> EntityType;
    fn get_speed(&self) -> f64;
    fn get_position(&self) -> (f64, f64);
    fn update(&mut self, reference_position: (f64, f64), key_data: KeyboardData, mouse_data: MouseData);
    fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: (f64, f64));
}