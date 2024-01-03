use std::time::Instant;

use sfml::{system::Vector2f, graphics::{RenderWindow, Texture, Sprite, Rect, Transformable, RenderTarget}, SfBox};

use super::entity::{Entity, EntityType, Behavior};

use rand::Rng;

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    behavior: Behavior,
    roaming_direction: Vector2f,
    behavior_delta: Instant,
}

impl Entity for Zombie {
    fn get_type(&self) -> EntityType {
        EntityType::ZOMBIE
    }

    fn get_speed(&self) -> f32 {
        0.25
    }

    fn get_position(&self) -> Vector2f {
        return Vector2f::new(self.x, self.y);
    }

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

    fn update(&mut self) {
        match self.behavior {
            Behavior::STATIC => { },
            Behavior::ROAMING => self.move_entity(self.roaming_direction.x, self.roaming_direction.y),
            Behavior::CHASING => {

            }
        }

        if self.behavior_delta.elapsed().as_millis() >= 4000 && self.behavior == Behavior::ROAMING {
            self.behavior_delta = Instant::now();
            self.behavior = Behavior::STATIC;
        }

        if self.behavior_delta.elapsed().as_millis() >= 2000 && self.behavior == Behavior::STATIC {
            self.behavior_delta = Instant::now();
            self.behavior = Behavior::ROAMING;
            self.roaming_direction = match (rand::thread_rng().gen::<f32>() * 10.0) as u16 {
                0 => Vector2f::new(0.0, -self.get_speed()),
                1 => Vector2f::new(self.get_speed(), -self.get_speed()),
                2 => Vector2f::new(self.get_speed(), 0.0),
                3 => Vector2f::new(self.get_speed(), self.get_speed()),
                4 => Vector2f::new(0.0, self.get_speed()),
                5 => Vector2f::new(-self.get_speed(), self.get_speed()),
                6 => Vector2f::new(-self.get_speed(), 0.0),
                7 => Vector2f::new(-self.get_speed(), -self.get_speed()),
                8 => Vector2f::new(0.0, 0.0),
                9 => Vector2f::new(0.0, 0.0),
                _ => Vector2f::new(0.0, 0.0),
            };
        }
    }

    fn render(&self, window: &mut RenderWindow, texture: &SfBox<Texture>, camera_offset: Vector2f) {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, true);
        sprite.set_texture_rect(Rect::new(32, 128, 32, 32));

        sprite.set_position(Vector2f::new(self.x - camera_offset.x, self.y - camera_offset.y));

        window.draw(&sprite);
    }
}

impl Zombie {
    pub fn new() -> Self {
        Zombie {
            x: 256.0,
            y: 256.0,
            behavior: Behavior::STATIC,
            roaming_direction: Vector2f::new(0.0, 0.0),
            behavior_delta: Instant::now(),
        }
    }
}