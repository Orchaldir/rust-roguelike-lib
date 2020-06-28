use glium::backend::Facade;

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
