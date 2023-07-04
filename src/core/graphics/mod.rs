pub trait CanvasService<CANVAS> {
    fn get_canvas(&mut self) -> &mut CANVAS;
}

pub trait TextService {
    fn create_text(
        &self,
        text: &str,
        x: i32,
        y :i32,
        w: u32,
        h: u32
    ) -> Result<(), String>;
}
