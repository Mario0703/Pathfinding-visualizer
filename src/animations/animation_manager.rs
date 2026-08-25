use crate::animations::SearchAnimation;
use crate::{CellState, Position, SearchResult};
use eframe::egui;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const PLACEMENT_ANIMATION_DURATION: Duration = Duration::from_millis(220);
const PLACEMENT_REPAINT_INTERVAL: Duration = Duration::from_millis(16);
const COMPLETE_PROGRESS: f32 = 1.0;
const CUBIC_EASING_EXPONENT: i32 = 3;
const INITIAL_INSET_RATIO: f32 = 0.5;

#[derive(Clone, Copy)]
pub enum PlacementAnimationKind {
    Wall,
    Weight,
    Start,
    End,
}

struct PlacementAnimation {
    kind: PlacementAnimationKind,
    started_at: Instant, // Used to calculate elapsed animation time.
}

impl PlacementAnimation {
    fn new(kind: PlacementAnimationKind) -> Self {
        Self {
            kind,
            started_at: Instant::now(),
        }
    }

    fn progress(&self, now: Instant) -> f32 {
        let min = 0.0;
        let max = 1.0;
        (now.saturating_duration_since(self.started_at).as_secs_f32()
            / PLACEMENT_ANIMATION_DURATION.as_secs_f32())
        .clamp(min, max)
    }
}

#[derive(Default)]
pub struct AnimationManager {
    search: Option<SearchAnimation>,
    placements: HashMap<Position, PlacementAnimation>, // Active placement animations indexed by grid position.
}

impl AnimationManager {
    pub fn update(&mut self, grid: &mut [Vec<CellState>], ctx: &egui::Context) {
        let search_animation = self.search.as_mut();

        let search_animation_is_finished = match search_animation {
            Some(animation) => {
                animation.update(grid, ctx);
                animation.is_finished()
            }
            None => false,
        };

        if search_animation_is_finished {
            self.search = None;
        } // Remove the search animation after all exploration and path steps are complete.

        let now = Instant::now();

        self.placements
            .retain(|_, animation| animation.progress(now) < COMPLETE_PROGRESS);
        // Keep unfinished placement animations and remove those that have completed.

        if !self.placements.is_empty() {
            ctx.request_repaint_after(PLACEMENT_REPAINT_INTERVAL);
        } // Continue repainting while placement animations are still active.
    }

    pub fn start_search(&mut self, result: SearchResult, ctx: &egui::Context) {
        self.search = Some(SearchAnimation::new(result));
        ctx.request_repaint();
    }

    pub fn start_placement(&mut self, position: Position, kind: PlacementAnimationKind) {
        self.placements
            .insert(position, PlacementAnimation::new(kind));
    }

    pub fn cancel_placement(&mut self, position: Position) {
        self.placements.remove(&position);
    }

    pub fn clear(&mut self) {
        self.search = None;
        self.placements.clear();
    }

    pub fn paint_cell(
        &self,
        painter: &egui::Painter,
        rect: egui::Rect,
        corner_radius: f32,
        position: Position,
        cell_state: CellState,
        cell_color: egui::Color32,
        placement_background: egui::Color32,
    ) {
        let Some(animation) = self.placements.get(&position) else {
            painter.rect_filled(rect, corner_radius, cell_color);
            return;
        };

        let progress = animation.progress(Instant::now());
        let remaining_progress = COMPLETE_PROGRESS - progress;
        let eased_progress = COMPLETE_PROGRESS - remaining_progress.powi(CUBIC_EASING_EXPONENT);
        let remaining_eased_progress = COMPLETE_PROGRESS - eased_progress;

        match (animation.kind, cell_state) {
            (PlacementAnimationKind::Wall, CellState::Wall) => {
                let maximum_gray_value = f32::from(u8::MAX);
                let gray = (maximum_gray_value * remaining_eased_progress).round() as u8;
                painter.rect_filled(rect, corner_radius, egui::Color32::from_gray(gray));
            }
            (PlacementAnimationKind::Weight, CellState::Unexplored)
            | (PlacementAnimationKind::Start, CellState::Start)
            | (PlacementAnimationKind::End, CellState::End) => {
                let background_color = match animation.kind {
                    PlacementAnimationKind::Weight => egui::Color32::WHITE,
                    _ => placement_background,
                };
                painter.rect_filled(rect, corner_radius, background_color);

                let shortest_cell_side = rect.width().min(rect.height());
                let inset = shortest_cell_side * remaining_eased_progress * INITIAL_INSET_RATIO;
                painter.rect_filled(rect.shrink(inset), corner_radius, cell_color);
            }
            _ => {
                painter.rect_filled(rect, corner_radius, cell_color);
            }
        }
    }
}
