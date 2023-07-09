use std::cell::RefCell;
use std::rc::Rc;
use crate::app::factories::music_factory::MusicFactory;
use crate::core::musics::CanPlayMusic;

pub struct MusicServiceImpl<'mf> {
    pub music_factory: Rc<RefCell<MusicFactory<'mf>>>
}

impl<'mf> CanPlayMusic for MusicServiceImpl<'mf> {
    fn play(&self) -> Result<(), String> {
        self.music_factory.borrow().music_menu.play(-1)
    }

    fn stop(&self) -> Result<(), String> {
        sdl2::mixer::Music::halt();
        Ok(())
    }
}