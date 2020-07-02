extern crate glium;
extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::input::KeyCode;
use rust_roguelike_core::interface::rendering::{Renderer, TextureId, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLUE, GREEN, RED, WHITE, YELLOW};
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct AsciiExample {
    texture_id: TextureId,
}

impl App for AsciiExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let ascii_renderer = renderer.get_ascii_renderer(self.texture_id);
        ascii_renderer.render_u8([200.0, 200.0], [100.0, 100.0], b'a', RED);
        ascii_renderer.render_char([300.0, 200.0], [100.0, 100.0], 'b', GREEN);
        ascii_renderer.render_text([300.0, 500.0], [20.0, 20.0], "Test?", WHITE);
        ascii_renderer.render_text(
            [0.0, 50.0],
            [20.0, 20.0],
            "Non-Ascii Symbols are replaced with '🎉'!",
            YELLOW,
        );
        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key)
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with ascii");
    let app = Rc::new(RefCell::new(AsciiExample::default()));

    window.run(app.clone());
}
