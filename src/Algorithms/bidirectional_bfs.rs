use crate::algorithms::{AlgorithmInfo, PathfindingAlgorithm, Position, SearchResult, get_neighbors};
use crate::CellState;
use std::collections::VecDeque;

pub struct BidirectionalBFS;

impl PathfindingAlgorithm for BidirectionalBFS {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Bidirectional Breadth-First Search",
            description: "An unweighted pathfinding algorithm that runs breadth-first searches from the start and end until their explored regions meet.",
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
        if start == end {
            return SearchResult {
                explored_order: vec![start],
                path: Some(vec![start]),
            };
        }

        let mut start_queue = VecDeque::new();
        let mut end_queue = VecDeque::new();

        start_queue.push_back(start);
        end_queue.push_back(end);

        let mut start_visited: Vec<Vec<bool>> =
            graph.iter().map(|row| vec![false; row.len()]).collect();

        let mut end_visited: Vec<Vec<bool>> =
            graph.iter().map(|row| vec![false; row.len()]).collect();

        let mut start_parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        let mut end_parents: Vec<Vec<Option<Position>>> =
            graph.iter().map(|row| vec![None; row.len()]).collect();

        let mut explored_order = Vec::new();

        let (start_row, start_column) = start;
        let (end_row, end_column) = end;

        start_visited[start_row][start_column] = true;
        end_visited[end_row][end_column] = true;

        let mut both_queues_have_nodes = !start_queue.is_empty() && !end_queue.is_empty();

        while both_queues_have_nodes {
            let start_meeting_point = expand_one_bfs_level(
                &mut start_queue,
                &mut start_visited,
                &end_visited,
                &mut start_parents,
                graph,
                &mut explored_order,
            );

            if start_meeting_point.is_some() {
                let meeting_point = start_meeting_point.expect("A meeting point should exist");

                let path =
                    reconstruct_path(start, end, meeting_point, &start_parents, &end_parents);

                return SearchResult {
                    explored_order,
                    path: Some(path),
                };
            }

            let end_meeting_point = expand_one_bfs_level(
                &mut end_queue,
                &mut end_visited,
                &start_visited,
                &mut end_parents,
                graph,
                &mut explored_order,
            );

            if end_meeting_point.is_some() {
                let meeting_point = end_meeting_point.expect("A meeting point should exist");

                let path =
                    reconstruct_path(start, end, meeting_point, &start_parents, &end_parents);

                return SearchResult {
                    explored_order,
                    path: Some(path),
                };
            }

            both_queues_have_nodes = !start_queue.is_empty() && !end_queue.is_empty();
        }

        SearchResult {
            explored_order,
            path: None,
        }
    }
}

/// Expands one BFS level and returns the meeting point with the opposite search.
fn expand_one_bfs_level(
    queue: &mut VecDeque<Position>,
    visited: &mut [Vec<bool>],
    other_visited: &[Vec<bool>],
    parents: &mut [Vec<Option<Position>>],
    graph: &[Vec<CellState>],
    explored_order: &mut Vec<Position>,
) -> Option<Position> {
    let level_size = queue.len();

    for _ in 0..level_size {
        let current = queue
            .pop_front()
            .expect("the current BFS level must contain level_size nodes");
        explored_order.push(current);

        for neighbor in get_neighbors(current, graph) {
            let (neighbor_row, neighbor_column) = neighbor;

            if other_visited[neighbor_row][neighbor_column] {
                if !visited[neighbor_row][neighbor_column] {
                    visited[neighbor_row][neighbor_column] = true;
                    parents[neighbor_row][neighbor_column] = Some(current);
                }

                return Some(neighbor);
            }

            if !visited[neighbor_row][neighbor_column] {
                visited[neighbor_row][neighbor_column] = true;
                parents[neighbor_row][neighbor_column] = Some(current);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn reconstruct_path(
    start: Position,
    end: Position,
    meeting_point: Position,
    start_parents: &[Vec<Option<Position>>],
    end_parents: &[Vec<Option<Position>>],
) -> Vec<Position> {
    let mut path = vec![meeting_point];
    let mut current = meeting_point;

    while current != start {
        current = start_parents[current.0][current.1]
            .expect("every node between the meeting point and start must have a parent");
        path.push(current);
    }
    path.reverse();

    current = meeting_point;
    while current != end {
        current = end_parents[current.0][current.1]
            .expect("every node between the meeting point and end must have a parent");
        path.push(current);
    }

    path
}
