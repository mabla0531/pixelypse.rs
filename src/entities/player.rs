use sfml::{SfBox, graphics::{Texture, Sprite, Rect, RenderTarget, Transformable, RenderWindow}, system::Vector2f};

use crate::assets::Assets;

use super::entity::{Entity, EntityType};

#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub entity_texture: SfBox<Texture>,
}

impl Entity for Player {
    fn move_entity(&mut self, x_move: f32, y_move: f32) {

        let mut x = x_move;
        let mut y = y_move;

        let sqrt_two = f32::from(2.0).sqrt();

        if x != 0.0 && y != 0.0 {
            x = self.get_speed() / sqrt_two;
            y = self.get_speed() / sqrt_two;
        }

        self.x += x;
        self.y += y;
    }

    fn get_type(&self) -> EntityType {
        EntityType::PLAYER
    }

    fn get_speed(&self) -> f32 {
        0.75
    }

    fn get_position(&self) -> Vector2f {
        return Vector2f::new(self.x, self.y);
    }

    fn update(&self) {
        
    }

    fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f) {
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.entity_texture, true);
        sprite.set_texture_rect(Rect::new(32, 0, 32, 32));

        sprite.set_position(Vector2f::new(self.x - camera_offset.x, self.y - camera_offset.y));

        window.draw(&sprite);
    }
}

impl Player {
    pub fn new(assets: &Assets) -> Self {

        Player {
            x: 32.0,
            y: 32.0,
            entity_texture: assets.entity_texture.clone(),
        }
    }
}