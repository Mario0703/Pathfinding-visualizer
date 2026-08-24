use crate::common::{assert_path_connects, empty_grid};
use path_finder_visualizer::{
    CellState,
    algorithms::{BFS, PathfindingAlgorithm},
};

#[test]
fn bfs_finds_a_shortest_path_around_a_wall() {
    let mut grid = empty_grid(3, 3);
    grid[0][1] = CellState::Wall;

    let start = (0, 0);
    let end = (0, 2);
    let result = BFS.find_path(start, end, &grid);

    assert_eq!(result.explored_order.first(), Some(&start));
    assert!(result.explored_order.contains(&end));

    let path = result.path.expect("BFS should find a path around the wall");
    assert_path_connects(&path, start, end);
    assert_eq!(path.len(), 5, "BFS should return a shortest path");
    assert!(
        path.iter()
            .all(|&(row, column)| { grid[row][column] != CellState::Wall })
    );
}
