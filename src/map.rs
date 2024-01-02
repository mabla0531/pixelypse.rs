use sfml::{graphics::{RenderWindow, Sprite, Transformable, RenderTarget, Rect, Texture}, system::{Vector2f, Vector2i, Vector2u}, SfBox};

use crate::{assets::Assets, entities::entity::Entity};

pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_SIZE_PIXELS: u32 = CHUNK_SIZE as u32 * 32;

pub struct Map {
    pub terrain_texture: SfBox<Texture>,
    pub chunks: Vec<Vec<Chunk>>,
    pub entities: Vec<Box<dyn Entity>>,
}

impl Map {
    pub fn new(assets: &Assets) -> Self {
        let mut chunks = Vec::new();

        for x in 0..8 {
            let mut chunk_row: Vec<Chunk> = Vec::new();
            for y in 0..8 {
                chunk_row.push(Chunk::new(x * CHUNK_SIZE_PIXELS, y * CHUNK_SIZE_PIXELS));
            }
            chunks.push(chunk_row);
        }

        Map {
            terrain_texture: assets.terrain_texture.clone(),
            chunks,
            entities: Vec::new(),
        }
    }

    pub fn get_map_size_pixels(&self) -> Vector2u {
        return Vector2u::new(self.chunks.len() as u32 * CHUNK_SIZE_PIXELS, self.chunks[0].len() as u32 * CHUNK_SIZE_PIXELS);
    }

    pub fn render(&self, window: &mut RenderWindow, game_camera: Rect<f32>) {
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.terrain_texture, true);
        sprite.set_texture_rect(Rect::new(224, 512, 32, 32));


        for y in 0..self.chunks.len() as i32 {
            for x in 0..self.chunks[y as usize].len() as i32 {

                if  (x + 1) * CHUNK_SIZE_PIXELS as i32 >= game_camera.left as i32 && 
                    (y + 1) * CHUNK_SIZE_PIXELS as i32 >= game_camera.top as i32 && 
                    (x - 1) * CHUNK_SIZE_PIXELS as i32 <= (game_camera.left + game_camera.width) as i32 &&
                    (y - 1) * CHUNK_SIZE_PIXELS as i32 <= (game_camera.top + game_camera.height) as i32 {
                        self.chunks[x as usize][y as usize].render(window, &mut sprite, Vector2f::new(game_camera.left, game_camera.top));
                }
            }
        }
    }
}

pub struct Chunk {
    pub tiles: [[u16; CHUNK_SIZE]; CHUNK_SIZE],
    pub x: u32,
    pub y: u32,
}

impl Chunk {
    pub fn new(x: u32, y: u32) -> Self {
        let tiles: [[u16; CHUNK_SIZE]; CHUNK_SIZE] = [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ];

        Chunk {
            tiles,
            x, 
            y,
        }
    }

    pub fn render(&self, window: &mut RenderWindow, sprite: &mut Sprite, camera_offset: Vector2f) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                match self.tiles[x][y] {
                    1 => {
                        sprite.set_position(
                            Vector2f::new(
                                (self.x as f32) + (x as f32 * 32.0) - camera_offset.x, 
                                (self.y as f32) + (y as f32 * 32.0) - camera_offset.y
                            )
                        );
                        
                        window.draw(sprite);
                    },
                    _ => {}
                }
            }
        }
    }
}
