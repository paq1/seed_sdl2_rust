use crate::core::graphics::SpriteService;
use crate::core::scene::scene_exemple::SceneExemple;
use crate::core::scene::scene_menu::SceneMenu;

pub mod scene_menu;
pub mod scene_exemple;


pub enum SceneEnum<SS>
    where
        SS: SpriteService
{
    SceneMenu(SceneMenu<SS>),
    SceneExemple(SceneExemple)
}

// pub trait Scene {
//     // si il y a une transition de scene Some(nouvelle scene) sinon None
//     fn on_scene<S: Scene>(&mut self, dt: f32) -> Option<Box<S>>;
// }

pub struct SceneManager<SS>
    where
        SS: SpriteService
{
    pub current: SceneEnum<SS>
}

impl<SS: SpriteService> SceneManager<SS> {
    pub fn update_scene(&mut self, dt: f32) {

        let nouvelle_scene = match &mut self.current {
            SceneEnum::SceneMenu(menu) => menu.on_scene(dt),
            SceneEnum::SceneExemple(exemple) => exemple.on_scene(dt)
        };

        if let Some(x) = nouvelle_scene {
            self.current = x;
        }

        // if let Some(x) = self.current.on_scene(dt) {
        //     // let c = &Box::into_inner(x);
        //     self.current = x;
        // }
    }
}
