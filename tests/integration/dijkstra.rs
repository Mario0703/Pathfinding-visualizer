use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::algorithms::{Dijkstra, PathfindingAlgorithm};

#[test]
fn dijkstra_finds_a_path_across_an_empty_grid() {
    let grid = empty_grid(3, 3);
    let weights = vec![vec![1; 3]; 3];
    let start = (0, 0);
    let end = (2, 2);

    let result = Dijkstra.find_path(start, end, &grid, &weights);

    assert_eq!(result.explored_order.first(), Some(&start));
    assert!(result.explored_order.contains(&end));

    let path = result
        .path
        .expect("Dijkstra should find a path across an empty grid");

    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 5, "Dijkstra should return a shortest path");
}

#[test]
fn dijkstra_prefers_a_longer_route_with_a_lower_total_weight() {
    let grid = empty_grid(2, 5);
    let weights = vec![vec![1, 15, 15, 15, 1], vec![1, 1, 1, 1, 1]];
    let start = (0, 0);
    let end = (0, 4);

    let result = Dijkstra.find_path(start, end, &grid, &weights);
    let path = result.path.expect("Dijkstra should find a weighted path");

    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 7);
    assert!(path.contains(&(1, 2)));
    assert!(!path.contains(&(0, 1)));
    assert!(!path.contains(&(0, 2)));
    assert!(!path.contains(&(0, 3)));
}
