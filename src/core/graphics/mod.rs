use crate::core::graphics::models::color::Color;

pub mod models;

pub trait CanDrawText {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y :i32,
        font_size: u32,
        color: Color
    ) -> Result<(), String>;
}

pub trait CanDrawSprite {
    fn draw_sprite(
        &mut self,
        index: &str,
        x: i32,
        y: i32,
    ) -> Result<(), String>;
}