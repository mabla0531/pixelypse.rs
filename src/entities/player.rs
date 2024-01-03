use sfml::{SfBox, graphics::{Texture, Sprite, Rect, RenderTarget, Transformable, RenderWindow}, system::Vector2f};

use super::entity::{Entity, EntityType};

#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Entity for Player {
    fn move_entity(&mut self, x: f32, y: f32) {

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

    fn update(&mut self) {
        
    }

    fn render(&self, window: &mut RenderWindow, texture: &SfBox<Texture>, camera_offset: Vector2f) {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, true);
        sprite.set_texture_rect(Rect::new(32, 128, 32, 32));

        sprite.set_position(Vector2f::new(self.x - camera_offset.x, self.y - camera_offset.y));

        window.draw(&sprite);
    }
}

impl Player {
    pub fn new() -> Self {

        Player {
            x: 32.0,
            y: 32.0,
        }
    }
}