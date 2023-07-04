use std::cell::{Ref, RefCell};
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::app::factories::FontFactory;
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::core::graphics::{CanvasService, TextService};

pub struct TextServiceSDL<'a> {
    pub canvas_service: Rc<RefCell<Box<dyn CanvasService<WindowCanvas>>>>,
    pub texture_creator_service: Rc<RefCell<TextureCreatorService>>,
    pub texture_factory: Rc<RefCell<FontFactory<'a>>>
}

impl<'a> TextServiceSDL<'a> {
    pub fn new(
        canvas_service: Rc<RefCell<Box<dyn CanvasService<WindowCanvas>>>>,
        texture_creator_service: Rc<RefCell<TextureCreatorService>>,
        texture_factory: Rc<RefCell<FontFactory<'a>>>
    ) -> Self {
        Self {
            canvas_service,
            texture_creator_service,
            texture_factory
        }
    }
}

impl TextService for TextServiceSDL<'_> {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        w: u32,
        h: u32
    ) -> Result<(), String> {
        let surface = self.texture_factory.borrow().font
            .render(text)
            .blended(Color::RGB(255,0,0))
            .map_err(|err| err.to_string())?;

        let t_creator: Ref<TextureCreatorService> = self.texture_creator_service.borrow();
        let texture_creator = &t_creator.texture_creator;

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|err| err.to_string())?;

        let target = Rect::new(x, y, w, h);

        self.canvas_service.borrow_mut().get_canvas().copy(&texture, None, Some(target))?;

        Ok(())
    }
}