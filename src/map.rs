use std::sync::Arc;

use graphics::{Context, DrawState, Image, Transformed};
use opengl_graphics::GlGraphics;

use crate::{assets::Assets, entities::entity::Entity, util::{Point, Rect}, TILE_SIZE};

pub const CHUNK_SIZE: usize = 8;

// source Image objects, containing the image size and pointing to where the texture exists on the spritesheet
pub const DIRT_IMG: Image = Image {
    color: None,
    rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
    source_rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
};

pub struct Map {
    pub chunks: Vec<Vec<Chunk>>,
    pub entities: Vec<Box<dyn Entity>>,
    pub assets: Arc<Assets>
}

impl Map {
    pub fn new(assets: Arc<Assets>) -> Self {
        let mut chunks = Vec::new();

        for x in 0..8 {
            let mut chunk_row: Vec<Chunk> = Vec::new();
            for y in 0..8 {
                chunk_row.push(
                    Chunk::new(
                        (x * Self::chunk_size_pixels()) as u32, 
                        (y * Self::chunk_size_pixels()) as u32
                    )
                );
            }
            chunks.push(chunk_row);
        }

        Map {
            chunks,
            entities: Vec::new(),
            assets
        }
    }

    pub fn chunk_size_pixels() -> usize {
        CHUNK_SIZE * TILE_SIZE
    }

    pub fn get_map_size_pixels(&self) -> Point<u32> {
        println!("{}\n{}\n{}\n", self.chunks.len(), Self::chunk_size_pixels(), self.chunks[0].len());
        return Point::new(
            self.chunks.len() as u32 * Self::chunk_size_pixels() as u32, 
            self.chunks[0].len() as u32 * Self::chunk_size_pixels() as u32
        );
    }

    pub fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: Rect<f64>) {

        for y in 0..self.chunks.len() {
            for x in 0..self.chunks[y as usize].len() {
                if camera_offset.contains(Point::new((x * TILE_SIZE) as f64, (y * TILE_SIZE) as f64)) {
                    DIRT_IMG.draw(
                        &self.assets.zombie_texture, 
                        &DrawState::default(), 
                        c.transform.trans(
                            x as f64 - camera_offset.left, 
                            y as f64 - camera_offset.top
                        ), 
                        g
                    );
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
}
