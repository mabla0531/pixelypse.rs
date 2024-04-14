use kira::sound::static_sound::{StaticSoundSettings, StaticSoundData};
use opengl_graphics::{Texture, TextureSettings};

pub struct Assets {
    pub terrain_texture: Texture,
    pub player_texture: Texture,
    pub zombie_texture: Texture,

    //pub font: SfBox<Font>,

    pub handgun_cock: StaticSoundData,
    pub handgun_fire: StaticSoundData,
}

impl Assets {
    pub fn new() -> Assets {

        let terrain_texture = Texture::from_path("res/textures/terrain_8.png", &TextureSettings::new()).unwrap();
        let player_texture = Texture::from_path("res/textures/player/Idle.png", &TextureSettings::new()).unwrap();
        let zombie_texture = Texture::from_path("res/textures/zombie/Idle.png", &TextureSettings::new()).unwrap();

        //let font = Font::from_file("res/default.ttf").unwrap();

        let handgun_cock = StaticSoundData::from_file("res/sounds/handgun_cock.wav", StaticSoundSettings::default()).unwrap();
        let handgun_fire = StaticSoundData::from_file("res/sounds/handgun_fire.wav", StaticSoundSettings::default()).unwrap();

        Assets {
            terrain_texture,
            player_texture,
            zombie_texture,
            
            //font,

            handgun_cock,
            handgun_fire,
        }
    }
}