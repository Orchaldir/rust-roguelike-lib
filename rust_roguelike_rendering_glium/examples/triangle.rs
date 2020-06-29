extern crate glium;
extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::rendering::{Renderer, Window};
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLUE, GREEN, RED};
use rust_roguelike_rendering_glium::window::GliumWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TriangleApp {}

impl App for TriangleApp {
    fn init(&mut self, _: &mut dyn Renderer) {}

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        renderer.render_triangle([400.0, 300.0], [600.0, 300.0], [500.0, 400.0], GREEN);
        renderer.render_triangle([100.0, 300.0], [300.0, 300.0], [200.0, 400.0], RED);
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Triangles");
    let app = Rc::new(RefCell::new(TriangleApp::default()));

    window.run(app.clone());
}
