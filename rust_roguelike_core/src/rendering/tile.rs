use crate::interface::rendering::{Renderer, TextureId};
use crate::math::color::Color;

const SIZE: [f32; 2] = [1.0, 1.0];

/// Simplifies rendering by focusing on a grid of tiles
pub struct TileRenderer {
    texture_id: TextureId,
}

impl TileRenderer {
    /// Creates a new TileRenderer
    pub fn new(texture_id: TextureId) -> TileRenderer {
        TileRenderer { texture_id }
    }

    /// Renders a tile with a uniform color
    pub fn render_full(&mut self, renderer: &mut dyn Renderer, index: usize, color: Color) {
        let point = renderer.get_size().to_point(index);
        let point = [point[0] as f32, point[1] as f32];
        renderer
            .get_color_renderer()
            .render_rectangle(point, SIZE, color);
    }

    /// Renders a tile as an ascii character
    pub fn render_ascii(
        &mut self,
        renderer: &mut dyn Renderer,
        index: usize,
        ascii: u8,
        color: Color,
    ) {
        let point = renderer.get_size().to_point(index);
        let point = [point[0] as f32, point[1] as f32];
        renderer
            .get_ascii_renderer(self.texture_id)
            .render_u8(point, SIZE, ascii, color);
    }
}
