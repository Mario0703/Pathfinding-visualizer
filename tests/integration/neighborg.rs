use path_finder_visualizer::{
    CellState,
    algorithms::{Position},
};

fn get_neighbors(position: Position, grid: &[Vec<CellState>]) -> Vec<Position> {
    let (row, column) = position;
    let candidates = [
        (row.wrapping_sub(1), column),
        (row + 1, column),
        (row, column.wrapping_sub(1)),
        (row, column + 1),
    ];

    candidates
        .into_iter()
        .filter(|&(neighbor_row, neighbor_column)| {
            neighbor_row < grid.len()
                && neighbor_column < grid[neighbor_row].len()
                && grid[neighbor_row][neighbor_column] != CellState::Wall
        })
        .collect()
}


#[test]
fn center_cell_has_four_neighbors_on_empty_grid() {
    let grid = empty_grid(3, 3);

    let neighbors = get_neighbors((1, 1), &grid);

    assert_eq!(neighbors.len(), 4);
}

#[test]
fn corner_cell_has_two_neighbors() {
    let grid = empty_grid(3, 3);

    let neighbors = get_neighbors((0, 0), &grid);

    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&(1, 0)));
    assert!(neighbors.contains(&(0, 1)));
}

#[test]
fn walls_are_not_returned_as_neighbors() {
    let mut grid = empty_grid(3, 3);
    grid[1][0] = CellState::Wall;

    let neighbors = get_neighbors((1, 1), &grid);

    assert!(!neighbors.contains(&(1, 0)));
    assert!(neighbors.contains(&(0, 1)));
    assert!(neighbors.contains(&(2, 1)));
    assert!(neighbors.contains(&(1, 2)));
}

#[test]
fn surrounded_cell_has_no_neighbors() {
    let mut grid = empty_grid(3, 3);

    grid[0][1] = CellState::Wall;
    grid[2][1] = CellState::Wall;
    grid[1][0] = CellState::Wall;
    grid[1][2] = CellState::Wall;

    let neighbors = get_neighbors((1, 1), &grid);

    assert!(neighbors.is_empty());
}
