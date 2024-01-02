use sfml::{graphics::{RenderWindow, Rect}, window::Key, system::{Vector2u, Vector2f}};

use crate::{State, assets::Assets, map::Map, entities::{entity::Entity, player::Player}};

pub struct GameplayKeys {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

pub struct GameState {
    pub map: Map,
    pub entities: Vec<Box<dyn Entity>>,
    pub player: usize, //index of player, so it can always be handled
    pub keys: GameplayKeys,
    pub window_size: Vector2u,
}

impl GameState {
    pub fn new(assets: &Assets, window_size: Vector2u) -> Self {

        let map = Map::new(assets);

        let mut entities: Vec<Box<dyn Entity>> = Vec::new();

        entities.push(Box::new(Player::new(assets)));
        let player = 0;

        let keys = GameplayKeys {w: false, a: false, s: false, d: false};

        GameState {
            map,
            entities,
            player,
            keys,
            window_size,
        }
    }
}

impl State for GameState {

    fn keypress_event(&mut self, key: Key) {
        match key {
            Key::W => self.keys.w = true,
            Key::A => self.keys.a = true,
            Key::S => self.keys.s = true,
            Key::D => self.keys.d = true,
            _ => {},
        }
    }

    fn keyrelease_event(&mut self, key: Key) {
        match key {
            Key::W => self.keys.w = false,
            Key::A => self.keys.a = false,
            Key::S => self.keys.s = false,
            Key::D => self.keys.d = false,
            _ => {},
        }
    }

    fn update(&mut self) {
        let player = &mut self.entities[self.player];

        if self.keys.w { player.move_entity(0.0,                 -player.get_speed()); }
        if self.keys.a { player.move_entity(-player.get_speed(), 0.0                ); }
        if self.keys.s { player.move_entity(0.0,                 player.get_speed() ); }
        if self.keys.d { player.move_entity(player.get_speed(),  0.0                ); }

        for entity in &self.entities { entity.update(); }
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

        self.map.render(window, game_camera);

        for entity in &self.entities {
            entity.render(window, Vector2f::new(game_camera.left, game_camera.top));
        }
    }
}