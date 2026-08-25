use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult};
use crate::{CellState, algorithms::get_neighbors};
use std::{cmp::Reverse, collections::BinaryHeap};
pub struct Dijkstra;

impl PathfindingAlgorithm for Dijkstra {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Dijkstra's Algorithm",
            description: "A pathfinding algorithm that finds the shortest path from a starting node to all other nodes in a weighted graph.",
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
        let mut explored_order = Vec::new();
        let mut visited: Vec<Vec<bool>> = 
            graph.iter().map(|row| vec![false; row.len()]).collect();
        let mut distances: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();
        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        let (start_row, start_column) = start;
        distances[start_row][start_column] = 0;
        queue.push(Reverse((0, start)));

        while !queue.is_empty() {
            let Reverse((current_distance, current)) = queue.pop().unwrap();
            let (current_row, current_coloumn) = current;

            if visited[current_row][current_coloumn] {
                continue; // Skip already visited nodes
            }

            visited[current_row][current_coloumn] = true; // Mark the current node as visited
            explored_order.push(current); // We have explored this node, so we add it to the explored orderee the shortest or cheapest path.
            if current == end {
                // Follow the parent links from the end back to the start.
                let mut reconstructed_path = vec![end];
                let mut current_path_position = end;

                while current_path_position != start {
                    let (path_row, path_column) = current_path_position;

                    current_path_position = parents[path_row][path_column]
                        .expect("Every reached node except the start should have a parent");

                    reconstructed_path.push(current_path_position);
                }

                // Parent links produced the path in reverse order.
                reconstructed_path.reverse();

                return SearchResult {
                    explored_order,
                    path: Some(reconstructed_path),
                };
            }

            for neighbor in get_neighbors(current, graph) {
                let (neighbor_row, neighbor_column) = neighbor;

                if visited[neighbor_row][neighbor_column] {
                    continue;
                }

                let neighbor_weight = weights[neighbor_row][neighbor_column];
                let tentative_distance = current_distance + neighbor_weight;

                if tentative_distance < distances[neighbor_row][neighbor_column] {
                    distances[neighbor_row][neighbor_column] = tentative_distance;
                    parents[neighbor_row][neighbor_column] = Some(current);

                    queue.push(Reverse((tentative_distance, neighbor)));
                }
            }
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}
