pub trait CanPlayMusic {
    fn play(&self, id: &str) -> Result<(), String>;
    fn play_sound(&self, id: &str) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}