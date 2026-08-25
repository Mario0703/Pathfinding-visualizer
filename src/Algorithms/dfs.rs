use crate::algorithms::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::CellState;
use crate::algorithms::get_neighbors;

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
        start: Position,
        end: Position,
        graph: &[Vec<CellState>],
        _weights: &[Vec<u32>],
    ) -> SearchResult {
        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();

        let mut explored_order: Vec<Position> = Vec::new();
        let path = dfs(start, end, graph, &mut visited, &mut explored_order);

        SearchResult {
            explored_order,
            path,
        }
    }
}
fn dfs(
    current: Position,
    end: Position,
    graph: &[Vec<CellState>],
    visited: &mut Vec<Vec<bool>>,
    explored_order: &mut Vec<Position>,
) -> Option<Vec<Position>> {
    let (current_row, current_column) = current;

    visited[current_row][current_column] = true;
    explored_order.push(current);

    if current == end {
        return Some(vec![current]);
    }

    for neighbor in get_neighbors(current, graph) {
        let (neighbor_row, neighbor_column) = neighbor;

        if !visited[neighbor_row][neighbor_column] {
            if let Some(mut path) = dfs(neighbor, end, graph, visited, explored_order) {
                // Prepend the current position to the path.
                path.insert(0, current);
                return Some(path);
            }
        }
    }

    None
}
