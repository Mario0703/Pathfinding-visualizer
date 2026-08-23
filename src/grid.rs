#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Explored,
    Unexplored,
    Wall,
    Start,
    End,
}

pub type Grid = Vec<Vec<CellState>>;
