use sfml::{graphics::{RenderWindow, Sprite, Transformable, RenderTarget, Rect, Texture}, system::{Vector2f, Vector2u}, SfBox};

use crate::entities::entity::Entity;

pub const CHUNK_SIZE: usize = 8;

pub struct Map {
    pub chunks: Vec<Vec<Chunk>>,
    pub entities: Vec<Box<dyn Entity>>,
    pub scale: u32,
    pub chunk_size_pixels: u32,
}

impl Map {
    pub fn new(scale: u32) -> Self {
        let mut chunks = Vec::new();
        let chunk_size_pixels = CHUNK_SIZE as u32 * 32 * scale;

        for x in 0..8 {
            let mut chunk_row: Vec<Chunk> = Vec::new();
            for y in 0..8 {
                chunk_row.push(Chunk::new(x * chunk_size_pixels, y * chunk_size_pixels, scale));
            }
            chunks.push(chunk_row);
        }

        Map {
            chunks,
            entities: Vec::new(),
            scale,
            chunk_size_pixels,
        }
    }

    pub fn get_map_size_pixels(&self) -> Vector2u {
        return Vector2u::new(self.chunks.len() as u32 * self.chunk_size_pixels, self.chunks[0].len() as u32 * self.chunk_size_pixels);
    }

    pub fn render(&self, window: &mut RenderWindow, terrain_texture: &SfBox<Texture>, game_camera: Rect<f32>) {
        let mut sprite = Sprite::new();
        sprite.set_texture(&terrain_texture, true);
        sprite.set_texture_rect(Rect::new(224, 512, 32, 32));


        for y in 0..self.chunks.len() as i32 {
            for x in 0..self.chunks[y as usize].len() as i32 {

                if  (x + 1) * self.chunk_size_pixels as i32 >= game_camera.left as i32 && 
                    (y + 1) * self.chunk_size_pixels as i32 >= game_camera.top as i32 && 
                    (x - 1) * self.chunk_size_pixels as i32 <= (game_camera.left + game_camera.width) as i32 &&
                    (y - 1) * self.chunk_size_pixels as i32 <= (game_camera.top + game_camera.height) as i32 {
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
    pub scale: u32,
}

impl Chunk {
    pub fn new(x: u32, y: u32, scale: u32) -> Self {
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
            scale,
        }
    }

    pub fn render(&self, window: &mut RenderWindow, sprite: &mut Sprite, camera_offset: Vector2f) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                sprite.set_position(
                    Vector2f::new(
                        (self.x as f32) + (x as u32 * (32 * self.scale)) as f32 - camera_offset.x, 
                        (self.y as f32) + (y as u32 * (32 * self.scale)) as f32 - camera_offset.y
                    )
                );
                sprite.set_scale(Vector2f::new(self.scale as f32, self.scale as f32));

                window.draw(
                    match self.tiles[x][y] {
                        1 => sprite,
                        _ => sprite,
                    }
                );
            }
        }
    }
}
