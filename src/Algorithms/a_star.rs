use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::CellState::{self};
use crate::algorithms::{get_neighbors};
use std::vec;
use std::{cmp::Reverse, collections::BinaryHeap};

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
        graph: &[Vec<CellState>],
        weights: &[Vec<u32>],
    ) -> SearchResult {
        let mut queue = BinaryHeap::<Reverse<(u32, Position)>>::new();

        let mut distances: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();
        distances[start.0][start.1] = 0;

        let mut manhattan_heuristics: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();
        let start_f_score = distances[start.0][start.1] + manhattan_heuristics[start.0][start.1];

        let mut explored_order = Vec::new();

        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        queue.push(Reverse((start_f_score, start)));

        manhattan_heuristics[start.0][start.1] = manhattan_distance_helper(start, end);

        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();

        while !queue.is_empty() {
            let Reverse((_, current)) = queue.pop().unwrap();
            visited[current.0][current.1] = true;
            explored_order.push(current);

            if current == end {
                let path = reconstruct_path_helper(start, end, &parents);

                return SearchResult {
                    explored_order,
                    path: Some(path),
                };
            }

            for neighbor in get_neighbors(current, graph) {
                let distance_from_start_to_neighbor_trough_current =
                    distances[current.0][current.1] + weights[neighbor.0][neighbor.1];
                    

                if distance_from_start_to_neighbor_trough_current < distances[neighbor.0][neighbor.1] {
                    distances[neighbor.0][neighbor.1] = distance_from_start_to_neighbor_trough_current;
                    parents[neighbor.0][neighbor.1] = Some(current);

                    let f_score = distance_from_start_to_neighbor_trough_current + manhattan_distance_helper(neighbor, end);

                    manhattan_heuristics[neighbor.0][neighbor.1] = f_score;

                    queue.push(Reverse((f_score, neighbor)));
                }
            }
        }

    SearchResult {
        explored_order,
        path: None, // did not finde a path
    }
    }
}

fn manhattan_distance_helper(current_cell: Position, end_cell: Position) -> u32 {
    (current_cell.0.abs_diff(end_cell.0) + current_cell.1.abs_diff(end_cell.1)) as u32
}

fn reconstruct_path_helper(
    start: Position,
    end: Position,
    parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut path = vec![end];
    let mut current = end;
    while current != start {
        current = parents[current.0][current.1].expect("Every path node should have a parent");
        path.push(current)
    }
    path.reverse();
    path
}
