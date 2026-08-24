use crate::{Position, SearchResult};
use std::time::{Duration, Instant};

pub struct SearchAnimation {
    pub(crate) path: Vec<Position>,
    pub(crate) explored_order: Vec<Position>,
    pub(crate) explored_index: usize,
    pub(crate) path_index: usize,
    pub(crate) phase: AnimationPhase,
    pub(crate) next_step: Instant,
    pub(crate) step_delay: Duration,
}

#[derive(PartialEq, Eq)]
pub(crate) enum AnimationPhase {
    Exploring,
    Pathfinding,
    Finished,
}

impl SearchAnimation {
    pub fn new(result: SearchResult) -> Self {
        Self {
            explored_order: result.explored_order,
            path: result.path.unwrap_or_default(),
            explored_index: 0,
            path_index: 0,
            phase: AnimationPhase::Exploring,
            next_step: Instant::now(),
            step_delay: Duration::from_millis(75),
        }
    }
}
