pub mod rendering;

use rendering::Renderer;

pub trait App {
    fn init(&mut self, renderer: &mut dyn Renderer);
    fn render(&mut self, renderer: &mut dyn Renderer);
}
