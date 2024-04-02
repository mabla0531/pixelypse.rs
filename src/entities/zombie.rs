use std::sync::Arc;

use graphics::{Context, DrawState, Image, Transformed};
use kira::manager::{AudioManager, AudioManagerSettings};
use opengl_graphics::GlGraphics;

use crate::{assets::Assets, states::game_state::{KeyboardData, MouseData}, util::{Point, UtilFunctions}, TILE_SIZE};

use super::entity::{Entity, EntityType, Behavior};

pub const ZOMBIE_IMG: Image = Image {
    color: None,
    rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
    source_rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
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

    fn get_speed(&self) -> f64 { 0.5 }

    fn get_position(&self) -> Point<f64> {
        return Point::new(self.x, self.y);
    }

    fn move_entity(&mut self, _: f64, _: f64) { }

    fn move_towards_position(&mut self, position: Point<f64>) {

        let angle = UtilFunctions::get_angle(position, self.get_position());

        let x = self.get_speed() * libm::cos(angle);
        let y = self.get_speed() * libm::sin(angle);


        self.x += x;
        self.y += y;
    }

    fn update(&mut self, reference_position: Point<f64>, _: KeyboardData, _: MouseData) {
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

    fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: Point<f64>) {
        ZOMBIE_IMG.draw(
            &self.assets.zombie_texture, 
            &DrawState::default(), 
            c.transform.trans(
                self.x - camera_offset.x, 
                self.y - camera_offset.y
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