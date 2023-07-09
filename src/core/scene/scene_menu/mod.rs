use std::cell::RefCell;
use std::rc::{Rc};
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::CanManageInput;
use crate::core::scene::{SceneEnum};
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu<SpriteService, TextService, InputService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput
{
    pub key_manager: Rc<RefCell<InputService>>,
    pub text_service: Rc<RefCell<TextService>>,
    pub sprite_service: Rc<RefCell<SpriteService>>
}

impl<SpriteService, TextService, InputService> SceneMenu<SpriteService, TextService, InputService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput
{
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService, InputService>> {

        self.text_service.borrow_mut()
            .create_text(
                "press space",
                300,
                30,
                32u32,
                Color::rgb(255u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");


        self.sprite_service.borrow_mut().draw_sprite("nimp", 300, 300).expect("err");

        if self.key_manager.borrow().is_key_pressed("Space") {
            let scene_exemple = SceneExemple {
                key_manager: Rc::clone(&self.key_manager),
                text_service: Rc::clone(&self.text_service),
                sprite_service: Rc::clone(&self.sprite_service)
            };
            Some(SceneEnum::SceneExemple(scene_exemple))
        } else {
            None
        }
    }
}