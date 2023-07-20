use std::collections::HashMap;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct SpriteFactory<'t> {
    pub sprites: HashMap<&'t str, Texture<'t>>,
    // pub spite_smiley: Texture<'t>
}

impl<'a> SpriteFactory<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<SpriteFactory<'a>, String> {
        let spite_smiley_path: &Path = Path::new("assets/sprites/smiley_sdl_seed.bmp");
        let spite_smiley: Texture<'a> = tc.load_texture(spite_smiley_path)?;

        let tile_herbe_path: &Path = Path::new("assets/sprites/tiles/tile-herbe-claire.png");
        let tile_herbe: Texture<'a> = tc.load_texture(tile_herbe_path)?;

        let tile_brique_path: &Path = Path::new("assets/sprites/tiles/brique.png");
        let tile_brique: Texture<'a> = tc.load_texture(tile_brique_path)?;

        let viseur_path: &Path = Path::new("assets/sprites/curseur/curseur.png");
        let viseur: Texture<'a> = tc.load_texture(viseur_path)?;

        let sprites: HashMap<&str, Texture> = [
            ("smiley", spite_smiley),
            ("tile_herbe", tile_herbe),
            ("tile_brique", tile_brique),
            ("viseur", viseur)
        ]
            .into_iter()
            .collect::<HashMap<&str, Texture>>();

        Ok(
            Self {
                sprites
            }
        )
    }
}