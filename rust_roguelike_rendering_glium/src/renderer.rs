use crate::shader::load_program;
use crate::vertex::ColoredVertex;
use cgmath::ortho;
use glium::{Program, Surface};
use rust_roguelike_core::interface::rendering::Renderer;
use rust_roguelike_core::math::color::Color;
use rust_roguelike_core::math::size2d::Size2d;

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct GliumRenderer {
    display: glium::Display,
    target: Option<glium::Frame>,
    colored_program: Program,
    colored_vertices: Vec<ColoredVertex>,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display, size: Size2d) -> GliumRenderer {
        let colored_program = load_program(&display, "colored.vertex", "colored.fragment");

        let matrix: cgmath::Matrix4<f32> =
            ortho(0.0, size.x() as f32, 0.0, size.y() as f32, -1.0, 1.0);

        GliumRenderer {
            display,
            target: None,
            colored_program,
            colored_vertices: Vec::new(),
            matrix,
        }
    }

    fn add(&mut self, position: [f32; 2], color: Color) {
        self.colored_vertices.push(ColoredVertex {
            position,
            color: color.to_array(),
        });
    }

    fn render_colored(&mut self) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer =
            glium::VertexBuffer::new(&self.display, &self.colored_vertices).unwrap();

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix)
        };

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.colored_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

impl Renderer for GliumRenderer {
    fn start(&mut self, color: Color) {
        let mut target = self.display.draw();
        target.clear_color(color.r(), color.g(), color.b(), 1.0);
        self.target = Some(target);

        self.colored_vertices.clear()
    }

    fn finish(&mut self) {
        self.render_colored();

        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }

    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }
}
