use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, TextService};
use crate::core::input::InputService;
use crate::core::scene::{SceneEnum};

pub struct SceneExemple<SpriteService>
    where
        SpriteService: CanDrawSprite
{
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService>>>,
    pub sprite_service: Rc<RefCell<Box<SpriteService>>>,
}

impl<SpriteService: CanDrawSprite> SceneExemple<SpriteService> {
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SpriteService>> {

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