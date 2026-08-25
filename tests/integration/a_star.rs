use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::algorithms::{AStar, PathfindingAlgorithm};

#[test]
fn a_star_finds_a_path_across_an_empty_grid() {
    let grid = empty_grid(3, 3);
    let weights = vec![vec![1; 3]; 3];
    let start = (0, 0);
    let end = (2, 2);

    let result = AStar.find_path(start, end, &grid, &weights);
    let path = result
        .path
        .expect("A* should find a path across an empty grid");

    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 5, "A* should return a shortest path");
}
