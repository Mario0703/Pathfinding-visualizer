use path_finder_visualizer::{CellState, Grid, Position};

pub fn empty_grid(rows: usize, columns: usize) -> Grid {
    vec![vec![CellState::Unexplored; columns]; rows]
}

pub fn assert_path_connects(path: &[Position], expected_start: Position, expected_end: Position) {
    assert_eq!(path.first(), Some(&expected_start));
    assert_eq!(path.last(), Some(&expected_end));

    for positions in path.windows(2) {
        let (first_row, first_column) = positions[0];
        let (second_row, second_column) = positions[1];
        let distance = first_row.abs_diff(second_row) + first_column.abs_diff(second_column);

        assert_eq!(distance, 1, "path contains non-adjacent cells");
    }
}
