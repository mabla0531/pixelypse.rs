use std::sync::Arc;

use graphics::{Context, DrawState, Image, Transformed};
use opengl_graphics::GlGraphics;
use rand::random;

use crate::{assets::Assets, entities::entity::Entity};

pub const TILE_SIZE: usize = 16;
pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_SIZE_PIXELS: usize = CHUNK_SIZE * TILE_SIZE;

// source Image objects, containing the image size and pointing to where the texture exists on the spritesheet
pub const DIRT_IMG: Image = Image {
    color: None,
    rectangle: Some([0.0, 0.0, TILE_SIZE as f64, TILE_SIZE as f64]),
    source_rectangle: Some([0.0, 0.0, TILE_SIZE as f64 - 1.0, TILE_SIZE as f64 - 1.0]), 
    //                                | for some dogass fricking reason, this is not width/height, this is right/bottom
};

pub struct Map {
    //pub seed: i32,

    pub chunks: Vec<Vec<Chunk>>,
    pub entities: Vec<Box<dyn Entity>>,
    pub assets: Arc<Assets>
}

impl Map {
    pub fn new(assets: Arc<Assets>) -> Self {

        let mut seed: i32 = (random::<f32>() * 1000000000.0) as i32;
        if random::<bool>() { seed *= -1 }

        // TODO: this should cause tile generation to be "random". methodical generation can be added later so it isn't just noisy bullshit. 

        let mut chunks = Vec::new();

        for x in 0..16 {
            let mut chunk_row: Vec<Chunk> = Vec::new();
            for y in 0..16 {
                chunk_row.push(Chunk::new(x * CHUNK_SIZE_PIXELS as i32, y * CHUNK_SIZE_PIXELS as i32));
            }
            chunks.push(chunk_row);
        }

        Map {
            //seed: seed as i32,
            chunks,
            entities: Vec::new(),
            assets
        }
    }

    pub fn get_map_size_pixels(&self) -> (u32, u32) {
        (self.chunks.len() as u32 * CHUNK_SIZE_PIXELS as u32, self.chunks[0].len() as u32 * CHUNK_SIZE_PIXELS as u32)
    }

    pub fn render(&self, c: Context, g: &mut GlGraphics, camera_offset: (f64, f64), display_size: (f64, f64)) {
        
        for chunk_row in self.chunks.iter() {
            for chunk in chunk_row {
                
                let chunk_bounds = (
                    chunk.x as f64,
                    chunk.y as f64,
                    (chunk.x + CHUNK_SIZE_PIXELS as i32) as f64,
                    (chunk.y + CHUNK_SIZE_PIXELS as i32) as f64
                );

                if  chunk_bounds.0 < camera_offset.0 + display_size.0 &&
                    chunk_bounds.2 > camera_offset.0 &&
                    chunk_bounds.1 < camera_offset.1 + display_size.1 &&
                    chunk_bounds.3 > camera_offset.1 {
                    
                    for tile_y in 0..chunk.tiles.len() {
                        for tile_x in 0..chunk.tiles[tile_y].len() {

                            let img = match chunk.tiles[tile_x][tile_y] {
                                1 => DIRT_IMG,
                                _ => DIRT_IMG,
                            };

                            img.draw(
                                &self.assets.terrain_texture, 
                                &DrawState::default(),
                                c.transform.trans(
                                    (chunk.x + (tile_x * TILE_SIZE) as i32) as f64 - camera_offset.0, 
                                    (chunk.y + (tile_y * TILE_SIZE) as i32) as f64 - camera_offset.1
                                ),
                                g
                            );
                        }
                    }
                }
            }
        }
    }
}

pub struct Chunk  {
    x: i32,
    y: i32,
    tiles: [[u16; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new(x: i32, y: i32) -> Self {
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
            x, 
            y, 
            tiles
        }
    }
}
