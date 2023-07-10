use std::cell::RefCell;
use std::rc::Rc;
use crate::app::factories::music_factory::MusicFactory;
use crate::core::musics::CanPlayMusic;

pub struct MusicServiceImpl<'mf> {
    pub music_factory: Rc<RefCell<MusicFactory<'mf>>>
}

impl<'mf> CanPlayMusic for MusicServiceImpl<'mf> {
    fn play(&self, id: &str) -> Result<(), String> {
        self.music_factory.borrow().musics.get(id).map(|music| music.play(-1))
            .map(|_| Ok(()))
            .unwrap_or(Err(format!("musique : '{}' inconnue", id).to_string()))
    }

    fn stop(&self) -> Result<(), String> {
        sdl2::mixer::Music::halt();
        Ok(())
    }
}