use super::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult, get_neighbors};
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
        let mut visited: Vec<Vec<bool>> =
            graph.iter().map(|row| vec![false; row.len()]).collect();
        let mut parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();
        let mut explored_order = Vec::new();

        visited[start.0][start.1] = true;
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
                if visited[neighbor.0][neighbor.1] {
                    continue;
                }

                visited[neighbor.0][neighbor.1] = true;
                parents[neighbor.0][neighbor.1] = Some(current);
                queue.push(Reverse((manhattan_distance(neighbor, end), neighbor)));
            }
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}

fn manhattan_distance(position: Position, end: Position) -> usize {
    position.0.abs_diff(end.0) + position.1.abs_diff(end.1)
}

fn reconstruct_path(
    start: Position,
    end: Position,
    parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut path = vec![end];
    let mut current = end;

    while current != start {
        current = parents[current.0][current.1]
            .expect("every discovered node except the start must have a parent");
        path.push(current);
    }

    path.reverse();
    path
}
