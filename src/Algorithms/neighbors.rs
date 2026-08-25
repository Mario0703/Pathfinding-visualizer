use crate::CellState;

use crate::Position;

pub fn get_neighbors(position: Position, graph: &[Vec<CellState>]) -> Vec<Position> {
    let left = (Some(position.0), position.1.checked_sub(1));
    let right = (Some(position.0), position.1.checked_add(1));
    let up = (position.0.checked_sub(1), Some(position.1));
    let down = (position.0.checked_add(1), Some(position.1));

    let possible_neighbors = vec![left, right, up, down];
    let mut neighbors = Vec::new();

    for neighbor in possible_neighbors {
        if let (Some(row), Some(column)) = neighbor {
            let row_is_in_bounds = row < graph.len();
            let column_is_in_bounds = row_is_in_bounds && column < graph[row].len();

            if column_is_in_bounds && graph[row][column] != CellState::Wall {
                neighbors.push((row, column));
            }
        }
    }

    neighbors
}
