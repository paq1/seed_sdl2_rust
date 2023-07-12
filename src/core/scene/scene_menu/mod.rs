use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::graphics::models::color::Color;
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::scene::SceneEnum;
use crate::core::scene::scene_exemple::SceneExemple;
use crate::core::scene::scene_menu::scene_menu_data::SceneMenuData;

pub mod scene_menu_data;

pub struct SceneMenu<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub key_manager: Rc<RefCell<InputService>>,
    pub text_service: Rc<RefCell<TextService>>,
    pub sprite_service: Rc<RefCell<SpriteService>>,
    pub music_service: Rc<RefCell<MusicService>>,
    pub data: SceneMenuData
}

impl<SpriteService, TextService, InputService, MusicService> SceneMenu<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {

        self.init_scene().expect("erreur lors de l'initialisation du menu");

        let next_scene = self.change_scene();
        self.test_play_sound();

        self.draw_text_for_change_scene();

        next_scene
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
    ) -> Self {
        Self {
            key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneMenuData {
                is_init: false,
            }
        }
    }

    fn init_scene(&mut self) -> Result<(), String> {
        if !self.data.is_init {
            self.data.is_init = true;
            self.music_service.borrow().play("digital-love")
        } else {
            Ok(())
        }
    }

    fn change_scene(&mut self) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {
        if self.key_manager.borrow().is_key_pressed("Space") {
            self.music_service.borrow().stop().expect("erreur lors de l'arret de la musique");
            let scene_exemple = SceneExemple::new(
                Rc::clone(&self.key_manager),
                Rc::clone(&self.text_service),
                Rc::clone(&self.sprite_service),
                Rc::clone(&self.music_service)
            );
            Some(SceneEnum::SceneExemple(scene_exemple))
        } else {
            None
        }
    }

    fn draw_text_for_change_scene(&mut self) {
        self.text_service.borrow_mut()
            .create_text(
                "[press space]",
                32 * 6,
                600 - 32 * 3,
                32u32,
                Color::rgb(255u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");
    }

    fn test_play_sound(&self) {
        if self.key_manager.borrow().is_key_pressed("X") {
            self.music_service.borrow().play_sound("arme").expect("erreur lors de la lecture du son arme");
        }
    }
}