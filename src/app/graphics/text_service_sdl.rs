use std::cell::{Ref, RefCell};
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::app::factories::FontFactory;
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::core::graphics::TextService;
use crate::core::graphics::models::color::Color as ColorCore;

pub struct TextServiceSDL<'a> {
    // pub canvas_service: Rc<RefCell<Box<dyn CanvasService<WindowCanvas>>>>,
    pub canvas: Rc<RefCell<WindowCanvas>>,
    pub texture_creator_service: Rc<RefCell<TextureCreatorService>>,
    pub texture_factory: Rc<RefCell<FontFactory<'a>>>
}

impl<'a> TextServiceSDL<'a> {
    pub fn new(
        canvas: Rc<RefCell<WindowCanvas>>,
        texture_creator_service: Rc<RefCell<TextureCreatorService>>,
        texture_factory: Rc<RefCell<FontFactory<'a>>>
    ) -> Self {
        Self {
            canvas,
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
        font_size: u32,
        color: ColorCore,
    ) -> Result<(), String> {

        let color_sdl: Color = color.into();

        let surface = self.texture_factory.borrow().font
            .render(text)
            .blended(color_sdl)
            .map_err(|err| err.to_string())?;

        let t_creator: Ref<TextureCreatorService> = self.texture_creator_service.borrow();
        let texture_creator = &t_creator.texture_creator;

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|err| err.to_string())?;

        let width = font_size * text.len() as u32;
        let height = font_size;

        let target = Rect::new(x, y, width, height);

        self.canvas.borrow_mut().copy(&texture, None, Some(target))?;

        Ok(())
    }
}

impl Into<Color> for ColorCore {
    fn into(self) -> Color {
        Color::RGBA(
            self.r,
            self.g,
            self.b,
            self.a
        )
    }
}
