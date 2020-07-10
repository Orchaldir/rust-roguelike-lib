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
    take_screenshot: bool,
}

impl App for AsciiExample {
    fn init(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let ascii_renderer = renderer.get_ascii_renderer(self.texture_id);
        ascii_renderer.render_u8([10.0, 10.0], [5.0, 5.0], b'a', RED);
        ascii_renderer.render_char([15.0, 10.0], [5.0, 5.0], 'b', GREEN);
        ascii_renderer.render_text([15.0, 25.0], [1.0, 1.0], "Test?", WHITE);
        ascii_renderer.render_text(
            [0.0, 2.5],
            [1.0, 1.0],
            "Non-Ascii Symbols are replaced with '🎉'!",
            YELLOW,
        );
        renderer.finish();

        if self.take_screenshot {
            println!("Take screenshot");
            renderer.take_screenshot("ascii.png");
            self.take_screenshot = false;
        }
    }

    fn on_key_released(&mut self, key: KeyCode) {
        println!("Released {:?}", key);

        if key == KeyCode::Snapshot {
            self.take_screenshot = true;
        }
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with ascii");
    let app = Rc::new(RefCell::new(AsciiExample::default()));

    window.run(app.clone());
}
