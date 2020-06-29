#[macro_use]
extern crate glium;
extern crate rust_roguelike_core;

pub mod renderer;
pub mod shader;
mod texture;
pub mod vertex;
pub mod window;

use crate::vertex::ColoredVertex;
use crate::vertex::TexturedVertex;

implement_vertex!(ColoredVertex, position, color);
implement_vertex!(TexturedVertex, position, color, tc);
