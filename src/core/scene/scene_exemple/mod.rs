use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::InputService;
use crate::core::scene::{SceneEnum};

pub struct SceneExemple<SpriteService, TextService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText
{
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<TextService>>,
    pub sprite_service: Rc<RefCell<SpriteService>>,
}

impl<SpriteService, TextService> SceneExemple<SpriteService, TextService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText
{
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService>> {

        let keys_pressed = self.get_keys_pressed();
        self.text_service.borrow_mut().create_text(
            format!("keys = {}", keys_pressed).as_str(),
            10i32,
            0i32,
            32u32,
            Color::rgb(0u8, 200u8, 100u8)
        ).expect("erreur lors de l'affichage");

        None
    }

    fn get_keys_pressed(&self) -> String {
        self
            .key_manager
            .borrow()
            .key_pressed()
            .join("-")
    }
}