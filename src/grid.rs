#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Explored,
    Unexplored,
    Wall,
    Start,
    End,
    Path,
}

pub type Grid = Vec<Vec<CellState>>;
