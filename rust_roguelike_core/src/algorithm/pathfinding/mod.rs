pub mod a_star;

use crate::math::graph::{Graph, Neighbor};
use std::fmt::Debug;

#[derive(Debug)]
pub enum PathfindingResult {
    GoalAlreadyReached,
    NotSearched,
    NoPathFound,
    Path {
        total_cost: u32,
        indices: Vec<usize>,
    },
}

impl Default for PathfindingResult {
    fn default() -> Self {
        PathfindingResult::NotSearched
    }
}

pub trait CostCalculator<E> {
    fn is_valid(&self, index: usize) -> bool;

    fn calculate_cost(&self, index: usize, neighbor: &Neighbor<E>) -> u32;
}

pub trait PathfindingAlgorithm<N, E> {
    fn find<G>(&self, graph: &G, start: usize, goal: usize) -> PathfindingResult
    where
        G: Graph<N, E> + CostCalculator<E>,
        E: Debug;
}
