use crate::renderer::get_corners;
use crate::vertex::TexturedVertex;
use rust_roguelike_core::interface::rendering::TextureRenderer;
use rust_roguelike_core::math::color::Color;

#[derive(Default)]
pub struct TextureBuilder {
    pub vertices: Vec<TexturedVertex>,
}

impl TextureBuilder {
    fn add_vertex(&mut self, position: [f32; 2], tc: [f32; 2], color: Color) {
        self.vertices.push(TexturedVertex {
            position,
            color: color.into(),
            tc,
        });
    }

    fn add_triangle(
        &mut self,
        a: [f32; 2],
        b: [f32; 2],
        c: [f32; 2],
        tc_a: [f32; 2],
        tc_b: [f32; 2],
        tc_c: [f32; 2],
        color: Color,
    ) {
        self.add_vertex(a, tc_a, color);
        self.add_vertex(b, tc_b, color);
        self.add_vertex(c, tc_c, color);
    }
}

impl TextureRenderer for TextureBuilder {
    fn render_tile(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        tc: [f32; 2],
        tc_size: [f32; 2],
        color: Color,
    ) {
        let [c00, c10, c01, c11] = get_corners(position, size);
        let [tc00, tc10, tc01, tc11] = get_corners(tc, tc_size);

        self.add_triangle(c00, c10, c11, tc00, tc10, tc11, color);
        self.add_triangle(c00, c11, c01, tc00, tc11, tc01, color);
    }
}
