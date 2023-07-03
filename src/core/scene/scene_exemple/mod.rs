use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::TextService;
use crate::core::input::InputService;
use crate::core::scene::Scene;

pub struct SceneExemple<TTFContext> {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService<TTFContext>>>>
}

impl<TTFContext> Scene<TTFContext> for SceneExemple<TTFContext> {
    fn on_scene(
        &mut self,
        ttf_ctx: &TTFContext
    ) -> Option<Box<dyn Scene<TTFContext>>> {

        let keys_pressed = self.get_keys_pressed();
        self.text_service.borrow_mut().create_text(
            ttf_ctx,
            format!("keys = {}", keys_pressed).as_str(),
            10i32,
            0i32,
            600u32,
            100u32
        ).expect("erreur lors de l'affichage");

        None
    }
}

impl<E> SceneExemple<E> {
    fn get_keys_pressed(&self) -> String {
        self
            .key_manager
            .borrow()
            .key_pressed()
            .join("-")
    }
}