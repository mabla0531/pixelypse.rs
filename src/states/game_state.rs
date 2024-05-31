use std::sync::Arc;

use sfml::{graphics::RenderWindow, system::Vector2f, window::{mouse::Button, Key}};

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
    pub position: Vector2f,
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
    pub player_index: usize,
    pub keyboard_data: KeyboardData,
    pub mouse_data: MouseData,
    pub window_size: Vector2f,
    pub camera_offset: Vector2f,
    pub assets: Arc<Assets>,
}

impl GameState {
    pub fn new(assets: Assets, window_size: Vector2f) -> Self {
        let assets = Arc::new(assets);

        let map = Map::new(assets.clone());

        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        entities.push(Box::new(Player::new(assets.clone())));
        entities.push(Box::new(Zombie::new(assets.clone())));
        let player = 0;

        let camera_offset = Vector2f::new(0.0, 0.0);

        let keyboard_data = KeyboardData {
            w: false,
            a: false,
            s: false,
            d: false,
        };

        let mouse_data = MouseData {
            position: Vector2f::new(0.0, 0.0),
            left_click: false,
            right_click: false,
        };

        GameState {
            map,
            entities,
            player_index: player,
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

    fn mouse_press_event(&mut self, button: Button) {
        match button {
            Button::Left => self.mouse_data.left_click = true,
            Button::Right => self.mouse_data.right_click = true,
            _ => {}
        }
    }

    fn mouse_release_event(&mut self, button: Button) {
        match button {
            Button::Left => self.mouse_data.left_click = false,
            Button::Right => self.mouse_data.right_click = false,
            _ => {}
        }
    }

    fn mouse_position_event(&mut self, position: Vector2f) {
        self.mouse_data.position = position;
    }

    fn update(&mut self) {
        let entities = &mut self.entities;
        let player_position = entities[self.player_index].get_position();

        self.camera_offset = player_position - (self.window_size / 2.0);

        let reference_position = entities[self.player_index].get_position();
        
        for index in 0..entities.len() {
            entities[index].update(
                reference_position,
                self.keyboard_data.clone(),
                self.mouse_data.clone(),
            );
        }
    }

    fn render(&mut self, window: &mut RenderWindow) {
        //sort entities by Y so the higher ones are rendered first
        self.entities
            .sort_by(|e1, e2| 
                e1.get_position()
                .y
                .total_cmp(&e2.get_position().y)
            );

        self.player_index = self.entities
            .iter()
            .position(|e| e.get_type() == EntityType::PLAYER)
            .expect("Player does not exist!");

        self.map.render(window, self.camera_offset, self.window_size);

        for entity in &self.entities {
            entity.render(
                window,
                self.camera_offset,
            );
        }
    }
}
