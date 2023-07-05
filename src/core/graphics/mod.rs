use crate::core::graphics::models::color::Color;

pub mod models;

pub trait CanvasService<CANVAS> {
    fn get_canvas(&mut self) -> &mut CANVAS;
}

pub trait TextService {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y :i32,
        font_size: u32,
        color: Color
    ) -> Result<(), String>;
}
