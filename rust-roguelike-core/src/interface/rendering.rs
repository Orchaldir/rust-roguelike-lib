use crate::math::color::Color;

pub trait Renderer {
    fn start(&mut self, color: Color);
    fn finish(&mut self);

    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color);
}
