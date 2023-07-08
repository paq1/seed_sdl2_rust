use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{SpriteService, TextService};
use crate::core::input::InputService;
use crate::core::scene::{SceneEnum};

pub struct SceneExemple {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService>>>
}

impl SceneExemple {
    pub fn on_scene<SS>(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SS>>
        where
            SS: SpriteService
    {

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