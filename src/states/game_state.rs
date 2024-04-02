use std::sync::Arc;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Key, MouseButton, Size};

use crate::{
    assets::Assets, entities::{entity::{Entity, EntityType}, player::Player, zombie::Zombie}, map::Map, util::{Rect, Point}, State
};

#[derive(Clone)]
pub struct MouseData {
    pub position: Point<f32>,
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
    pub game_camera: Rect<f64>,
    pub window_size: Size,
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

        let keyboard_data = KeyboardData {
            w: false,
            a: false,
            s: false,
            d: false,
        };
        

        let mouse_data = MouseData {
            position: Point::new(0.0, 0.0),
            left_click: false,
            right_click: false,
        };

        let game_camera = Rect::new(0.0, 0.0, 0.0, 0.0);

        GameState {
            map,
            entities,
            player,
            keyboard_data: keyboard_data,
            mouse_data: mouse_data,
            game_camera,
            window_size,
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

    fn mouse_position_event(&mut self, position: Point<f32>) {
        self.mouse_data.position = position;
    }

    fn update(&mut self) {
        let entities = &mut self.entities;
        let player_position = entities[self.player].get_position();

        self.game_camera = Rect::new(
            player_position.x + 16.0 - self.window_size.width as f64 / 2.0,
            player_position.y + 16.0 - self.window_size.height as f64 / 2.0,
            self.window_size.width as f64,
            self.window_size.height as f64,
        );

        self.game_camera.left = self.game_camera.left
            .max(0.0)
            .min(self.map.get_map_size_pixels().x as f64 - self.game_camera.width());
        self.game_camera.top = self.game_camera.top
            .max(0.0)
            .min(self.map.get_map_size_pixels().y as f64 - self.game_camera.height());

        for index in 0..entities.len() {
            if entities[index].get_type() == EntityType::PLAYER {
                self.player = index;
            }
        }

        for index in 0..entities.len() {
            let reference_position = match entities[index].get_type() {
                //if the entity is a player, pass the game camera as position
                EntityType::PLAYER => Point::new(self.game_camera.left, self.game_camera.top), 
                _ => entities[self.player].get_position()
            };

            entities[index].update(reference_position, self.keyboard_data.clone(), self.mouse_data.clone());
        }
    }

    fn render(&mut self, c: Context, g: &mut GlGraphics) {
        //sort entities by Y as to render the higher ones first
        self.entities.sort_by(
            |e1, e2| 
            e1.get_position().y.total_cmp(&e2.get_position().y)
        );
        
        self.map.render(c, g, self.game_camera);

        for entity in &self.entities {
            entity.render(c, g, Point::new(self.game_camera.left, self.game_camera.top));
        }
    }
}
