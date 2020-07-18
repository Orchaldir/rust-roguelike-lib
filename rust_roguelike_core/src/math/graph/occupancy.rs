use crate::algorithm::pathfinding::CostCalculator;
use crate::math::graph::map2d::{Direction2d, Map2d};
use crate::math::graph::{Graph, Neighbor};
use crate::math::size2d::Size2d;

#[derive(Default)]
pub struct OccupancyMap {
    pub is_occupied: Vec<bool>,
    size: Size2d,
}

impl OccupancyMap {
    pub fn new(size: Size2d, default: bool) -> Self {
        let cells = vec![default; size.get_tiles()];
        OccupancyMap {
            is_occupied: cells,
            size,
        }
    }

    pub fn add_border(&mut self) {
        self.add_rectangle(0, 0, self.size.width(), self.size.height(), true)
    }

    pub fn add_rectangle(
        &mut self,
        start_x: u32,
        start_y: u32,
        width: u32,
        height: u32,
        value: bool,
    ) {
        let end_x = start_x + width;
        let end_y = start_y + height;

        for x in start_x..end_x {
            self.is_occupied[self.size.to_index(x, start_y)] = value;
            self.is_occupied[self.size.to_index(x, end_y - 1)] = value;
        }

        for y in start_y..end_y {
            self.is_occupied[self.size.to_index(start_x, y)] = value;
            self.is_occupied[self.size.to_index(end_x - 1, y)] = value;
        }
    }

    fn add_neighbor(
        &self,
        neighbors: &mut Vec<Neighbor<Direction2d>>,
        point: [i32; 2],
        dir: Direction2d,
        dx: i32,
        dy: i32,
    ) {
        let index = self
            .size
            .to_index((point[0] + dx) as u32, (point[1] + dy) as u32);

        if self.is_valid(index) {
            neighbors.push(Neighbor { index, edge: dir });
        }
    }
}

impl CostCalculator<Direction2d> for OccupancyMap {
    fn is_valid(&self, index: usize) -> bool {
        return !*self.is_occupied.get(index).unwrap_or(&true);
    }

    fn calculate_cost(&self, _index: usize, _neighbor: &Neighbor<Direction2d>) -> u32 {
        1
    }
}

impl Graph<bool, Direction2d> for OccupancyMap {
    fn get_size(&self) -> usize {
        self.size.get_tiles()
    }

    fn get_node(&self, index: usize) -> Option<&bool> {
        self.is_occupied.get(index)
    }

    fn get_neighbors(&self, index: usize) -> Vec<Neighbor<Direction2d>> {
        let [x, y] = self.size.to_point(index);
        let point = [x as i32, y as i32];
        let mut neighbors = Vec::new();

        self.add_neighbor(&mut neighbors, point, Direction2d::NORTH, 0, 1);
        self.add_neighbor(&mut neighbors, point, Direction2d::EAST, 1, 0);
        self.add_neighbor(&mut neighbors, point, Direction2d::SOUTH, 0, -1);
        self.add_neighbor(&mut neighbors, point, Direction2d::WEST, -1, 0);

        neighbors
    }
}

impl Map2d<bool, Direction2d> for OccupancyMap {
    fn get_size_2d(&self) -> Size2d {
        self.size
    }
}
