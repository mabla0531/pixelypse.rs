use std::sync::Arc;

use graphics::{Context, DrawState, Image, Transformed};
use kira::manager::{AudioManager, AudioManagerSettings};
use opengl_graphics::GlGraphics;

use crate::{assets::Assets, states::game_state::{KeyboardData, MouseData}, util::UtilFunctions};

use super::entity::{Behavior, Entity, EntityType, ENTITY_SIZE};

pub const ZOMBIE_IMG: Image = Image {
    color: None,
    rectangle: Some([0.0, 0.0, ENTITY_SIZE as f64, ENTITY_SIZE as f64]),
    source_rectangle: Some([0.0, 0.0, ENTITY_SIZE as f64, ENTITY_SIZE as f64]),
};

pub struct Zombie {
    pub x: f64,
    pub y: f64,
    behavior: Behavior,

    pub assets: Arc<Assets>,

    pub audio_manager: AudioManager,
}

impl Entity for Zombie {
    fn get_type(&self) -> EntityType {
        EntityType::ZOMBIE
    }

    fn get_speed(&self) -> f64 { 0.25 }

    fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn move_entity(&mut self, _: f64, _: f64) { }

    fn move_towards_position(&mut self, position: (f64, f64)) {

        let angle = UtilFunctions::get_angle(position, self.get_position());

        let x = self.get_speed() * libm::cos(angle);
        let y = self.get_speed() * libm::sin(angle);

        self.x += x;
        self.y += y;
    }

    fn update(&mut self, reference_position: (f64, f64), _: KeyboardData, _: MouseData) {
        match self.behavior {
            Behavior::STATIC => { },
            Behavior::CHASING => self.move_towards_position(reference_position), 
        }
        
        let delta = (self.get_position().0 - reference_position.0, self.get_position().1 - reference_position.1);
        self.behavior = if delta.0.abs() < 256.0 && delta.1.abs() < 256.0 {
            Behavior::CHASING
        } else {
            Behavior::STATIC
        };
    }

    fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: (f64, f64)) {
        ZOMBIE_IMG.draw(
            &self.assets.zombie_texture, 
            &DrawState::default(), 
            c.transform.trans(
                self.x - camera_offset.0, 
                self.y - camera_offset.1
            ), 
            g
        );
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