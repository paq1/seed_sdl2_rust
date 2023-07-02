pub mod scene_menu;
pub mod scene_exemple;

pub trait Scene {
    // si il y a une transition de scene Some(nouvelle scene) sinon None
    fn on_scene(&mut self) -> Option<Box<dyn Scene>>;
}

pub struct SceneManager {
    pub current: Box<dyn Scene>
}

impl SceneManager {
    pub fn update_scene(&mut self) {
        if let Some(x) = self.current.on_scene() {
            // let c = &Box::into_inner(x);
            self.current = x;
        }
    }
}
