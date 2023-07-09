use std::path::Path;
use sdl2::mixer::Music;

pub struct MusicFactory<'m> {
    pub music_menu: Music<'m>
}

impl<'m> MusicFactory<'m> {
    pub fn new() -> Result<MusicFactory<'m>, String> {
        let music_menu_path: &Path = Path::new("assets/musics/digital-love.wav");
        // let spite_smiley: Texture<'a> = tc.load_texture(spite_smiley_path)?;
        let music_menu = Music::from_file(music_menu_path)?;
        Ok(
            Self {
                music_menu
            }
        )
    }
}