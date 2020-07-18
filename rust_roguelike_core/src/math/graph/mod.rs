pub mod map2d;
pub mod occupancy;

/// A trait to represent a graph
pub trait Graph<N, E> {
    /// Returns the graph's number of nodes
    fn get_size(&self) -> usize;

    /// Returns a node of the graph
    fn get_node(&self, index: usize) -> Option<&N>;

    /// Returns the neighbors of a node
    fn get_neighbors(&self, index: usize) -> Vec<Neighbor<E>>;
}

/// The neighbor of one node of the graph
#[derive(Clone, Copy, Debug)]
pub struct Neighbor<T> {
    pub index: usize,
    pub edge: T,
}
