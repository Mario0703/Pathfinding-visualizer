use crate::CellState;
use crate::algorithms::{
    AStar, BFS, BidirectionalBFS, DFS, Dijkstra, GreedyBestFirstSearch, PathfindingAlgorithm,
    Position,
};
use crate::gui::animations::SearchAnimation;
use eframe::egui;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const COST_5_COLOR: egui::Color32 = egui::Color32::from_rgb(190, 220, 120);
const COST_10_COLOR: egui::Color32 = egui::Color32::from_rgb(210, 155, 80);
const COST_15_COLOR: egui::Color32 = egui::Color32::from_rgb(145, 95, 60);
const PLACEMENT_ANIMATION_DURATION: Duration = Duration::from_millis(220);
const PLACEMENT_REPAINT_INTERVAL: Duration = Duration::from_millis(16);

pub fn run() -> eframe::Result {
    let height = 800.0;
    let width = 800.0;
    let title = "Path Finder Visualizer";

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([width, height]),
        ..Default::default()
    };
    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Ok(Box::new(PathFinderVisualizerApp::default()))),
    )
}
#[derive(Clone, Copy, PartialEq)]
enum DrawingTool {
    DrawWall,
    DrawWeight(u32),
    EraseWall,
    DrawStart,
    DrawEnd,
}

#[derive(Clone, Copy)]
enum PlacementAnimationKind {
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
        (now.duration_since(self.started_at).as_secs_f32()
            / PLACEMENT_ANIMATION_DURATION.as_secs_f32())
        .clamp(0.0, 1.0)
    }
}

struct PathFinderVisualizerApp {
    rows: usize,
    columns: usize,
    selected_rows: String,
    selected_columns: String,
    matrix: Vec<Vec<CellState>>,
    weights: Vec<Vec<u32>>,
    algorithm: String,
    drawing_tool: DrawingTool,
    animation: Option<SearchAnimation>,
    placement_animations: HashMap<Position, PlacementAnimation>,
}

impl Default for PathFinderVisualizerApp {
    fn default() -> Self {
        let rows = 10;
        let columns = 10;

        Self {
            rows,
            columns,
            selected_rows: rows.to_string(),
            selected_columns: columns.to_string(),
            matrix: vec![vec![CellState::Unexplored; columns]; rows],
            weights: vec![vec![1; columns]; rows],
            algorithm: String::from("Select Algorithm"),
            drawing_tool: DrawingTool::DrawWall,
            animation: None,
            placement_animations: HashMap::new(),
        }
    }
}

