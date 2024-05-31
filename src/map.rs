use std::{sync::{mpsc::{self, Receiver, Sender}, Arc}, thread::{self, JoinHandle}};

use rand::{rngs::StdRng, Rng, SeedableRng};
use sfml::{graphics::{FloatRect, IntRect, Rect, RenderTarget, RenderWindow, Sprite, Transformable}, system::Vector2f};

use crate::{assets::Assets, entities::entity::Entity};

pub const TILE_SIZE: i32 = 32;
pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_SIZE_PIXELS: usize = CHUNK_SIZE * TILE_SIZE as usize;

// source Image objects, containing the image size and pointing to where the texture exists on the spritesheet
pub const GRASS_IMG: IntRect = Rect::new(TILE_SIZE * 0, TILE_SIZE * 0, TILE_SIZE * 1, TILE_SIZE * 1);
pub const SAND_IMG: IntRect  = Rect::new(TILE_SIZE * 1, TILE_SIZE * 0, TILE_SIZE * 1, TILE_SIZE * 1);
pub const DIRT_IMG: IntRect  = Rect::new(TILE_SIZE * 0, TILE_SIZE * 1, TILE_SIZE * 1, TILE_SIZE * 1);
pub const STONE_IMG: IntRect = Rect::new(TILE_SIZE * 1, TILE_SIZE * 1, TILE_SIZE * 1, TILE_SIZE * 1);

pub struct Map {
    pub chunks: Vec<Chunk>,
    pub entities: Vec<Box<dyn Entity>>,
    pub assets: Arc<Assets>,

    chunk_generator: JoinHandle<()>,
    chunk_generator_sender: Sender<Vec<(i32, i32)>>,
    generated_chunks_receiver: Receiver<Vec<Chunk>>,
}

impl Map {
    pub fn new(assets: Arc<Assets>) -> Self {

        let seed = rand::thread_rng().gen_range(100000000..1000000000);
        
        println!("{}", seed);

        // TODO: this should cause tile generation to be "random". methodical generation can be added later so it isn't just noisy bullshit. 

        let mut chunks = Vec::new();

        for x in 0..16 {
            for y in 0..16 {
                chunks.push(Chunk::random(x * CHUNK_SIZE_PIXELS as i32, y * CHUNK_SIZE_PIXELS as i32, seed));
            }
        }

        let (chunk_generator_sender, chunk_generator_receiver) = mpsc::channel::<Vec<(i32, i32)>>();
        let (generated_chunks_sender, generated_chunks_receiver) = mpsc::channel::<Vec<Chunk>>();

        let chunk_generator = thread::spawn(move || {
            loop {
                let chunk_request = chunk_generator_receiver.recv().expect("Channel closed");
                let chunks: Vec<Chunk> = chunk_request
                    .iter()
                    .map(|chunk| {
                        Chunk::random(chunk.0, chunk.1, seed)
                    }).collect();

                generated_chunks_sender.send(chunks).expect("Channel closed");
            }
        });

        Map {
            chunks,
            entities: Vec::new(),
            assets,

            chunk_generator,
            chunk_generator_sender,
            generated_chunks_receiver
        }
    }
    
    pub fn render(&self, window: &mut RenderWindow, camera_offset: Vector2f, display_size: Vector2f) {
        
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.assets.terrain_texture, true);

        for chunk in self.chunks.iter() {
            let chunk_bounds = FloatRect::new(
                chunk.x as f32,
                chunk.y as f32,
                (chunk.x + CHUNK_SIZE_PIXELS as i32) as f32,
                (chunk.y + CHUNK_SIZE_PIXELS as i32) as f32
            );

            if FloatRect::from_vecs(camera_offset, display_size)
                .intersection(&chunk_bounds)
                .is_some() {
                
                for tile_y in 0..chunk.tiles.len() {
                    for tile_x in 0..chunk.tiles[tile_y].len() {

                        sprite.set_texture_rect(match chunk.tiles[tile_x][tile_y] {
                            0 => GRASS_IMG,
                            1 => SAND_IMG,
                            2 => DIRT_IMG,
                            3 => STONE_IMG,
                            _ => GRASS_IMG,
                        });

                        sprite.set_position(
                            Vector2f::new(
                                (chunk.x + (tile_x as i32 * TILE_SIZE)) as f32, 
                                (chunk.y + (tile_y as i32 * TILE_SIZE)) as f32
                            ) - camera_offset
                        );
        
                        window.draw(&sprite);
                    }
                }
            }
        }
    }
}

pub struct Chunk {
    x: i32,
    y: i32,
    tiles: [[u16; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn template(x: i32, y: i32) -> Self {
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

    pub fn random(x: i32, y: i32, seed: u32) -> Self {
        let mut rng = StdRng::seed_from_u64((seed as u64 * x.abs() as u64 * y.abs() as u64) as u64);
        println!("Seeding with: {}", (seed as u64 * x.abs() as u64 * y.abs() as u64) as u64);
        let mut tiles: [[u16; CHUNK_SIZE]; CHUNK_SIZE] = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];

        for tile_row in tiles.iter_mut() {
            for tile in tile_row {
                *tile = rng.gen_range(0..=3);
            }
        }

        Chunk {
            x,
            y,
            tiles
        }
    }
}
