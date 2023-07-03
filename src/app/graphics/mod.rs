use std::path::Path;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

use crate::core::graphics::CanvasService;

pub struct CanvasServiceImpl {
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl CanvasServiceImpl {
    pub fn new(window: Window) -> Result<Self, String> {
        let canvas = window.into_canvas()
            .build()
            .expect("Failed to initialize canvas");
        let texture_creator = canvas.texture_creator();

        Ok(
            Self {
                canvas,
                texture_creator,
            }
        )
    }
}

impl CanvasService<Sdl2TtfContext, WindowCanvas> for CanvasServiceImpl {
    fn create_text(
        &mut self,
        ctx: &Sdl2TtfContext,
        text: &str,
        x: i32,
        y :i32,
        w: u32,
        h: u32
    ) -> Result<(), String> {
        let font_path: &Path = Path::new(&"assets/fonts/dpcomic.ttf");
        let mut font: Font = ctx.load_font(font_path, 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font
            .render(text)
            .blended(Color::RGB(255,0,0))
            .map_err(|err| err.to_string())?;

        let texture = self.texture_creator
            .create_texture_from_surface(surface)
            .map_err(|err| err.to_string())?;

        let target = Rect::new(x, y, w, h);

        self.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    fn get_canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}
