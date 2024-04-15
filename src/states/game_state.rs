use std::sync::Arc;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Key, MouseButton, Size};

use crate::{
    assets::Assets,
    entities::{
        entity::{Entity, EntityType},
        player::Player,
        zombie::Zombie,
    },
    map::Map,
    State,
};

#[derive(Clone)]
pub struct MouseData {
    pub position: (f64, f64),
    pub left_click: bool,
    pub right_click: bool,
}

#[derive(Clone)]
pub struct KeyboardData {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

pub struct GameState {
    pub map: Map,
    pub entities: Vec<Box<dyn Entity>>,
    pub player: usize, //index of player, so it can always be handled
    pub keyboard_data: KeyboardData,
    pub mouse_data: MouseData,
    pub window_size: Size,
    pub camera_offset: (f64, f64),
    pub assets: Arc<Assets>,
}

impl GameState {
    pub fn new(assets: Assets, window_size: Size) -> Self {
        let assets = Arc::new(assets);

        let map = Map::new(assets.clone());

        let mut entities: Vec<Box<dyn Entity>> = Vec::new();

        entities.push(Box::new(Player::new(assets.clone())));
        entities.push(Box::new(Zombie::new(assets.clone())));
        let player = 0;

        let camera_offset = (0.0, 0.0);

        let keyboard_data = KeyboardData {
            w: false,
            a: false,
            s: false,
            d: false,
        };

        let mouse_data = MouseData {
            position: (0.0, 0.0),
            left_click: false,
            right_click: false,
        };

        GameState {
            map,
            entities,
            player,
            keyboard_data,
            mouse_data,
            window_size,
            camera_offset,
            assets: assets.clone(),
        }
    }
}

impl State for GameState {
    fn keypress_event(&mut self, key: Key) {
        match key {
            Key::W => self.keyboard_data.w = true,
            Key::A => self.keyboard_data.a = true,
            Key::S => self.keyboard_data.s = true,
            Key::D => self.keyboard_data.d = true,
            _ => {}
        }
    }

    fn keyrelease_event(&mut self, key: Key) {
        match key {
            Key::W => self.keyboard_data.w = false,
            Key::A => self.keyboard_data.a = false,
            Key::S => self.keyboard_data.s = false,
            Key::D => self.keyboard_data.d = false,
            _ => {}
        }
    }

    fn mouse_press_event(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_data.left_click = true,
            MouseButton::Right => self.mouse_data.right_click = true,
            _ => {}
        }
    }

    fn mouse_release_event(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_data.left_click = false,
            MouseButton::Right => self.mouse_data.right_click = false,
            _ => {}
        }
    }

    fn mouse_position_event(&mut self, position: (f64, f64)) {
        self.mouse_data.position = position;
    }

    fn update(&mut self) {
        let entities = &mut self.entities;
        let player_position = entities[self.player].get_position();

        let map_size: (f64, f64) = (self.map.get_map_size_pixels().0 as f64, self.map.get_map_size_pixels().1 as f64);

        self.camera_offset = (
            player_position.0 - (self.window_size.width / 2.0), 
            player_position.1 - (self.window_size.height / 2.0)
        );
        
        self.camera_offset.0 = self.camera_offset.0
            .max(0.0)
            .min(map_size.0 - self.window_size.width);
    
        self.camera_offset.1 = self.camera_offset.1
            .max(0.0)
            .min(map_size.1 as f64 - self.window_size.height);
        
        for index in 0..entities.len() {
            if entities[index].get_type() == EntityType::PLAYER {
                self.player = index;
            }
        }

        for index in 0..entities.len() {
            let reference_position = match entities[index].get_type() {
                //if the entity is a player, pass the game camera as position
                EntityType::PLAYER => (0.0, 0.0),
                _ => entities[self.player].get_position(),
            };

            entities[index].update(
                reference_position,
                self.keyboard_data.clone(),
                self.mouse_data.clone(),
            );
        }
    }

    fn render(&mut self, c: Context, g: &mut GlGraphics) {
        //sort entities by Y so the higher ones are rendered first
        self.entities
            .sort_by(|e1, e2| e1.get_position().1.total_cmp(&e2.get_position().1));

        self.map.render(c, g, self.camera_offset, self.window_size.into());

        for entity in &self.entities {
            entity.render(
                c,
                g,
                self.camera_offset,
            );
        }
    }
}
