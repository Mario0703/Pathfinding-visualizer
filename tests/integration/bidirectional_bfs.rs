use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::{
    CellState,
    algorithms::{BFS, BidirectionalBFS, PathfindingAlgorithm},
};

#[test]
fn bidirectional_bfs_finds_a_shortest_path_around_a_wall() {
    let mut grid = empty_grid(3, 3);
    let weights = vec![vec![1; 3]; 3];
    grid[0][1] = CellState::Wall;

    let start = (0, 0);
    let end = (0, 2);
    let result = BidirectionalBFS.find_path(start, end, &grid, &weights);

    assert_eq!(result.explored_order.first(), Some(&start));
    assert!(result.explored_order.contains(&end));

    let path = result
        .path
        .expect("bidirectional BFS should find a path around the wall");
    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 5, "the returned path should be a shortest path");
    assert!(
        path.iter()
            .all(|&(row, column)| grid[row][column] != CellState::Wall)
    );
}

#[test]
fn bidirectional_bfs_joins_both_parent_chains() {
    let grid = empty_grid(1, 7);
    let weights = vec![vec![1; 7]];

    let result = BidirectionalBFS.find_path((0, 0), (0, 6), &grid, &weights);

    assert_eq!(
        result.path,
        Some(vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),])
    );
}

#[test]
fn bidirectional_bfs_returns_none_when_no_path_exists() {
    let mut grid = empty_grid(3, 3);
    let weights = vec![vec![1; 3]; 3];
    grid[1] = vec![CellState::Wall; 3];

    let result = BidirectionalBFS.find_path((0, 0), (2, 2), &grid, &weights);

    assert_eq!(result.path, None);
}

#[test]
fn bidirectional_bfs_handles_identical_start_and_end() {
    let grid = empty_grid(1, 1);
    let weights = vec![vec![1]];

    let result = BidirectionalBFS.find_path((0, 0), (0, 0), &grid, &weights);

    assert_eq!(result.explored_order, vec![(0, 0)]);
    assert_eq!(result.path, Some(vec![(0, 0)]));
}

#[test]
fn bidirectional_bfs_matches_bfs_shortest_lengths_across_wall_layouts() {
    let start = (0, 0);
    let end = (2, 3);
    let weights = vec![vec![1; 4]; 3];
    let variable_cells: Vec<_> = (0..3)
        .flat_map(|row| (0..4).map(move |column| (row, column)))
        .filter(|&position| position != start && position != end)
        .collect();

    for wall_mask in 0..(1_usize << variable_cells.len()) {
        let mut grid = empty_grid(3, 4);

        for (bit, &(row, column)) in variable_cells.iter().enumerate() {
            if wall_mask & (1 << bit) != 0 {
                grid[row][column] = CellState::Wall;
            }
        }

        let bfs_path = BFS.find_path(start, end, &grid, &weights).path;
        let bidirectional_path = BidirectionalBFS.find_path(start, end, &grid, &weights).path;

        assert_eq!(
            bidirectional_path.as_ref().map(Vec::len),
            bfs_path.as_ref().map(Vec::len),
            "path length differed for wall mask {wall_mask:#014b}",
        );

        if let Some(path) = bidirectional_path {
            assert_path_connects(&path, start, end);
        }
    }
}
