use crate::interface::App;
use crate::math::color::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub type TextureId = usize;

pub trait Renderer {
    fn start(&mut self, color: Color);
    fn finish(&mut self);

    fn load_texture(&mut self, filename: &str) -> TextureId;

    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color);
}

pub trait Window {
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
