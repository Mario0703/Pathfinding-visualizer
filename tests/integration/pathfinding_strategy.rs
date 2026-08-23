use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::{
    AlgorithmInfo, CellState, PathfindingAlgorithm, Position, SearchResult,
};

struct TestAlgorithm;

impl PathfindingAlgorithm for TestAlgorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Test",
            description: "Integration-test strategy",
            time_complexity: "O(1)",
            space_complexity: "O(1)",
        }
    }

    fn find_path(&self, start: Position, end: Position, _grid: &[Vec<CellState>]) -> SearchResult {
        SearchResult {
            explored_order: vec![start, (0, 1), end],
            path: Some(vec![start, (0, 1), end]),
        }
    }
}

#[test]
fn strategy_is_usable_through_the_public_api() {
    let algorithm: Box<dyn PathfindingAlgorithm> = Box::new(TestAlgorithm);
    let grid = empty_grid(1, 3);

    let result = algorithm.find_path((0, 0), (0, 2), &grid);
    let path = result.path.expect("the test strategy should find a path");

    assert_eq!(algorithm.info().name, "Test");
    assert_eq!(result.explored_order, vec![(0, 0), (0, 1), (0, 2)]);
    assert_path_connects(&path, (0, 0), (0, 2));
}

#[test]
fn basic_dfs_implementation() {
    let algorithm: Box<dyn PathfindingAlgorithm> =
        Box::new(path_finder_visualizer::algorithms::DFS);
    let grid = empty_grid(3, 3);
    let start = (0, 0);
    let end = (2, 2);

    let result = algorithm.find_path(start, end, &grid);

    assert_eq!(result.explored_order.first(), Some(&start));
    assert!(result.explored_order.contains(&end));

    let path = result
        .path
        .expect("DFS should find a path across an empty grid");
    assert_path_connects(&path, start, end);
}
