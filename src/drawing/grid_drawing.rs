use super::{drawing_manager::DrawingManager, drawing_tool::DrawingTool};
use crate::animations::PlacementAnimationKind;
use crate::{animations::AnimationManager, CellState, Position};
use eframe::egui;

const CELL_WIDTH: f32 = 30.0;
const CELL_HEIGHT: f32 = 30.0;
const HORIZONTAL_GAP: f32 = -9.0;
const VERTICAL_GAP: f32 = 0.0;
const CELL_CORNER_RADIUS: f32 = 0.0;
const CELL_BORDER_WIDTH: f32 = 1.0;
const CELL_BORDER_COLOR: egui::Color32 = egui::Color32::BLACK;

pub const COST_5_COLOR: egui::Color32 = egui::Color32::from_rgb(190, 220, 120);
pub const COST_10_COLOR: egui::Color32 = egui::Color32::from_rgb(210, 155, 80);
pub const COST_15_COLOR: egui::Color32 = egui::Color32::from_rgb(145, 95, 60);
pub const LOW_TERRAIN_WEIGHT: u32 = 5;
pub const MEDIUM_TERRAIN_WEIGHT: u32 = 10;
pub const HIGH_TERRAIN_WEIGHT: u32 = 15;

impl DrawingManager {
    pub fn show_grid(
        &self,
        ui: &mut egui::Ui,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        egui::Grid::new("pathfinding_grid")
            .spacing([HORIZONTAL_GAP, VERTICAL_GAP])
            .show(ui, |ui| {
                for row in 0..grid.len() {
                    for column in 0..grid[row].len() {
                        self.show_cell(ui, (row, column), grid, weights, animations);
                    }

                    ui.end_row();
                }
            });
    }

    fn show_cell(
        &self,
        ui: &mut egui::Ui,
        position: Position,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        let cell_size = egui::vec2(CELL_WIDTH, CELL_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(cell_size, egui::Sense::click_and_drag());
        let (primary_down, pointer_over_cell) = ui.input(|input| {
            let pointer_over_cell = input
                .pointer
                .hover_pos()
                .is_some_and(|pointer_position| rect.contains(pointer_position));

            (input.pointer.primary_down(), pointer_over_cell)
        });

        self.apply_tool(
            position,
            primary_down && pointer_over_cell,
            response.clicked(),
            grid,
            weights,
            animations,
        );
        self.paint_cell(ui, rect, position, grid, weights, animations);
    }

    fn apply_tool(
        &self,
        position: Position,
        is_drawing: bool,
        was_clicked: bool,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        match self.tool {
            DrawingTool::DrawWall if is_drawing => {
                Self::draw_wall(position, grid, weights, animations)
            }
            DrawingTool::DrawWeight(weight) if is_drawing => {
                Self::draw_weight(position, weight, grid, weights, animations)
            }
            DrawingTool::EraseWall if is_drawing => {
                Self::erase_cell(position, grid, weights, animations)
            }
            DrawingTool::DrawStart if was_clicked => Self::place_marker(
                position,
                CellState::Start,
                PlacementAnimationKind::Start,
                grid,
                weights,
                animations,
            ),
            DrawingTool::DrawEnd if was_clicked => Self::place_marker(
                position,
                CellState::End,
                PlacementAnimationKind::End,
                grid,
                weights,
                animations,
            ),
            _ => {}
        }
    }

    fn draw_wall(
        position: Position,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        let (row, column) = position;

        if grid[row][column] != CellState::Wall {
            animations.start_placement(position, PlacementAnimationKind::Wall);
        }

        grid[row][column] = CellState::Wall;
        weights[row][column] = 1;
    }

    fn draw_weight(
        position: Position,
        weight: u32,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        let (row, column) = position;

        if matches!(grid[row][column], CellState::Start | CellState::End) {
            return;
        }

        if weights[row][column] != weight || grid[row][column] != CellState::Unexplored {
            animations.start_placement(position, PlacementAnimationKind::Weight);
        }

        weights[row][column] = weight;
        grid[row][column] = CellState::Unexplored;
    }

    fn erase_cell(
        position: Position,
        grid: &mut [Vec<CellState>],
        weights: &mut [Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        let (row, column) = position;
        animations.cancel_placement(position);
        grid[row][column] = CellState::Unexplored;
        weights[row][column] = 1;
    }

    fn place_marker(
        position: Position,
        marker: CellState,
        animation_kind: PlacementAnimationKind,
        grid: &mut [Vec<CellState>],
        weights: &[Vec<u32>],
        animations: &mut AnimationManager,
    ) {
        let (row, column) = position;

        if weights[row][column] != 1 {
            return;
        }

        for (existing_row, cells) in grid.iter_mut().enumerate() {
            for (existing_column, cell) in cells.iter_mut().enumerate() {
                if *cell == marker {
                    *cell = CellState::Unexplored;
                    animations.cancel_placement((existing_row, existing_column));
                }
            }
        }

        grid[row][column] = marker;
        animations.start_placement(position, animation_kind);
    }

    fn paint_cell(
        &self,
        ui: &egui::Ui,
        rect: egui::Rect,
        position: Position,
        grid: &[Vec<CellState>],
        weights: &[Vec<u32>],
        animations: &AnimationManager,
    ) {
        let (row, column) = position;
        let cell_state = grid[row][column];
        let weight = weights[row][column];
        let cell_color = cell_color(cell_state, weight);

        animations.paint_cell(
            ui.painter(),
            rect,
            CELL_CORNER_RADIUS,
            position,
            cell_state,
            cell_color,
            weight_color(weight),
        );

        ui.painter().rect_stroke(
            rect,
            CELL_CORNER_RADIUS,
            egui::Stroke::new(CELL_BORDER_WIDTH, CELL_BORDER_COLOR),
            egui::StrokeKind::Inside,
        );

        if weight > 1 && cell_state != CellState::Wall {
            paint_weight_label(ui.painter(), rect, weight);
        }
    }
}

fn cell_color(cell_state: CellState, weight: u32) -> egui::Color32 {
    match cell_state {
        CellState::Explored => egui::Color32::LIGHT_BLUE,
        CellState::Unexplored => weight_color(weight),
        CellState::Wall => egui::Color32::BLACK,
        CellState::Start => egui::Color32::GREEN,
        CellState::End => egui::Color32::RED,
        CellState::Path => egui::Color32::YELLOW,
    }
}

fn weight_color(weight: u32) -> egui::Color32 {
    match weight {
        LOW_TERRAIN_WEIGHT => COST_5_COLOR,
        MEDIUM_TERRAIN_WEIGHT => COST_10_COLOR,
        HIGH_TERRAIN_WEIGHT => COST_15_COLOR,
        _ => egui::Color32::WHITE,
    }
}

fn paint_weight_label(painter: &egui::Painter, rect: egui::Rect, weight: u32) {
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        weight.to_string(),
        egui::FontId::proportional(12.0),
        egui::Color32::BLACK,
    );
}
