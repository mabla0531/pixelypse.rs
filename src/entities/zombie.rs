use std::sync::Arc;

use kira::manager::{AudioManager, AudioManagerSettings};
use sfml::{graphics::{IntRect, Rect, RenderTarget, RenderWindow, Sprite, Transformable}, system::Vector2f};

use crate::{assets::Assets, states::game_state::{KeyboardData, MouseData}, util::UtilFunctions};
use super::entity::{Behavior, Entity, EntityType, ENTITY_SIZE};

pub const ZOMBIE_IMG: IntRect = Rect::new(0, 0, ENTITY_SIZE as i32, ENTITY_SIZE as i32);

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    
    pub behavior: Behavior,
    pub assets: Arc<Assets>,
    pub audio_manager: AudioManager,
}

impl Entity for Zombie {
    fn get_type(&self) -> EntityType {
        EntityType::ZOMBIE
    }

    fn get_speed(&self) -> f32 { 0.25 }

    fn get_position(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }

    fn move_entity(&mut self, _: f32, _: f32) { }

    fn move_towards_position(&mut self, position: Vector2f) {

        let angle = UtilFunctions::get_angle(position, self.get_position());

        let x = self.get_speed() as f64 * libm::cos(angle);
        let y = self.get_speed() as f64 * libm::sin(angle);

        self.x += x as f32;
        self.y += y as f32;
    }

    fn update(&mut self, reference_position: Vector2f, _: KeyboardData, _: MouseData) {
        match self.behavior {
            Behavior::STATIC => { },
            Behavior::CHASING => self.move_towards_position(reference_position), 
        }
        
        let delta = self.get_position() - reference_position;
        self.behavior = if delta.x.abs() < 256.0 && delta.y.abs() < 256.0 {
            Behavior::CHASING
        } else {
            Behavior::STATIC
        };
    }

    fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f) {
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.assets.zombie_texture, true);
        sprite.set_texture_rect(ZOMBIE_IMG);
        sprite.set_position(self.get_position() - camera_offset);
        window.draw(&sprite);
    }
}

impl Zombie {
    pub fn new(assets: Arc<Assets>) -> Self {
        Zombie {
            x: 256.0,
            y: 256.0,
            behavior: Behavior::STATIC,
            assets,
            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}