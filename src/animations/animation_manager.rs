use crate::animations::SearchAnimation;
use crate::{CellState, Position, SearchResult};
use eframe::egui;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const PLACEMENT_ANIMATION_DURATION: Duration = Duration::from_millis(220);
const PLACEMENT_REPAINT_INTERVAL: Duration = Duration::from_millis(16);

#[derive(Clone, Copy)]
pub enum PlacementAnimationKind {
    Wall,
    Weight,
    Start,
    End,
}

struct PlacementAnimation {
    kind: PlacementAnimationKind,
    started_at: Instant,
}

impl PlacementAnimation {
    fn new(kind: PlacementAnimationKind) -> Self {
        Self {
            kind,
            started_at: Instant::now(),
        }
    }

    fn progress(&self, now: Instant) -> f32 {
        (now.saturating_duration_since(self.started_at).as_secs_f32()
            / PLACEMENT_ANIMATION_DURATION.as_secs_f32())
        .clamp(0.0, 1.0)
    }
}

#[derive(Default)]
pub struct AnimationManager {
    search: Option<SearchAnimation>,
    placements: HashMap<Position, PlacementAnimation>,
}

impl AnimationManager {
    pub fn update(&mut self, grid: &mut [Vec<CellState>], ctx: &egui::Context) {
        if let Some(animation) = &mut self.search {
            animation.update(grid, ctx);

            if animation.is_finished() {
                self.search = None;
            }
        }

        let now = Instant::now();
        self.placements
            .retain(|_, animation| animation.progress(now) < 1.0);

        if !self.placements.is_empty() {
            ctx.request_repaint_after(PLACEMENT_REPAINT_INTERVAL);
        }
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
        let eased_progress = 1.0 - (1.0 - progress).powi(3);

        match (animation.kind, cell_state) {
            (PlacementAnimationKind::Wall, CellState::Wall) => {
                let gray = (255.0 * (1.0 - eased_progress)).round() as u8;
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

                let inset = rect.width().min(rect.height()) * (1.0 - eased_progress) * 0.5;
                painter.rect_filled(rect.shrink(inset), corner_radius, cell_color);
            }
            _ => {
                painter.rect_filled(rect, corner_radius, cell_color);
            }
        }
    }
}
