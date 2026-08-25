use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::{
    CellState,
    algorithms::{GreedyBestFirstSearch, PathfindingAlgorithm},
};

#[test]
fn greedy_best_first_finds_a_path_around_walls() {
    let mut grid = empty_grid(4, 5);
    let weights = vec![vec![1; 5]; 4];
    grid[1][1] = CellState::Wall;
    grid[1][2] = CellState::Wall;
    grid[1][3] = CellState::Wall;

    let start = (1, 0);
    let end = (1, 4);
    let result = GreedyBestFirstSearch.find_path(start, end, &grid, &weights);

    let path = result
        .path
        .expect("greedy best-first search should find a path around the wall");
    assert_path_connects(&path, start, end);
    assert!(
        path.iter()
            .all(|&(row, column)| grid[row][column] != CellState::Wall)
    );
}

#[test]
fn greedy_best_first_uses_only_manhattan_distance_for_priority() {
    let grid = empty_grid(3, 5);
    let mut weights = vec![vec![1; 5]; 3];
    weights[1][1] = 100;
    weights[1][2] = 100;
    weights[1][3] = 100;

    let result = GreedyBestFirstSearch.find_path((1, 0), (1, 4), &grid, &weights);

    assert_eq!(
        result.path,
        Some(vec![(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)]),
        "weights must not influence greedy best-first priority",
    );
}

#[test]
fn greedy_best_first_returns_none_when_no_path_exists() {
    let mut grid = empty_grid(3, 3);
    let weights = vec![vec![1; 3]; 3];
    grid[1] = vec![CellState::Wall; 3];

    let result = GreedyBestFirstSearch.find_path((0, 0), (2, 2), &grid, &weights);

    assert_eq!(result.path, None);
}

#[test]
fn greedy_best_first_handles_identical_start_and_end() {
    let grid = empty_grid(1, 1);
    let weights = vec![vec![1]];

    let result = GreedyBestFirstSearch.find_path((0, 0), (0, 0), &grid, &weights);

    assert_eq!(result.explored_order, vec![(0, 0)]);
    assert_eq!(result.path, Some(vec![(0, 0)]));
}
