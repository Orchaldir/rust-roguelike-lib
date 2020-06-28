use glium::backend::Facade;
use std::fs;

pub fn get_default_program<F: Facade>(display: &F) -> glium::Program {
    let vertex_shader = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.08, 0.58, 1.0);
        }
    "#;

    glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap()
}

pub fn load_program<F: Facade>(
    display: &F,
    vertex_file: &str,
    fragment_file: &str,
) -> glium::Program {
    let path = "resources/shader/";
    let vertex_shader =
        fs::read_to_string([path, vertex_file].concat()).expect("Could not load vertex shader");
    let fragment_shader =
        fs::read_to_string([path, fragment_file].concat()).expect("Could not load vertex shader");

    glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap()
}
