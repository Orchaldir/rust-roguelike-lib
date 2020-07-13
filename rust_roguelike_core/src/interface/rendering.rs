use crate::interface::App;
use crate::math::color::Color;
use crate::math::size2d::Size2d;
use std::cell::RefCell;
use std::rc::Rc;

pub type TextureId = usize;

/// A trait to abstract away different rendering libraries and render targets.
pub trait Renderer {
    /// Returns the size of the render target in tiles.
    /// A tile is big enough to hold a single ascii character.
    fn get_size(&self) -> Size2d;

    /// Starts the rendering and fills the render target with the Color `color`
    fn start(&mut self, color: Color);

    /// Finishes the rendering
    fn finish(&mut self);

    /// Takes a screenshot and saves it.
    fn take_screenshot(&self, filename: &str);

    /// Loads a texture from a file and returns a `TextureId` as a handle
    ///
    /// # Panics
    ///
    /// Panics if the file does not exist.
    ///
    /// Panics if the file is not an image.
    ///
    /// Panics if it can not create a texture from the image.
    fn load_texture(&mut self, filename: &str) -> TextureId;

    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer;
    fn get_texture_renderer(&mut self, id: TextureId) -> &mut dyn TextureRenderer;
    fn get_ascii_renderer(&mut self, id: TextureId) -> &mut dyn AsciiRenderer;
}

pub trait ColorRenderer {
    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color);
    fn render_rectangle(&mut self, position: [f32; 2], size: [f32; 2], color: Color);
}

pub trait TextureRenderer {
    fn render_rectangle(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        tc: [f32; 2],
        tc_size: [f32; 2],
        color: Color,
    );
}

pub trait AsciiRenderer {
    fn render_text(&mut self, position: [f32; 2], size: [f32; 2], string: &str, color: Color);

    fn render_char(&mut self, position: [f32; 2], size: [f32; 2], c: char, color: Color);

    fn render_u8(&mut self, position: [f32; 2], size: [f32; 2], ascii: u8, color: Color);
}

pub trait Window {
    /// Returns the size of the window in tiles.
    fn get_tiles(&self) -> Size2d;

    /// Runs the application using the window as the render target
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
