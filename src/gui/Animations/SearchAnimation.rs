use super::animations_traits::{AnimationPhase, SearchAnimation};
use crate::CellState;
use eframe::egui;
use std::time::Instant;

impl SearchAnimation {
    pub fn update(&mut self, grid: &mut [Vec<CellState>], ctx: &egui::Context) {
        let now = Instant::now();

        if now < self.next_step {
            ctx.request_repaint_after(self.next_step - now);
            return;
        }

        match self.phase {
            AnimationPhase::Exploring => {
                if let Some(&position) = self.explored_order.get(self.explored_index) {
                    if !matches!(
                        grid[position.0][position.1],
                        CellState::Start | CellState::End
                    ) {
                        grid[position.0][position.1] = CellState::Explored;
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
                if let Some(&position) = self.path.get(self.path_index) {
                    if !matches!(
                        grid[position.0][position.1],
                        CellState::Start | CellState::End
                    ) {
                        grid[position.0][position.1] = CellState::Path;
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

        self.next_step = now + self.step_delay;
        ctx.request_repaint_after(self.step_delay);
    }

    pub fn is_finished(&self) -> bool {
        self.phase == AnimationPhase::Finished
    }
}