impl eframe::App for PathFinderVisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let animation_finished = if let Some(animation) = &mut self.animation {
            animation.update(&mut self.matrix, ui.ctx());
            animation.is_finished()
        } else {
            false
        };

        if animation_finished {
            self.animation = None;
        }

        let animation_time = Instant::now();
        self.placement_animations
            .retain(|_, animation| animation.progress(animation_time) < 1.0);

        let supports_weights = matches!(self.algorithm.as_str(), "Dijkstra" | "A*");

        if !supports_weights && matches!(self.drawing_tool, DrawingTool::DrawWeight(_)) {
            self.drawing_tool = DrawingTool::DrawWall;
        }

        let min_width = 50.0;
        let cell_width = 30.0;
        let cell_height = 30.0;
        let horizontal_gap = -9.0;
        let vertical_gap = 0.0;
        let path_finding_algorithms = [
            "Dijkstra",
            "A*",
            "Greedy Best-First",
            "BFS",
            "Bidirectional BFS",
            "DFS",
        ];
        const CELL_CORNER_RADIUS: f32 = 0.0;
        const CELL_BORDER_WIDTH: f32 = 1.0;
        const CELL_BORDER_COLOR: egui::Color32 = egui::Color32::BLACK;

        egui::Grid::new("pathfinding_grid")
            .spacing([horizontal_gap, vertical_gap])
            .show(ui, |ui| {
                for row in 0..self.rows {
                    for column in 0..self.columns {
                        let cell_size = egui::vec2(cell_width, cell_height);
                        let (rect, response) =
                            ui.allocate_exact_size(cell_size, egui::Sense::click_and_drag());
                        let (primary_down, pointer_over_cell) = ui.input(|input| {
                            let pointer_over_cell = input
                                .pointer
                                .hover_pos()
                                .is_some_and(|position| rect.contains(position));

                            (input.pointer.primary_down(), pointer_over_cell)
                        });

                        match self.drawing_tool {
                            DrawingTool::DrawWall if primary_down && pointer_over_cell => {
                                if self.matrix[row][column] != CellState::Wall {
                                    self.placement_animations.insert(
                                        (row, column),
                                        PlacementAnimation::new(PlacementAnimationKind::Wall),
                                    );
                                }
                                self.matrix[row][column] = CellState::Wall;
                                self.weights[row][column] = 1;
                            }
                            DrawingTool::DrawWeight(weight)
                                if primary_down && pointer_over_cell =>
                            {
                                let can_draw_weight = !matches!(
                                    self.matrix[row][column],
                                    CellState::Start | CellState::End
                                );

                                if can_draw_weight
                                    && (self.weights[row][column] != weight
                                        || self.matrix[row][column] != CellState::Unexplored)
                                {
                                    self.placement_animations.insert(
                                        (row, column),
                                        PlacementAnimation::new(PlacementAnimationKind::Weight),
                                    );
                                }

                                if can_draw_weight {
                                    self.weights[row][column] = weight;
                                    self.matrix[row][column] = CellState::Unexplored;
                                }
                            }
                            DrawingTool::EraseWall if primary_down && pointer_over_cell => {
                                self.placement_animations.remove(&(row, column));
                                self.matrix[row][column] = CellState::Unexplored;
                                self.weights[row][column] = 1;
                            }
                            DrawingTool::DrawStart
                                if response.clicked() && self.weights[row][column] == 1 =>
                            {
                                for row in 0..self.rows {
                                    for column in 0..self.columns {
                                        if self.matrix[row][column] == CellState::Start {
                                            self.matrix[row][column] = CellState::Unexplored;
                                            self.placement_animations.remove(&(row, column));
                                        }
                                    }
                                }
                                self.matrix[row][column] = CellState::Start;
                                self.placement_animations.insert(
                                    (row, column),
                                    PlacementAnimation::new(PlacementAnimationKind::Start),
                                );
                            }
                            DrawingTool::DrawEnd
                                if response.clicked() && self.weights[row][column] == 1 =>
                            {
                                for row in 0..self.rows {
                                    for column in 0..self.columns {
                                        if self.matrix[row][column] == CellState::End {
                                            self.matrix[row][column] = CellState::Unexplored;
                                            self.placement_animations.remove(&(row, column));
                                        }
                                    }
                                }
                                self.matrix[row][column] = CellState::End;
                                self.placement_animations.insert(
                                    (row, column),
                                    PlacementAnimation::new(PlacementAnimationKind::End),
                                );
                            }
                            _ => {}
                        }

                        let cell_state = self.matrix[row][column];
                        let weight = self.weights[row][column];
                        let cell_color = match cell_state {
                            CellState::Explored => egui::Color32::LIGHT_BLUE,
                            CellState::Unexplored => weight_color(weight),
                            CellState::Wall => egui::Color32::BLACK,
                            CellState::Start => egui::Color32::GREEN,
                            CellState::End => egui::Color32::RED,
                            CellState::Path => egui::Color32::YELLOW,
                        };

                        match self.placement_animations.get(&(row, column)) {
                            Some(animation)
                                if matches!(animation.kind, PlacementAnimationKind::Wall)
                                    && cell_state == CellState::Wall =>
                            {
                                let progress = animation.progress(animation_time);
                                let eased_progress = 1.0 - (1.0 - progress).powi(3);
                                let gray = (255.0 * (1.0 - eased_progress)).round() as u8;
                                ui.painter().rect_filled(
                                    rect,
                                    CELL_CORNER_RADIUS,
                                    egui::Color32::from_gray(gray),
                                );
                            }
                            Some(animation)
                                if matches!(
                                    animation.kind,
                                    PlacementAnimationKind::Weight
                                        | PlacementAnimationKind::Start
                                        | PlacementAnimationKind::End
                                ) && (matches!(
                                    animation.kind,
                                    PlacementAnimationKind::Weight
                                ) && cell_state == CellState::Unexplored
                                    || matches!(
                                        animation.kind,
                                        PlacementAnimationKind::Start | PlacementAnimationKind::End
                                    ) && matches!(
                                        cell_state,
                                        CellState::Start | CellState::End
                                    )) =>
                            {
                                let background_color =
                                    if matches!(animation.kind, PlacementAnimationKind::Weight) {
                                        egui::Color32::WHITE
                                    } else {
                                        weight_color(weight)
                                    };
                                ui.painter().rect_filled(
                                    rect,
                                    CELL_CORNER_RADIUS,
                                    background_color,
                                );

                                let progress = animation.progress(animation_time);
                                let eased_progress = 1.0 - (1.0 - progress).powi(3);
                                let inset =
                                    rect.width().min(rect.height()) * (1.0 - eased_progress) * 0.5;
                                ui.painter().rect_filled(
                                    rect.shrink(inset),
                                    CELL_CORNER_RADIUS,
                                    cell_color,
                                );
                            }
                            _ => {
                                ui.painter()
                                    .rect_filled(rect, CELL_CORNER_RADIUS, cell_color);
                            }
                        }

                        ui.painter().rect_stroke(
                            rect,
                            CELL_CORNER_RADIUS,
                            egui::Stroke::new(CELL_BORDER_WIDTH, CELL_BORDER_COLOR),
                            egui::StrokeKind::Inside,
                        );

                        if weight > 1 && cell_state != CellState::Wall {
                            let text_color = if cell_state == CellState::Unexplored && weight == 15
                            {
                                egui::Color32::WHITE
                            } else {
                                egui::Color32::BLACK
                            };

                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                weight.to_string(),
                                egui::FontId::proportional(12.0),
                                text_color,
                            );
                        }
                    }

                    ui.end_row();
                }
            });

        if !self.placement_animations.is_empty() {
            ui.ctx().request_repaint_after(PLACEMENT_REPAINT_INTERVAL);
        }

        egui::Grid::new("controls_grid")
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label("Rows:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.selected_rows)
                        .desired_width(min_width)
                        .hint_text("Rows"),
                );

                ui.label("Columns:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.selected_columns)
                        .desired_width(min_width)
                        .hint_text("Columns"),
                );

                if ui.button("Create Grid").clicked() {
                    if let (Ok(rows), Ok(columns)) = (
                        self.selected_rows.trim().parse::<usize>(),
                        self.selected_columns.trim().parse::<usize>(),
                    ) {
                        self.rows = rows;
                        self.columns = columns;
                        self.matrix = vec![vec![CellState::Unexplored; columns]; rows];
                        self.weights = vec![vec![1; columns]; rows];
                        self.animation = None;
                        self.placement_animations.clear();
                    }
                }

                if ui.button("Reset Grid").clicked() {
                    self.matrix = vec![vec![CellState::Unexplored; self.columns]; self.rows];
                    self.weights = vec![vec![1; self.columns]; self.rows];
                    self.animation = None;
                    self.placement_animations.clear();
                    self.drawing_tool = DrawingTool::DrawWall;
                }
                ui.end_row();

                ui.label("Algorithm:");
                egui::ComboBox::from_id_salt("algorithm_selector")
                    .selected_text(&self.algorithm)
                    .show_ui(ui, |ui| {
                        for algorithm in path_finding_algorithms {
                            ui.selectable_value(
                                &mut self.algorithm,
                                algorithm.to_string(),
                                algorithm,
                            );
                        }
                    });
                ui.end_row();

                let algorithm_info = match self.algorithm.as_str() {
                    "DFS" => Some(DFS.info()),
                    "BFS" => Some(BFS.info()),
                    "Bidirectional BFS" => Some(BidirectionalBFS.info()),
                    "Greedy Best-First" => Some(GreedyBestFirstSearch.info()),
                    "Dijkstra" => Some(Dijkstra.info()),
                    _ => None,
                };

                if let Some(info) = algorithm_info {
                    ui.label("Algorithm facts:");
                    ui.vertical(|ui| {
                        ui.set_max_width(520.0);
                        ui.strong(info.name);
                        ui.add(egui::Label::new(info.description).wrap());
                        ui.label(format!(
                            "Time: {}    Space: {}",
                            info.time_complexity, info.space_complexity
                        ));
                    });
                    ui.end_row();
                }

                if supports_weights {
                    ui.label("Terrain weights:");
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Cost 5").color(egui::Color32::BLACK),
                            )
                            .fill(COST_5_COLOR)
                            .selected(self.drawing_tool == DrawingTool::DrawWeight(5)),
                        )
                        .on_hover_text("Paint cells with a traversal cost of 5")
                        .clicked()
                    {
                        self.drawing_tool = DrawingTool::DrawWeight(5);
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Cost 10").color(egui::Color32::BLACK),
                            )
                            .fill(COST_10_COLOR)
                            .selected(self.drawing_tool == DrawingTool::DrawWeight(10)),
                        )
                        .on_hover_text("Paint cells with a traversal cost of 10")
                        .clicked()
                    {
                        self.drawing_tool = DrawingTool::DrawWeight(10);
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Cost 15").color(egui::Color32::BLACK),
                            )
                            .fill(COST_15_COLOR)
                            .selected(self.drawing_tool == DrawingTool::DrawWeight(15)),
                        )
                        .on_hover_text("Paint cells with a traversal cost of 15")
                        .clicked()
                    {
                        self.drawing_tool = DrawingTool::DrawWeight(15);
                    }
                    ui.end_row();
                }

                ui.label("Search:");
                if ui.button("Find Path").clicked() {
                    let start = find_cell(&self.matrix, CellState::Start);
                    let end = find_cell(&self.matrix, CellState::End);

                    if let (Some(start), Some(end)) = (start, end) {
                        clear_previous_search(&mut self.matrix);

                        let result = match self.algorithm.as_str() {
                            "DFS" => Some(DFS.find_path(start, end, &self.matrix, &self.weights)),
                            "BFS" => Some(BFS.find_path(start, end, &self.matrix, &self.weights)),
                            "Bidirectional BFS" => Some(BidirectionalBFS.find_path(
                                start,
                                end,
                                &self.matrix,
                                &self.weights,
                            )),
                            "Greedy Best-First" => Some(GreedyBestFirstSearch.find_path(
                                start,
                                end,
                                &self.matrix,
                                &self.weights,
                            )),
                            "Dijkstra" => {
                                Some(Dijkstra.find_path(start, end, &self.matrix, &self.weights))
                            }
                            "A*" => Some(AStar.find_path(start, end, &self.matrix, &self.weights)),
                            _ => None,
                        };

                        if let Some(result) = result {
                            self.animation = Some(SearchAnimation::new(result));
                            ui.ctx().request_repaint();
                        }
                    }
                }
                ui.end_row();

                ui.label("Drawing tool:");
                ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawWall, "Walls");
                ui.selectable_value(&mut self.drawing_tool, DrawingTool::EraseWall, "Eraser");
                ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawStart, "Start");
                ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawEnd, "End");
                ui.end_row();
            });
    }
}

fn weight_color(weight: u32) -> egui::Color32 {
    match weight {
        5 => COST_5_COLOR,
        10 => COST_10_COLOR,
        15 => COST_15_COLOR,
        _ => egui::Color32::WHITE,
    }
}

fn find_cell(grid: &[Vec<CellState>], target: CellState) -> Option<Position> {
    for (row, cells) in grid.iter().enumerate() {
        for (column, &cell) in cells.iter().enumerate() {
            if cell == target {
                return Some((row, column));
            }
        }
    }

    None
}

fn clear_previous_search(grid: &mut [Vec<CellState>]) {
    for row in grid {
        for cell in row {
            if matches!(*cell, CellState::Explored | CellState::Path) {
                *cell = CellState::Unexplored;
            }
        }
    }
}
