extern crate glium;
extern crate rust_roguelike_rendering_glium;

use rust_roguelike_core::interface::rendering::Renderer;
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::color::{BLUE, GREEN};
use rust_roguelike_core::math::size2d::Size2d;
use rust_roguelike_rendering_glium::renderer::GliumRenderer;

#[derive(Default)]
pub struct MapApp {}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        renderer.render_triangle([400.0, 300.0], [600.0, 300.0], [500.0, 400.0], GREEN);
        renderer.finish();
    }
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let size = Size2d::new(800, 600);
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(size.x(), size.y());
    let wb = glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(window_size);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut app = MapApp::default();

    let mut renderer = GliumRenderer::new(display, size);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::RedrawRequested(_) => (),
            _ => return,
        }

        app.render(&mut renderer);
    });
}
