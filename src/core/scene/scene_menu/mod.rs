use std::cell::RefCell;
use std::rc::Rc;

use crate::core::graphics::TextService;
use crate::core::input::InputService;
use crate::core::scene::Scene;
use crate::core::scene::scene_exemple::SceneExemple;

pub struct SceneMenu {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>,
    pub text_service: Rc<RefCell<Box<dyn TextService>>>
}

impl Scene for SceneMenu {
    fn on_scene(
        &mut self,
        _dt: f32
    ) -> Option<Box<dyn Scene>> {

        self.text_service.borrow_mut()
            .create_text(
                "press space",
                300,
                30,
                300,
                100
            ).expect("erreur lors de l'affichage");

        if self.key_manager.borrow().is_key_pressed("Space") {
            let scene_exemple = SceneExemple {
                key_manager: Rc::clone(&self.key_manager),
                text_service: Rc::clone(&self.text_service),
            };
            Some(Box::new(scene_exemple))
        } else {
            None
        }
    }
}