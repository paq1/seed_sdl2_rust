use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::InputService;
use crate::core::scene::scene_exemple::SceneExemple;
use crate::core::scene::scene_menu::SceneMenu;

pub mod scene_menu;
pub mod scene_exemple;

pub enum SceneEnum<SpriteService, TextService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText
{
    SceneMenu(SceneMenu<SpriteService, TextService>),
    SceneExemple(SceneExemple<SpriteService, TextService>),
}

pub struct SceneManager<SpriteService, TextService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText
{
    pub current: SceneEnum<SpriteService, TextService>,
}

impl<SpriteService, TextService> SceneManager<SpriteService, TextService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText
{
    pub fn new(
        key_manager: Rc<RefCell<Box<dyn InputService>>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
    ) -> Self {
        let scene_menu = SceneMenu {
            key_manager: Rc::clone(&key_manager),
            text_service: Rc::clone(&text_service),
            sprite_service: Rc::clone(&sprite_service),
        };
        Self { current: SceneEnum::SceneMenu(scene_menu) }
    }

    pub fn update_scene(&mut self, dt: f32) {
        let nouvelle_scene = match &mut self.current {
            SceneEnum::SceneMenu(menu) => menu.on_scene(dt),
            SceneEnum::SceneExemple(exemple) => exemple.on_scene(dt)
        };

        if let Some(x) = nouvelle_scene {
            self.current = x;
        }
    }
}
