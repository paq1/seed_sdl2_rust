use std::path::Path;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontFactory<'a, 'b> {
    font: Font<'a, 'b>
}

impl<'a, 'b, 'c: 'a> FontFactory<'a, 'b> {
    pub fn new(ctx: &'c Sdl2TtfContext) -> Result<FontFactory<'a, 'b>, String> {
        let font_path: &Path = Path::new(&"assets/fonts/dpcomic.ttf");
        let mut font: Font = ctx.load_font(font_path, 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        Ok(
            Self {
                font
            }
        )
    }
}