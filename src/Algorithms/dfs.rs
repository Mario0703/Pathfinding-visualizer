use crate::CellState;

use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};

pub struct DFS;

impl PathfindingAlgorithm for DFS {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Depth-First Search",
            description: "A pathfinding algorithm that explores as far as possible along each branch before backtracking.",
            time_complexity: "O(V + E)",
            space_complexity: "O(V)",
        }
    }

    fn find_path(
        &self,
        _start: Position,
        _end: Position,
        _graph: &[Vec<CellState>],
    ) -> SearchResult {
        todo!("implement DFS pathfinding")
    }
}
