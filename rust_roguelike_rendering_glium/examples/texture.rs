extern crate glium;
extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLUE, RED};
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TextureExample {
    texture_id: TextureId,
}

impl App for TextureExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        renderer.get_texture_renderer(self.texture_id).render_rectangle(
            [200.0, 100.0],
            [400.0, 400.0],
            [0.0, 0.0],
            [1.0, 1.0],
            RED,
        );
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with a texture");
    let app = Rc::new(RefCell::new(TextureExample::default()));

    window.run(app.clone());
}
