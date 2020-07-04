extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::input::{KeyCode, MouseButton};
use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLACK, BLUE, GREEN, RED, YELLOW};
use rust_roguelike_core::rendering::tile::TileRenderer;
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TileExample {
    texture_id: TextureId,
    index: usize,
}

impl App for TileExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut tile_renderer = TileRenderer::new(self.texture_id);

        renderer.start(BLACK);

        tile_renderer.render_full(renderer, self.index, RED);
        tile_renderer.render_full(renderer, 1, GREEN);
        tile_renderer.render_full(renderer, 2, BLUE);
        tile_renderer.render_ascii(renderer, 40, b'A', YELLOW);

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key);
    }

    fn on_button_released(&mut self, button: MouseButton, index: usize) {
        println!("Released {:?} at {}", button, index);
        self.index = index;
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with tiles");
    let app = Rc::new(RefCell::new(TileExample::default()));

    window.run(app.clone());
}
