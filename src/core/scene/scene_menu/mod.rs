use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::core::graphics::CanvasService;
use crate::core::input::InputService;
use crate::core::scene::Scene;
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu<SDLCTX, CANVAS> {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub canvas_service: Rc<RefCell<Box<dyn CanvasService<SDLCTX, CANVAS>>>>,
    pub next_scenes: HashMap<String, Box<dyn Scene>>
}

impl<SDLCTX: 'static, CANVAS: 'static> Scene for SceneMenu<SDLCTX, CANVAS> {
    fn on_scene(
        &mut self
    ) -> Option<Box<dyn Scene>> {

        self.canvas_service.borrow_mut()
            .create_text("menu", 0,0,100,100)
            .expect("erreur lors de l'affichage du text");

        let mut scene_exemple = SceneExemple {
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