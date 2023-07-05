pub trait InputService {
    fn is_key_pressed(&self, value: &str) -> bool;

    fn key_down(&mut self, keyname: String);

    fn key_up(&mut self, keyname: String);

    fn key_pressed(&self) -> Vec<String>;
}