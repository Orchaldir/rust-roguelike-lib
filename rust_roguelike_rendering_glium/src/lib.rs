#[macro_use]
extern crate glium;
extern crate rust_roguelike_core;

pub mod shader;
pub mod vertex;

use crate::vertex::ColoredVertex;

implement_vertex!(ColoredVertex, position, color);
