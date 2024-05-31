use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use sfml::SfBox;
use sfml::graphics::{Texture, Font};

#[derive(Clone)]
pub struct Assets {
    pub terrain_texture: SfBox<Texture>,
    pub player_texture: SfBox<Texture>,
    pub zombie_texture: SfBox<Texture>,

    pub font: SfBox<Font>,

    pub handgun_cock: StaticSoundData,
    pub handgun_fire: StaticSoundData,
}

impl Assets {
    pub fn new() -> Assets {

        let terrain_texture = Texture::from_file("res/textures/terrain_32.png").unwrap();
        let player_texture = Texture::from_file("res/textures/player/Idle.png").unwrap();
        let zombie_texture = Texture::from_file("res/textures/zombie/Idle.png").unwrap();

        let font = Font::from_file("res/default.ttf").unwrap();

        let handgun_cock = StaticSoundData::from_file("res/sounds/handgun_cock.wav", StaticSoundSettings::default()).unwrap();
        let handgun_fire = StaticSoundData::from_file("res/sounds/handgun_fire.wav", StaticSoundSettings::default()).unwrap();

        Assets {
            terrain_texture,
            player_texture,
            zombie_texture,
            
            font,

            handgun_cock,
            handgun_fire,
        }
    }
}