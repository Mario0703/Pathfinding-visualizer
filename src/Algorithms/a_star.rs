use std::vec;
use libm::math::support::int_traits::Int::abs;
use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::CellState;
use crate::algorithms::get_neighbors;

pub struct AStar;

impl PathfindingAlgorithm for AStar {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "A* Search",
            description: "A pathfinding algorithm that uses path cost and a heuristic estimate to efficiently find the shortest path to a target node.",
            time_complexity: "O((V + E) log V)",
            space_complexity: "O(V)",
        }
    }

    fn find_path(
        &self,
        start: Position,
        end: Position,
        grid: &[Vec<CellState>],
        weights: &[Vec<u32>],
    ) -> SearchResult {
        SearchResult { explored_order: vec![(1 as usize,1 as usize)], path: None }
    }
}

fn manhatten_distance_helper(current_cell : Position, end_cell : Position) -> u32{
    let h = abs(current_cell.0 - end_cell.0) - abs(current_cell.0 - end_cell.1);
    h
}
