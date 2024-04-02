use std::{f32::consts::SQRT_2, sync::Arc, time::Instant};

use graphics::{Context, DrawState, Image, Transformed};
use kira::manager::{AudioManager, AudioManagerSettings};
use opengl_graphics::GlGraphics;

use crate::{assets::Assets, states::game_state::{KeyboardData, MouseData}, util::Point, TILE_SIZE};

use super::entity::{Entity, EntityType};

pub const PLAYER_IMG: Image = Image {
    color: None,
    rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
    source_rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
};

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub destination: Option<Point<f64>>,

    pub firing_cooldown: Instant,

    pub assets: Arc<Assets>,

    pub audio_manager: AudioManager,
}

impl Entity for Player {
    
    fn move_towards_position(&mut self, _: Point<f64>) {
        
    }

    fn move_entity(&mut self, x: f64, y: f64) {
        if x != 0.0 && y != 0.0 {
            let diagonal_movement = self.get_speed() / SQRT_2 as f64;
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

    fn get_speed(&self) -> f64 { 0.75 }

    fn get_position(&self) -> Point<f64> {
        return Point::new(self.x, self.y);
    }

    fn update(&mut self, _: Point<f64>, key_data: KeyboardData, mouse_data: MouseData) {
        
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

    fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: Point<f64>) {
        PLAYER_IMG.draw(
            &self.assets.player_texture, 
            &DrawState::default(), 
            c.transform.trans(
                self.x - camera_offset.x, 
                self.y - camera_offset.y
            ), 
            g
        );
    }
}

impl Player {
    pub fn new(assets: Arc<Assets>) -> Self {
        Player {
            x: 32.0,
            y: 32.0,
            destination: None,
            firing_cooldown: Instant::now(),
            assets,
            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}