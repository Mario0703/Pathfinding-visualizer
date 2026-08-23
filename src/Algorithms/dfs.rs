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
                SearchResult {
            explored_order: Vec::new(),
            path: None,
        }
    }
}

pub fn get_neighbors(position: Position, graph: &[Vec<CellState>]) -> Vec<Position> {
    let left = (Some(position.0), position.1.checked_sub(1));

    let right = (Some(position.0), position.1.checked_add(1));

    let up = (position.0.checked_sub(1), Some(position.1));

    let down = (position.0.checked_add(1), Some(position.1));

    let possible_neighbors = vec![left, right, up, down];
    let mut neighbors = Vec::new();

    for neighbor in possible_neighbors {
        if let (Some(row), Some(column)) = neighbor {
            if row < graph.len()
                && column < graph[row].len()
                && graph[row][column] != CellState::Wall
            {
                neighbors.push((row, column));
            }
        }
    }

    neighbors
}
