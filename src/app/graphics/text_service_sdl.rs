use std::cell::{Ref, RefCell};
use std::path::Path;
use std::rc::Rc;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::core::graphics::{CanvasService, TextService};

pub struct TextServiceSDL {
    pub canvas_service: Rc<RefCell<Box<dyn CanvasService<Sdl2TtfContext, WindowCanvas>>>>,
    pub texture_creator_service: Rc<RefCell<TextureCreatorService>>
}

impl TextServiceSDL {
    pub fn new(
        canvas_service: Rc<RefCell<Box<dyn CanvasService<Sdl2TtfContext, WindowCanvas>>>>,
        texture_creator_service: Rc<RefCell<TextureCreatorService>>
    ) -> Self {
        Self {
            canvas_service,
            texture_creator_service
        }
    }
}

impl TextService<Sdl2TtfContext> for TextServiceSDL {
    fn create_text(
        &self,
        ctx_ttf: &Sdl2TtfContext,
        text: &str,
        x: i32,
        y: i32,
        w: u32,
        h: u32
    ) -> Result<(), String> {

        let font_path: &Path = Path::new(&"assets/fonts/dpcomic.ttf");
        let mut font: Font = ctx_ttf.load_font(font_path, 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font
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