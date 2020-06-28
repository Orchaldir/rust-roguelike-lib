#[macro_use]
extern crate glium;
extern crate rust_roguelike_core;

pub mod renderer;
pub mod shader;
pub mod vertex;
pub mod window;

use crate::vertex::ColoredVertex;

implement_vertex!(ColoredVertex, position, color);
