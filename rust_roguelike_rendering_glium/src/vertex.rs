#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub tc: [f32; 2], // texture coordinates
}
