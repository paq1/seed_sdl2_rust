use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct SpriteFactory<'t> {
    pub spite_smiley: Texture<'t>
}

impl<'a> SpriteFactory<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<SpriteFactory<'a>, String> {
        let spite_smiley_path: &Path = Path::new("assets/sprites/smiley_sdl_seed.bmp");
        let spite_smiley: Texture<'a> = tc.load_texture(spite_smiley_path)?;

        Ok(
            Self {
                spite_smiley
            }
        )
    }
}