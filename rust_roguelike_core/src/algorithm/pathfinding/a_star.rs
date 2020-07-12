use crate::algorithm::pathfinding::{CostCalculator, PathfindingAlgorithm, PathfindingResult};
use crate::math::graph::{Graph, Neighbor};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct AStar {}

impl<N, E> PathfindingAlgorithm<N, E> for AStar {
    fn find<G>(&self, graph: &G, start: usize, goal: usize) -> PathfindingResult<E>
    where
        G: Graph<N, E> + CostCalculator<E>,
    {
        println!("Find a path from {} to {}", start, goal);

        let mut open_nodes = BinaryHeap::new();
        open_nodes.push(OpenNode { index: start, total_cost: 0});

        let mut nodes: HashMap<usize,Node> = HashMap::new();
        nodes.insert(start, Node::default());

        while let Some(node) = open_nodes.pop() {
            if node.index == goal {
                return PathfindingResult::Path {total_cost: 0, neighbors: Vec::new()};
            }

            for neighbor in graph.get_neighbors(node.index) {
                let neighbor_node = nodes
                    .entry(neighbor.index)
                    .or_insert_with(|| Node::new(neighbor.index));

                let cost_to_neighbor = graph.calculate_cost(node.index, neighbor);
                let new_total_cost = node.total_cost + cost_to_neighbor + neighbor_node.heuristic;

                if new_total_cost < neighbor_node.total_cost {
                    neighbor_node.cost_from_previous = cost_to_neighbor;
                    neighbor_node.total_cost = new_total_cost;
                    open_nodes.push(OpenNode { index: start, total_cost: neighbor_node.total_cost});
                }
            }
        }

        return PathfindingResult::NoPathFound
    }
}

#[derive(Copy, Clone)]
struct OpenNode {
    index: usize,
    total_cost: u32,
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for OpenNode {}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct Node {
    cost_from_previous: u32,
    heuristic: u32,
    total_cost: u32,
    previous: Option<usize>,
}
