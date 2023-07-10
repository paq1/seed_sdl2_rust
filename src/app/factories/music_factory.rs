use std::path::Path;

use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, Music, Sdl2MixerContext};

pub struct MusicFactory<'m> {
    pub music_menu: Music<'m>,
    _context_mixer: Sdl2MixerContext,
}

impl<'m> MusicFactory<'m> {
    pub fn new() -> Result<MusicFactory<'m>, String> {
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
        sdl2::mixer::allocate_channels(4);
        let mixer_ctx: Sdl2MixerContext = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3)?;

        {
            let n = sdl2::mixer::get_music_decoders_number();
            println!("available musique decoders : {}", n);
            for i in 0..n {
                println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
            }
        }

        let music_menu_path: &Path = Path::new("assets/musics/digital-love.wav");
        let music_menu = Music::from_file(music_menu_path)?;
        Ok(
            Self {
                music_menu,
                _context_mixer: mixer_ctx,
            }
        )
    }
}