use std::{f32::consts::SQRT_2, sync::Arc, time::Instant};

use sfml::{graphics::{IntRect, Rect, RenderTarget, RenderWindow, Sprite, Transformable}, system::{Vector2f, Vector2i}};
use kira::manager::{AudioManager, AudioManagerSettings};

use crate::{assets::Assets, states::game_state::{KeyboardData, MouseData}};
use super::entity::{Entity, EntityType, ENTITY_SIZE};

pub const PLAYER_RECT: IntRect = Rect::new(0, 0, ENTITY_SIZE as i32, ENTITY_SIZE as i32);

pub struct Player {
    pub x: f32,
    pub y: f32,

    pub firing_cooldown: Instant,

    pub assets: Arc<Assets>,
    pub audio_manager: AudioManager,
}

impl Entity for Player {
    
    fn move_towards_position(&mut self, _: Vector2f) {
        
    }

    fn move_entity(&mut self, x: f32, y: f32) {
        if x != 0.0 && y != 0.0 {
            let diagonal_movement = self.get_speed() / SQRT_2;
            self.x += diagonal_movement * x.signum();
            self.y += diagonal_movement * y.signum();
        } else {
            self.x += x;
            self.y += y;
        }
    }

    fn get_type(&self) -> EntityType {
        EntityType::PLAYER
    }

    fn get_speed(&self) -> f32 { 0.5 }

    fn get_position(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }

    fn update(&mut self, _: Vector2f, key_data: KeyboardData, mouse_data: MouseData) {
        if mouse_data.left_click {
            if self.firing_cooldown.elapsed().as_millis() >= 1000 {
                self.audio_manager.play(self.assets.handgun_fire.clone()).unwrap();
                self.firing_cooldown = Instant::now();
            }
        }

        let (mut x_move, mut y_move) = (0.0, 0.0);
        if key_data.w { y_move -= self.get_speed() }
        if key_data.s { y_move += self.get_speed() }
        if key_data.a { x_move -= self.get_speed() }
        if key_data.d { x_move += self.get_speed() }
        self.move_entity(x_move, y_move);
    }

    fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f) {
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.assets.player_texture, true);
        sprite.set_texture_rect(PLAYER_RECT);
        let final_position = self.get_position() - camera_offset - Vector2f::new((ENTITY_SIZE / 2) as f32, (ENTITY_SIZE / 2) as f32);
        let final_position = Vector2f::new(
            final_position.x.round(),
            final_position.y.round(),
        );
        sprite.set_position(final_position);
        window.draw(&sprite);
    }
}

impl Player {
    pub fn new(assets: Arc<Assets>) -> Self {
        Player {
            x: 32.0,
            y: 32.0,
            firing_cooldown: Instant::now(),
            assets,
            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}