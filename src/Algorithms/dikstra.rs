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

        let mut visited: Vec<Vec<bool>> = graph.iter().map(|row| vec![false; row.len()]).collect();

        let mut distances: Vec<Vec<u32>> =
            graph.iter().map(|row| vec![u32::MAX; row.len()]).collect();
        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        distances[start.0][start.1] = 0;
        queue.push(Reverse((0, start)));

        while !queue.is_empty() {
            let Reverse((current_distance, current)) = queue.pop().unwrap();

            if visited[current.0][current.1] {
                continue; // Skip already visited nodes
            }

            visited[current.0][current.1] = true; // Mark the current node as visited
            explored_order.push(current); // We have explored this node, so we add it to the explored orderee the shortest or cheapest path.
            if current == end {
                // we are at the end, build path from end to start using parents, reverse path to get final path as we are starting from the end
                let mut path = vec![end];
                let mut position = end;

                while position != start {
                    // while we have not reached the start, keep going up the parents to build the path
                    position =
                        parents[position.0][position.1].expect("reached node should have a parent");
                    path.push(position);
                }

                path.reverse();

                return SearchResult {
                    explored_order,
                    path: Some(path),
                };
            }

            for neighbor in get_neighbors(current, graph) {
                if visited[neighbor.0][neighbor.1] {
                    // Skip already visited neighbors
                    continue;
                }
                let distance_to_neighbor = weights[neighbor.0][neighbor.1]; // Get the weight of the edge to the neighbor
                let alternative_distance = current_distance + distance_to_neighbor;

                if alternative_distance < distances[neighbor.0][neighbor.1] {
                    distances[neighbor.0][neighbor.1] = alternative_distance;
                    parents[neighbor.0][neighbor.1] = Some(current);

                    queue.push(Reverse((alternative_distance, neighbor)));
                }
            }
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}
