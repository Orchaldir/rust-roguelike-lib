use crate::renderer::get_corners;
use crate::vertex::ColoredVertex;
use rust_roguelike_core::interface::rendering::ColorRenderer;
use rust_roguelike_core::math::color::Color;

#[derive(Default)]
pub struct ColorBuilder {
    pub vertices: Vec<ColoredVertex>,
}

impl ColorBuilder {
    fn add(&mut self, position: [f32; 2], color: Color) {
        self.vertices.push(ColoredVertex {
            position,
            color: color.into(),
        });
    }
}

impl ColorRenderer for ColorBuilder {
    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    fn render_tile(&mut self, position: [f32; 2], size: [f32; 2], color: Color) {
        let [c00, c10, c01, c11] = get_corners(position, size);

        self.render_triangle(c00, c10, c11, color);
        self.render_triangle(c00, c11, c01, color);
    }
}
