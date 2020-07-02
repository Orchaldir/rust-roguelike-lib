pub mod input;
pub mod rendering;

use input::KeyCode;
use rendering::Renderer;

pub trait App {
    fn init(&mut self, _renderer: &mut dyn Renderer) {}
    fn render(&mut self, renderer: &mut dyn Renderer);

    fn on_key_released(&mut self, _key: KeyCode) {}
}
