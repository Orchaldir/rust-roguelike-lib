pub mod rendering;

use rendering::Renderer;

pub trait App {
    fn render(&mut self, renderer: &mut dyn Renderer);
}
