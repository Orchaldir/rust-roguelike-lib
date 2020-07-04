use crate::interface::App;
use crate::math::color::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub type TextureId = usize;

pub trait Renderer {
    fn start(&mut self, color: Color);
    fn finish(&mut self);

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
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
