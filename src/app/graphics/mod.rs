use std::collections::HashMap;
use std::path::Path;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use crate::core::graphics::CanvasService;

pub struct CanvasServiceImpl {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    // window: Window,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
    pub font_service: FontService
}

impl CanvasServiceImpl {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("seed sdl2 -- paq1", 800, 600)
            .position_centered()
            .build()
            .expect("could not initialize the video subsystem");
        let canvas = window.into_canvas()
            .build()
            .expect("Failed to initialize canvas");
        let texture_creator = canvas.texture_creator();

        Ok(
            Self {
                sdl_context,
                video_subsystem,
                // window,
                canvas,
                texture_creator,
                font_service: FontService::new()?
            }
        )
    }
}

impl CanvasService<Sdl, WindowCanvas> for CanvasServiceImpl {
    fn create_text(&mut self, text: &str, x: i32, y :i32, w: u32, h: u32) -> Result<(), String> {
        let font_path: &Path = Path::new(&"assets/fonts/dpcomic.ttf");
        let mut font: Font = self.font_service.sdl_ttf_context.load_font(font_path, 128)?;
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

    fn get_ctx(&mut self) -> &mut Sdl {
        &mut self.sdl_context
    }
}

pub struct FontService {
    sdl_ttf_context: Sdl2TtfContext
}

impl FontService {
    pub fn new() -> Result<Self, String> {
        let sdl_ttf_context= sdl2::ttf::init().map_err(|e| e.to_string())?;
        Ok(
            Self {
                sdl_ttf_context
            }
        )
    }
}