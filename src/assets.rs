use sfml::SfBox;
use sfml::graphics::{Texture, Font};

pub struct Assets {
    pub terrain_texture: SfBox<Texture>,
    pub entity_texture: SfBox<Texture>,
    pub font: SfBox<Font>,

}

impl Assets {
    pub fn new() -> Assets {

        println!("{:?}", std::env::current_dir().unwrap());

        let terrain_texture = Texture::from_file("res/terrain.png").unwrap();
        let entity_texture = Texture::from_file("res/entities.png").unwrap();
        let font = Font::from_file("res/default.ttf").unwrap();

        Assets {
            terrain_texture,
            entity_texture,
            font
        }
    }
}