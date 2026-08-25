use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::CellState;
use crate::algorithms::get_neighbors;
use std::collections::VecDeque;

pub struct BFS;

impl PathfindingAlgorithm for BFS {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Breadth-First Search",
            description: "A pathfinding algorithm that explores all neighbors at the present depth prior to moving on to nodes at the next depth level.",
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
        let mut queue = VecDeque::new();

        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();

        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        let mut explored_order = Vec::new();

        let (start_row, start_column) = start;

        visited[start_row][start_column] = true;
        queue.push_back(start);

        while !queue.is_empty() {
            let current = queue
                .pop_front()
                .expect("The queue should contain an element");

            explored_order.push(current);

            if current == end {
                return SearchResult {
                    explored_order,
                    path: Some(reconstruct_path(start, end, &parents)),
                };
            }

            for neighbor in get_neighbors(current, graph) {
                let (neighbor_row, neighbor_column) = neighbor;

                if !visited[neighbor_row][neighbor_column] {
                    visited[neighbor_row][neighbor_column] = true;
                    parents[neighbor_row][neighbor_column] = Some(current);
                    queue.push_back(neighbor);
                }
            }
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}

fn reconstruct_path(
    start: Position,
    end: Position,
    parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut path = vec![end];
    let mut current = end;

    while current != start {
        let (current_row, current_column) = current;

        current = parents[current_row][current_column]
            .expect("Every discovered node except the start must have a parent");

        path.push(current);
    }

    path.reverse();
    path
}
