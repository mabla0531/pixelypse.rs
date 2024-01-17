use kira::manager::{AudioManager, AudioManagerSettings};
use sfml::{system::Vector2f, graphics::{RenderWindow, Sprite, Rect, Transformable, RenderTarget}};

use crate::{states::game_state::{MouseData, KeyboardData}, Util, assets::Assets};

use super::entity::{Entity, EntityType, Behavior};

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    behavior: Behavior,
    pub scale: u32,

    pub assets: Assets,

    pub audio_manager: AudioManager,
}

impl Entity for Zombie {
    fn get_type(&self) -> EntityType {
        EntityType::ZOMBIE
    }

    fn get_speed(&self) -> f32 {
        0.5 * self.scale as f32
    }

    fn get_position(&self) -> Vector2f {
        return Vector2f::new(self.x, self.y);
    }

    fn move_entity(&mut self, _: f32, _: f32) {

        
    }

    fn move_towards_position(&mut self, position: Vector2f) {

        let angle = Util::get_angle(position, self.get_position());

        let x = self.get_speed() * libm::cos(angle) as f32;
        let y = self.get_speed() * libm::sin(angle) as f32;


        self.x += x;
        self.y += y;
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
        sprite.set_texture_rect(Rect::new(32, 128, 32, 32));

        let half_sprite = self.scale as f32 / 2.0;
        sprite.set_position(self.get_position() - camera_offset - Vector2f::new(half_sprite, half_sprite));
        sprite.set_scale(Vector2f::new(self.scale as f32, self.scale as f32));

        window.draw(&sprite);
    }
}

impl Zombie {
    pub fn new(scale: u32, assets: Assets) -> Self {
        Zombie {
            x: 256.0,
            y: 256.0,
            behavior: Behavior::STATIC,
            scale,
            assets,
            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}