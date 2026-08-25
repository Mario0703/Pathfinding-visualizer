use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::CellState::{self};
use crate::algorithms::get_neighbors;
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
        // Prioritizes positions by f-score, processing the lowest score first.
        let mut priority_queue = BinaryHeap::<Reverse<(u32, Position)>>::new();

        let mut distances_from_start: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();

        let (start_row, start_col) = start;

        distances_from_start[start_row][start_col] = 0;

        let mut manhattan_distances_to_goal: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();

        let start_f_score = manhattan_distance(start, end);

        let mut explored_order = Vec::new();

        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        priority_queue.push(Reverse((start_f_score, start)));

        manhattan_distances_to_goal[start_row][start_col] = manhattan_distance(start, end);

        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();

        while !priority_queue.is_empty() {
            let Reverse((_, current)) = priority_queue.pop().unwrap();
            let (current_row, current_col) = current;

            if visited[current_row][current_col] {
                continue;
            }

            visited[current_row][current_col] = true;

            explored_order.push(current);

            if current == end {
                let path = reconstruct_path(start, end, &parents);

                return SearchResult {
                    explored_order,
                    path: Some(path),
                };
            }

            let (current_row, current_col) = current;
            visited[current_row][current_col] = true;

            for neighbor in get_neighbors(current, graph) {
                let (neighbor_row, neighbor_col) = neighbor;

                //distance from start to neighbor though the current node
                let tentative_distance = distances_from_start[current_row][current_col]
                    + weights[neighbor_row][neighbor_col];
                
                if tentative_distance < distances_from_start[neighbor_row][neighbor_col] {
                    // A shorter path to this neighbor was found, so record it.
                    distances_from_start[neighbor_row][neighbor_col] = tentative_distance;
                    parents[neighbor_row][neighbor_col] = Some(current);

                    let distance_to_goal = manhattan_distance(neighbor, end);
                    let f_score = tentative_distance + distance_to_goal;

                    // Schedule the neighbor to be processed according to its new f-score.
                    priority_queue.push(Reverse((f_score, neighbor)));
                }
            }
        }

        SearchResult {
            explored_order,
            path: None, // did not finde a path
        }
    }
}

fn manhattan_distance(current: Position, end: Position) -> u32 {
    let (current_row, current_col) = current;
    let (end_row, end_col) = end;

    (current_row.abs_diff(end_row) + current_col.abs_diff(end_col)) as u32
}

fn reconstruct_path(
    start: Position,
    end: Position,
    parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut path = vec![end];
    let mut current = end;

    while current != start {
        let (current_row, current_col) = current;

        current = parents[current_row][current_col].expect("Every path node should have a parent");

        path.push(current);
    }

    path.reverse();
    path
}
