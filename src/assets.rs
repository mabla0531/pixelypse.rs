use sfml::SfBox;
use sfml::graphics::{Texture, Font};

#[derive(Clone)]
pub struct Assets {
    pub terrain_texture: SfBox<Texture>,
    pub player_texture: SfBox<Texture>,
    pub zombie_texture: SfBox<Texture>,
    pub font: SfBox<Font>,

}

impl Assets {
    pub fn new() -> Assets {

        println!("{:?}", std::env::current_dir().unwrap());

        let terrain_texture = Texture::from_file("res/terrain.png").unwrap();
        let player_texture = Texture::from_file("res/player.png").unwrap();
        let zombie_texture = Texture::from_file("res/zombie.png").unwrap();
        let font = Font::from_file("res/default.ttf").unwrap();

        Assets {
            terrain_texture,
            player_texture,
            zombie_texture,
            font
        }
    }
}