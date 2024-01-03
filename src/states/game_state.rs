use std::time::Instant;

use sfml::{graphics::{RenderWindow, Rect}, window::{Key, mouse::Button}, system::{Vector2u, Vector2f}};

use crate::{State, assets::Assets, map::Map, entities::{entity::{Entity, self}, player::Player, zombie::Zombie}};

pub struct KeyboardData {
    
}

pub struct MouseData {
    pub position: Vector2f,
    pub left_click: bool,
    pub right_click: bool,
}

pub struct GameState {
    pub map: Map,
    pub entities: Vec<Box<dyn Entity>>,
    pub player: usize, //index of player, so it can always be handled
    pub keyboard_data: KeyboardData,
    pub mouse_data: MouseData,
    pub window_size: Vector2u,
    pub assets: Assets,
}

impl GameState {
    pub fn new(assets: &Assets, window_size: Vector2u) -> Self {

        let map = Map::new();

        let mut entities: Vec<Box<dyn Entity>> = Vec::new();

        entities.push(Box::new(Player::new()));
        entities.push(Box::new(Zombie::new()));
        let player = 0;

        let keyboard = KeyboardData {};
        let mouse = MouseData {
            position: Vector2f::new(0.0, 0.0),
            left_click: false,
            right_click: false,
        };

        GameState {
            map,
            entities,
            player,
            keyboard_data: keyboard,
            mouse_data: mouse,
            window_size,
            assets: assets.clone(),
        }
    }
}

impl State for GameState {

    fn keypress_event(&mut self, key: Key) {
        match key {
            
            _ => {},
        }
    }

    fn keyrelease_event(&mut self, key: Key) {
        match key {
            
            _ => {},
        }
    }

    fn mouse_press_event(&mut self, button: Button) {
        match button {
            Button::Left => self.mouse_data.left_click = true,
            Button::Right => self.mouse_data.right_click = true,
            _ => {},
        }
    }

    fn mouse_release_event(&mut self, button: Button) {
        match button {
            Button::Left => self.mouse_data.left_click = false,
            Button::Right => self.mouse_data.right_click = false,
            _ => {},
        }
    }

    fn mouse_position_event(&mut self, position: Vector2f) {
        self.mouse_data.position = position;
    }

    fn update(&mut self) {
        let player = &mut self.entities[self.player];

        if self.mouse_data.left_click {
            let delta = self.mouse_data.position - player.get_position();

            let angle = libm::atan2(delta.y as f64, delta.x as f64);


            let x = player.get_speed() * libm::cos(angle) as f32;
            let y = player.get_speed() * libm::sin(angle) as f32;

            player.move_entity(x, y);
        }

        for entity in &mut self.entities { entity.update(); }
    }

    fn render(&mut self, window: &mut RenderWindow) {
        let player_position = self.entities[self.player].get_position();

        let mut game_camera = Rect::new(player_position.x + 16.0 - self.window_size.x as f32 / 2.0,
                                                   player_position.y + 16.0 - self.window_size.y as f32 / 2.0,
                                                   self.window_size.x as f32,
                                                   self.window_size.y as f32);
        
        game_camera.left = game_camera.left.max(0.0);
        game_camera.top = game_camera.top.max(0.0);
        game_camera.left = game_camera.left.min(self.map.get_map_size_pixels().x as f32 - game_camera.width);
        game_camera.top = game_camera.top.min(self.map.get_map_size_pixels().y as f32 - game_camera.height);

        self.map.render(window, &self.assets.terrain_texture, game_camera);

        for entity_index in 0..self.entities.len() {
            self
            .entities[entity_index]
            .render(
                window, 
                if entity_index != self.player {
                    &self.assets.zombie_texture
                } else {
                    &self.assets.player_texture
                }, 
                Vector2f::new(game_camera.left, game_camera.top)
            );
        }
    }
}