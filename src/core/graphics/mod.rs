pub trait CanvasService<SDLCTX, CANVAS> {
    fn get_canvas(&mut self) -> &mut CANVAS;
}

pub trait TextService<CTXTTF> {
    fn create_text(
        &self,
        ctx_ttf: &CTXTTF,
        text: &str,
        x: i32,
        y :i32,
        w: u32,
        h: u32
    ) -> Result<(), String>;
}
