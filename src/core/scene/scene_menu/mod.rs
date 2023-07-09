use std::cell::RefCell;
use std::rc::{Rc};
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, TextService};
use crate::core::input::InputService;
use crate::core::scene::{SceneEnum};
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu<SpriteService>
    where
        SpriteService: CanDrawSprite
{
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService>>>,
    pub sprite_service: Rc<RefCell<SpriteService>>
}

impl<SpriteService: CanDrawSprite> SceneMenu<SpriteService> {
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService>> {

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