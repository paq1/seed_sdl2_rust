use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::CanvasService;
use crate::core::input::InputService;
use crate::core::scene::Scene;

pub struct SceneExemple<TTFContext, CANVAS> {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub canvas_service: Rc<RefCell<Box<dyn CanvasService<TTFContext, CANVAS>>>>
}

impl<TTFContext, CANVAS> Scene<TTFContext> for SceneExemple<TTFContext, CANVAS> {
    fn on_scene(
        &mut self,
        ttf_ctx: &TTFContext
    ) -> Option<Box<dyn Scene<TTFContext>>> {

        let keys_pressed = self.get_keys_pressed();
        self.canvas_service.borrow_mut().create_text(
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

impl<SDLCTX, CANVAS> SceneExemple<SDLCTX, CANVAS> {
    fn get_keys_pressed(&self) -> String {
        self
            .key_manager
            .borrow()
            .key_pressed()
            .join("-")
    }
}