use crate::interface::rendering::Renderer;
use crate::math::color::Color;

const SIZE: [f32; 2] = [1.0, 1.0];

/// Simplifies rendering by focusing on a grid of tiles
#[derive(Default)]
pub struct TileRenderer {}

impl TileRenderer {
    /// Renders a tile with a uniform color
    pub fn render_full(&mut self, renderer: &mut dyn Renderer, index: usize, color: Color) {
        let point = renderer.get_size().to_point(index);
        let point = [point[0] as f32, point[1] as f32];
        renderer
            .get_color_renderer()
            .render_rectangle(point, SIZE, color);
    }
}
