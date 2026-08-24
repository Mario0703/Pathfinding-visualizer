use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::algorithms::{Dijkstra, PathfindingAlgorithm};

#[test]
fn dijkstra_finds_a_path_across_an_empty_grid() {
    let grid = empty_grid(3, 3);
    let start = (0, 0);
    let end = (2, 2);

    let result = Dijkstra.find_path(start, end, &grid);

    assert_eq!(result.explored_order.first(), Some(&start));
    assert!(result.explored_order.contains(&end));

    let path = result
        .path
        .expect("Dijkstra should find a path across an empty grid");

    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 5, "Dijkstra should return a shortest path");
}
