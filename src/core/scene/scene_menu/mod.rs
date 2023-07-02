use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::core::input::InputService;
use crate::core::scene::Scene;

pub struct SceneMenu {
    pub key_manager: Rc<RefCell<Box<dyn InputService>>>
}

impl Scene for SceneMenu {
    fn on_scene(&mut self) -> Option<Box<dyn Scene>> {
        None
    }
}