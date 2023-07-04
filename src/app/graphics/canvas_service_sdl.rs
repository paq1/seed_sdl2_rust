use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::WindowCanvas;
use crate::app::graphics::texture_creator_service::TextureCreatorService;

use crate::core::graphics::CanvasService;

pub struct CanvasServiceImpl {
    pub canvas: WindowCanvas,
    pub texture_creator_service: Rc<RefCell<TextureCreatorService>>
}

impl CanvasServiceImpl {
    pub fn new(
        canvas: WindowCanvas,
        texture_creator_service: Rc<RefCell<TextureCreatorService>>
    ) -> Result<Self, String> {
        Ok(
            Self {
                canvas,
                texture_creator_service,
            }
        )
    }
}

impl CanvasService<WindowCanvas> for CanvasServiceImpl {
    fn get_canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}