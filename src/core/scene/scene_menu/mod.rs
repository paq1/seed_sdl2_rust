use std::cell::RefCell;
use std::rc::{Rc};
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{SpriteService, TextService};
use crate::core::input::InputService;
use crate::core::scene::{SceneEnum};
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu<SS>
    where
        SS: SpriteService
{
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService>>>,
    pub sprite_service: Rc<RefCell<Box<SS>>>
}

impl<SS: SpriteService> SceneMenu<SS> {
    pub fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<SceneEnum<SS>> {

        self.text_service.borrow_mut()
            .create_text(
                "press space",
                300,
                30,
                32u32,
                Color::rgb(255u8, 0u8, 0u8)
            ).expect("erreur lors de l'affichage");

        if self.key_manager.borrow().is_key_pressed("Space") {
            let scene_exemple = SceneExemple {
                key_manager: Rc::clone(&self.key_manager),
                text_service: Rc::clone(&self.text_service),
            };
            Some(SceneEnum::SceneExemple(scene_exemple))
        } else {
            None
        }
    }
}