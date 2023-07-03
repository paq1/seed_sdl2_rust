pub mod scene_menu;
pub mod scene_exemple;

pub trait Scene<TTFContext> {
    // si il y a une transition de scene Some(nouvelle scene) sinon None
    fn on_scene(&mut self, ttf_ctx: &TTFContext) -> Option<Box<dyn Scene<TTFContext>>>;
}

pub struct SceneManager<TTFContext> {
    pub current: Box<dyn Scene<TTFContext>>
}

impl<TTFContext> SceneManager<TTFContext> {
    pub fn update_scene(&mut self, ttf_ctx: &TTFContext) {
        if let Some(x) = self.current.on_scene(ttf_ctx) {
            // let c = &Box::into_inner(x);
            self.current = x;
        }
    }
}
