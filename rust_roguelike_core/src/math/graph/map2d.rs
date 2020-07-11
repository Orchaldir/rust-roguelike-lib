use crate::math::graph::Graph;
use crate::math::size2d::Size2d;

/// A trait to represent a 2d map
pub trait Map2d<N, E>: Graph<N, E> {
    /// Returns the map's size in 2d
    fn get_size_2d(&self) -> Size2d;
}

/// The direction between neighbors in a Map2d
pub enum Direction2d {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
