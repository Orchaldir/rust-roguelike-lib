use crate::shader::load_program;
use crate::texture::load_texture;
use crate::vertex::{ColoredVertex, TexturedVertex};
use cgmath::ortho;
use glium::{Program, Surface};
use rust_roguelike_core::interface::rendering::{Renderer, TextureId};
use rust_roguelike_core::math::color::Color;
use rust_roguelike_core::math::size2d::Size2d;

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

struct TextureData {
    texture: glium::texture::Texture2d,
    vertices: Vec<TexturedVertex>,
}

impl TextureData {
    pub fn add_triangle(
        &mut self,
        a: [f32; 2],
        b: [f32; 2],
        c: [f32; 2],
        tc_a: [f32; 2],
        tc_b: [f32; 2],
        tc_c: [f32; 2],
        color: Color,
    ) {
        self.add(a, tc_a, color);
        self.add(b, tc_b, color);
        self.add(c, tc_c, color);
    }

    fn add(&mut self, position: [f32; 2], tc: [f32; 2], color: Color) {
        self.vertices.push(TexturedVertex {
            position,
            color: color.into(),
            tc,
        });
    }
}

pub struct GliumRenderer {
    display: glium::Display,
    target: Option<glium::Frame>,
    colored_program: Program,
    colored_vertices: Vec<ColoredVertex>,
    textured_program: Program,
    texture_data: Vec<TextureData>,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display, size: Size2d) -> GliumRenderer {
        let colored_program = load_program(&display, "colored.vertex", "colored.fragment");
        let textured_program = load_program(&display, "textured.vertex", "textured.fragment");

        let matrix: cgmath::Matrix4<f32> =
            ortho(0.0, size.x() as f32, 0.0, size.y() as f32, -1.0, 1.0);

        GliumRenderer {
            display,
            target: None,
            colored_program,
            colored_vertices: Vec::new(),
            textured_program,
            texture_data: Vec::new(),
            matrix,
        }
    }

    fn add(&mut self, position: [f32; 2], color: Color) {
        self.colored_vertices.push(ColoredVertex {
            position,
            color: color.into(),
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

    fn render_textured(&mut self) {
        for data in &self.texture_data {
            let target = self.target.as_mut().unwrap();
            let vertex_buffer = glium::VertexBuffer::new(&self.display, &data.vertices).unwrap();

            let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix),
            tex: &data.texture,
            };

            let draw_parameters = glium::draw_parameters::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                ..glium::draw_parameters::DrawParameters::default()
            };

            target
                .draw(
                    &vertex_buffer,
                    &INDICES,
                    &self.textured_program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }
}

impl Renderer for GliumRenderer {
    fn start(&mut self, color: Color) {
        let mut target = self.display.draw();
        target.clear_color(color.r(), color.g(), color.b(), 1.0);
        self.target = Some(target);

        self.colored_vertices.clear();
        self.texture_data
            .iter_mut()
            .for_each(|x| x.vertices.clear());
    }

    fn finish(&mut self) {
        self.render_colored();
        self.render_textured();

        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }

    fn load_texture(&mut self, filename: &str) -> TextureId {
        let texture = load_texture(&self.display, filename).unwrap();
        let id = self.texture_data.len();

        self.texture_data.push(TextureData {
            texture,
            vertices: Vec::new(),
        });

        id
    }

    fn render_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: Color) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    fn render_texture(
        &mut self,
        id: TextureId,
        position: [f32; 2],
        size: [f32; 2],
        tc: [f32; 2],
        tc_size: [f32; 2],
        color: Color,
    ) {
        let [c00, c10, c01, c11] = get_corners(position, size);
        let [tc00, tc10, tc01, tc11] = get_corners(tc, tc_size);

        let data = &mut self.texture_data[id];

        data.add_triangle(c00, c10, c11, tc00, tc10, tc11, color);
        data.add_triangle(c00, c11, c01, tc00, tc11, tc01, color);
    }
}

fn get_corners(position: [f32; 2], size: [f32; 2]) -> [[f32; 2]; 4] {
    let corner00 = position;
    let corner10 = [position[0] + size[0], position[1]];
    let corner01 = [position[0], position[1] + size[1]];
    let corner11 = [position[0] + size[0], position[1] + size[1]];

    [corner00, corner10, corner01, corner11]
}
