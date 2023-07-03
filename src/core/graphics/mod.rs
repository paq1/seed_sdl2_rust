pub trait CanvasService<SDLCTX, CANVAS> {
    fn create_text(
        &mut self,
        ctx: &SDLCTX,
        text: &str,
        x: i32,
        y :i32,
        w: u32,
        h: u32
    ) -> Result<(), String>; // todo rendre parametrable
    fn get_canvas(&mut self) -> &mut CANVAS;
}
