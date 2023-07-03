use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::CanvasService;
use crate::core::input::InputService;
use crate::core::scene::Scene;
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu<TTFContext, CANVAS> {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub canvas_service: Rc<RefCell<Box<dyn CanvasService<TTFContext, CANVAS>>>>
}

impl<TTFContext: 'static, CANVAS: 'static> Scene<TTFContext> for SceneMenu<TTFContext, CANVAS> {
    fn on_scene(
        &mut self,
        ttf_ctx: &TTFContext
    ) -> Option<Box<dyn Scene<TTFContext>>> {

        self.canvas_service.borrow_mut()
            .create_text(
                ttf_ctx,
                "press space",
                0,
                0,
                100,
                100
            )
            .expect("erreur lors de l'affichage du text");

        let scene_exemple = SceneExemple {
            key_manager: Rc::clone(&self.key_manager),
            canvas_service: Rc::clone(&self.canvas_service),
        };

        if self.key_manager.borrow().is_key_pressed("Space") {
            Some(Box::new(scene_exemple))
        } else {
            None
        }
    }
}