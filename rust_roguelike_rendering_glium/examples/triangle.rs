extern crate glium;
extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::rendering::{Renderer, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLUE, GREEN, RED, YELLOW};
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TriangleExample {}

impl App for TriangleExample {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        let color_renderer = renderer.get_color_renderer();
        color_renderer.render_triangle([20.0, 15.0], [30.0, 15.0], [25.0, 20.0], GREEN);
        color_renderer.render_triangle([5.0, 15.0], [15.0, 15.0], [10.0, 20.0], RED);
        color_renderer.render_rectangle([15.0, 2.0], [7.0, 2.5], YELLOW);
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Triangles");
    let app = Rc::new(RefCell::new(TriangleExample::default()));

    window.run(app.clone());
}
