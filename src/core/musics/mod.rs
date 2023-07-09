pub trait CanPlayMusic {
    fn play(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}