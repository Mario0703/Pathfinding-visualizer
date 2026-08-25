use super::animations::{AnimationPhase, SearchAnimation};
use crate::CellState;
use eframe::egui;
use std::time::Instant;

impl SearchAnimation {
    pub fn update(&mut self, grid: &mut [Vec<CellState>], ctx: &egui::Context) {
        let current_time = Instant::now();

        if current_time < self.next_step {
            let time_until_next_step = self.next_step - current_time;
            ctx.request_repaint_after(time_until_next_step);
            return;
        }
        // Animate the next cell according to the current search phase.
        match self.phase {
            AnimationPhase::Exploring => {
                let next_explored_position = self.explored_order.get(self.explored_index);

                if let Some(&explored_position) = next_explored_position {
                    let (explored_row, explored_column) = explored_position;
                    let cell_state = grid[explored_row][explored_column];

                    if !matches!(cell_state, CellState::Start | CellState::End) {
                        grid[explored_row][explored_column] = CellState::Explored;
                    }

                    self.explored_index += 1;

                    if self.explored_index == self.explored_order.len() {
                        self.phase = AnimationPhase::Pathfinding;
                    }
                } else {
                    self.phase = AnimationPhase::Pathfinding;
                }
            }

            AnimationPhase::Pathfinding => {
                let next_path_position = self.path.get(self.path_index);

                if let Some(&path_position) = next_path_position {
                    let (path_row, path_column) = path_position;
                    let cell_state = grid[path_row][path_column];

                    if !matches!(cell_state, CellState::Start | CellState::End) {
                        grid[path_row][path_column] = CellState::Path;
                    }

                    self.path_index += 1;

                    if self.path_index == self.path.len() {
                        self.phase = AnimationPhase::Finished;
                    }
                } else {
                    self.phase = AnimationPhase::Finished;
                }
            }

            AnimationPhase::Finished => return,
        }

        self.next_step = current_time + self.step_delay;
        ctx.request_repaint_after(self.step_delay);
    }

    pub fn is_finished(&self) -> bool {
        self.phase == AnimationPhase::Finished
    }
}
