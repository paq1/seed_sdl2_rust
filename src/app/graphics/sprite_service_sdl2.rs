use std::cell::RefCell;
use std::rc::Rc;

use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::app::factories::sprite_factory::SpriteFactory;
use crate::core::graphics::CanDrawSprite;

pub struct SpriteServiceSdl2<'sf> {
    pub canvas: Rc<RefCell<WindowCanvas>>,
    pub sprite_factory: Rc<RefCell<SpriteFactory<'sf>>>
}

impl<'sf> CanDrawSprite for SpriteServiceSdl2<'sf> {
    fn draw_sprite(&mut self, index: &str, x: i32, y: i32) -> Result<(), String> {

        let fact = self.sprite_factory.borrow();
        let sprite = fact.sprites.get(index).expect(format!("erreur sprite {} inconnu", index).as_str());

        self.canvas.borrow_mut().copy_ex(
            sprite,
            Rect::new(0, 0, 32, 32),
            Rect::new(x, y,32,32),
            0.0,
            Point::new(16, 16),
            false,
            false
        )?;

        Ok(())
    }
}