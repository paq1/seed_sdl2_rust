use std::collections::HashMap;
use crate::core::input::CanManageInput;

pub struct InputServiceImpl {
    pub key_manager: HashMap<String, bool>
}

impl InputServiceImpl {
    pub fn new() -> Self {
        Self {
            key_manager: HashMap::new()
        }
    }
}

impl CanManageInput for InputServiceImpl {
    fn is_key_pressed(&self, value: &str) -> bool {
        *self.key_manager.get(value).unwrap_or(&false)
    }

    fn key_down(&mut self, keyname: String) {
        if !self.key_manager.contains_key(&keyname) {
            self.key_manager.entry(keyname).or_insert(true);
        } else {
            if let Some(x) = self.key_manager.get_mut(&keyname) {
                *x = true;
            }
        }
    }

    fn key_up(&mut self, keyname: String) {
        if !self.key_manager.contains_key(&keyname) {
            self.key_manager.entry(keyname).or_insert(false);
        } else {
            if let Some(x) = self.key_manager.get_mut(&keyname) {
                *x = false;
            }
        }
    }

    fn key_pressed(&self) -> Vec<String> {
        self.key_manager
            .keys()
            .filter(|key| self.is_key_pressed(key))
            .map(|k| k.clone())
            .collect::<Vec<_>>()
    }
}