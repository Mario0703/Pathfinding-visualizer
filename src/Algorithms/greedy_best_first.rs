use crate::algorithms::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult, get_neighbors};
use crate::CellState;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct GreedyBestFirstSearch;

impl PathfindingAlgorithm for GreedyBestFirstSearch {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Greedy Best-First Search",
            description: "An unweighted pathfinding algorithm that explores the node with the smallest Manhattan distance to the end. It can find paths quickly, but does not guarantee a shortest or cheapest path.",
            time_complexity: "O((V + E) log V)",
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
        let mut queue = BinaryHeap::new();
        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();
        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();
        let mut explored_order = Vec::new();
        let (start_row, start_column) = start;

        visited[start_row][start_column] = true;
        queue.push(Reverse((manhattan_distance(start, end), start)));

        while !queue.is_empty() {
            let Reverse((_, current)) = queue
                .pop()
                .expect("the queue must contain an item while it is not empty");

            explored_order.push(current);

            if current == end {
                return SearchResult {
                    explored_order,
                    path: Some(reconstruct_path(start, end, &parents)),
                };
            }

            for neighbor in get_neighbors(current, graph) {
                let (neighbor_row, neighbor_column) = neighbor;

                if visited[neighbor_row][neighbor_column] {
                    continue;
                }

                visited[neighbor_row][neighbor_column] = true;
                parents[neighbor_row][neighbor_column] = Some(current);

                let neighbor_distance_to_end = manhattan_distance(neighbor, end);
                queue.push(Reverse((neighbor_distance_to_end, neighbor)));
            }
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}

fn manhattan_distance(position: Position, end: Position) -> usize {
    let (position_row, position_column) = position;
    let (end_row, end_column) = end;

    position_row.abs_diff(end_row) + position_column.abs_diff(end_column)
}

fn reconstruct_path(
    start: Position,
    end: Position,
    parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut reconstructed_path = vec![end];
    let mut current_path_position = end;

    while current_path_position != start {
        let (current_row, current_column) = current_path_position;

        current_path_position = parents[current_row][current_column]
            .expect("Every discovered node except the start must have a parent");

        reconstructed_path.push(current_path_position);
    }

    reconstructed_path.reverse();
    reconstructed_path
}
