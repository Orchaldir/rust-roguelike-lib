use crate::input::convert_key_code;
use crate::renderer::GliumRenderer;
use glium::{glutin, Display};
use rust_roguelike_core::interface::rendering::Window;
use rust_roguelike_core::interface::App;
use rust_roguelike_core::math::size2d::Size2d;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    size: Size2d,
}

impl GliumWindow {
    pub fn new(title: &'static str, size: Size2d) -> GliumWindow {
        GliumWindow { title, size }
    }

    pub fn default_size(title: &'static str) -> GliumWindow {
        GliumWindow::new(title, Size2d::new(800, 600))
    }

    fn create_display(&self, event_loop: &glutin::event_loop::EventLoop<()>) -> Display {
        let size = glutin::dpi::LogicalSize::new(self.size.x(), self.size.y());
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        glium::Display::new(wb, cb, event_loop).unwrap()
    }
}

impl Window for GliumWindow {
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        let event_loop = glutin::event_loop::EventLoop::new();
        let display = self.create_display(&event_loop);
        let mut renderer = GliumRenderer::new(display, self.size);

        {
            let mut reference = app.borrow_mut();
            reference.init(&mut renderer);
        }

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
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == glutin::event::ElementState::Released {
                            if let Some(glutin_key) = input.virtual_keycode {
                                if let Some(key) = convert_key_code(glutin_key) {
                                    let mut reference = app.borrow_mut();
                                    reference.on_key_released(key);
                                }
                            }
                        }
                    }
                    _ => return,
                },
                glutin::event::Event::RedrawRequested(_) => (),
                _ => return,
            }

            let mut reference = app.borrow_mut();
            reference.render(&mut renderer);
        });
    }
}
