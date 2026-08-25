use crate::CellState;
pub type Position = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlgorithmInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub time_complexity: &'static str,
    pub space_complexity: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub explored_order: Vec<Position>,
    pub path: Option<Vec<Position>>,
}

pub trait PathfindingAlgorithm {
    fn info(&self) -> AlgorithmInfo;

    fn find_path(
        &self,
        start: Position,
        end: Position,
        grid: &[Vec<CellState>],
        weights: &[Vec<u32>],
    ) -> SearchResult;
}

#[cfg(test)]
#[path = "../../tests/unit/pathfinding_traits.rs"]
mod tests;
