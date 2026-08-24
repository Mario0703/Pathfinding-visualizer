use super::*;

struct TestAlgorithm;

impl PathfindingAlgorithm for TestAlgorithm {
    fn info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: "Test",
            description: "A test pathfinding strategy",
            time_complexity: "O(1)",
            space_complexity: "O(1)",
        }
    }

    fn find_path(
        &self,
        start: Position,
        end: Position,
        _grid: &[Vec<CellState>],
        _weights: &[Vec<u32>],
    ) -> SearchResult {
        SearchResult {
            explored_order: vec![start, end],
            path: Some(vec![start, end]),
        }
    }
}

#[test]
fn algorithm_can_be_used_as_a_strategy() {
    let algorithm: Box<dyn PathfindingAlgorithm> = Box::new(TestAlgorithm);
    let grid = vec![vec![CellState::Unexplored; 2]];
    let weights = vec![vec![1; 2]];

    let result = algorithm.find_path((0, 0), (0, 1), &grid, &weights);

    assert_eq!(algorithm.info().name, "Test");
    assert_eq!(result.explored_order, vec![(0, 0), (0, 1)]);
    assert_eq!(result.path, Some(vec![(0, 0), (0, 1)]));
}
