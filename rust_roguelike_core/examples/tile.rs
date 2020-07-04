extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::input::KeyCode;
use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLACK, BLUE, GREEN, RED};
use rust_roguelike_core::rendering::tile::TileRenderer;
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TileExample {
    texture_id: TextureId,
    tile_renderer: TileRenderer,
}

impl App for TileExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        self.tile_renderer.render_full(renderer, 0, RED);
        self.tile_renderer.render_full(renderer, 1, GREEN);
        self.tile_renderer.render_full(renderer, 2, BLUE);

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key)
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with tiles");
    let app = Rc::new(RefCell::new(TileExample::default()));

    window.run(app.clone());
}
