use crate::builder::color::ColorBuilder;
use crate::builder::texture::TextureBuilder;
use crate::shader::load_program;
use crate::texture::load_texture;
use cgmath::ortho;
use glium::{Program, Surface};
use rust_roguelike_core::interface::rendering::{
    AsciiRenderer, ColorRenderer, Renderer, TextureId, TextureRenderer,
};
use rust_roguelike_core::math::color::Color;
use rust_roguelike_core::math::size2d::Size2d;

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

struct TextureData {
    texture: glium::texture::Texture2d,
    builder: TextureBuilder,
}

pub struct GliumRenderer {
    size: Size2d,
    display: glium::Display,
    target: Option<glium::Frame>,
    color_builder: ColorBuilder,
    colored_program: Program,
    textured_program: Program,
    texture_data: Vec<TextureData>,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display, size: Size2d) -> GliumRenderer {
        let colored_program = load_program(&display, "colored.vertex", "colored.fragment");
        let textured_program = load_program(&display, "textured.vertex", "textured.fragment");

        let matrix: cgmath::Matrix4<f32> = ortho(
            0.0,
            size.width() as f32,
            0.0,
            size.height() as f32,
            -1.0,
            1.0,
        );

        GliumRenderer {
            size,
            display,
            target: None,
            color_builder: ColorBuilder::default(),
            colored_program,
            textured_program,
            texture_data: Vec::new(),
            matrix,
        }
    }

    fn render_colored_triangles(&mut self) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer =
            glium::VertexBuffer::new(&self.display, &self.color_builder.vertices).unwrap();

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

    fn render_textured_triangles(&mut self) {
        let target = self.target.as_mut().unwrap();

        let draw_parameters = glium::draw_parameters::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..glium::draw_parameters::DrawParameters::default()
        };

        for data in &self.texture_data {
            let vertex_buffer =
                glium::VertexBuffer::new(&self.display, &data.builder.vertices).unwrap();

            let uniforms = uniform! {
                matrix: Into::<[[f32; 4]; 4]>::into(self.matrix),
                tex: &data.texture,
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
    fn get_size(&self) -> Size2d {
        self.size
    }

    fn start(&mut self, color: Color) {
        let mut target = self.display.draw();
        target.clear_color(color.r(), color.g(), color.b(), 1.0);
        self.target = Some(target);

        self.color_builder.vertices.clear();
        self.texture_data
            .iter_mut()
            .for_each(|x| x.builder.vertices.clear());
    }

    fn finish(&mut self) {
        self.render_colored_triangles();
        self.render_textured_triangles();

        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }

    fn take_screenshot(&self, filename: &str) {
        let image: glium::texture::RawImage2d<u8> = self.display.read_front_buffer().unwrap();
        let image =
            image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned())
                .unwrap();
        let image = image::DynamicImage::ImageRgba8(image).flipv();
        image.save(filename).unwrap();
    }

    fn load_texture(&mut self, filename: &str) -> TextureId {
        let texture = load_texture(&self.display, filename).unwrap();
        let id = self.texture_data.len();

        self.texture_data.push(TextureData {
            texture,
            builder: TextureBuilder::default(),
        });

        id
    }

    fn get_color_renderer(&mut self) -> &mut dyn ColorRenderer {
        &mut self.color_builder
    }

    fn get_texture_renderer(&mut self, id: usize) -> &mut dyn TextureRenderer {
        &mut self.texture_data[id].builder
    }

    fn get_ascii_renderer(&mut self, id: usize) -> &mut dyn AsciiRenderer {
        &mut self.texture_data[id].builder
    }
}

pub fn get_corners(position: [f32; 2], size: [f32; 2]) -> [[f32; 2]; 4] {
    let corner00 = position;
    let corner10 = [position[0] + size[0], position[1]];
    let corner01 = [position[0], position[1] + size[1]];
    let corner11 = [position[0] + size[0], position[1] + size[1]];

    [corner00, corner10, corner01, corner11]
}
