use crate::CellState;
use crate::algorithms::{DFS, PathfindingAlgorithm, Position};
use eframe::egui;
use std::time::{Duration, Instant};

const PATH_ANIMATION_DELAY: Duration = Duration::from_millis(75);

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
    EraseWall,
    DrawStart,
    DrawEnd,
}

struct PathFinderVisualizerApp {
    rows: usize,
    columns: usize,
    selected_rows: String,
    selected_columns: String,
    matrix: Vec<Vec<CellState>>,
    algorithm: String,
    drawing_tool: DrawingTool,
    explored_animation: Vec<Position>,
    explored_animation_index: usize,
    path_animation: Vec<Position>,
    path_animation_index: usize,
    next_animation_step: Option<Instant>,
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
            algorithm: String::from("Select Algorithm"),
            drawing_tool: DrawingTool::DrawWall,
            explored_animation: Vec::new(),
            explored_animation_index: 0,
            path_animation: Vec::new(),
            path_animation_index: 0,
            next_animation_step: None,
        }
    }
}

impl eframe::App for PathFinderVisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        show_next_dfs_animation_cell(
            &self.explored_animation,
            &mut self.explored_animation_index,
            &self.path_animation,
            &mut self.path_animation_index,
            &mut self.next_animation_step,
            &mut self.matrix,
            ui.ctx(),
        );

        let min_width = 50.0;
        let cell_width = 30.0;
        let cell_height = 30.0;
        let horizontal_gap = -9.0;
        let vertical_gap = 0.0;
        let path_finding_algorithms = ["Dijkstra", "A*", "BFS", "DFS"];
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
                                self.matrix[row][column] = CellState::Wall;
                            }
                            DrawingTool::EraseWall if primary_down && pointer_over_cell => {
                                self.matrix[row][column] = CellState::Unexplored;
                            }
                            DrawingTool::DrawStart if response.clicked() => {
                                for row in 0..self.rows {
                                    for column in 0..self.columns {
                                        if self.matrix[row][column] == CellState::Start {
                                            self.matrix[row][column] = CellState::Unexplored;
                                        }
                                    }
                                }
                                self.matrix[row][column] = CellState::Start;
                            }
                            DrawingTool::DrawEnd if response.clicked() => {
                                for row in 0..self.rows {
                                    for column in 0..self.columns {
                                        if self.matrix[row][column] == CellState::End {
                                            self.matrix[row][column] = CellState::Unexplored;
                                        }
                                    }
                                }
                                self.matrix[row][column] = CellState::End;
                            }
                            _ => {}
                        }

                        let cell_color = match self.matrix[row][column] {
                            CellState::Explored => egui::Color32::LIGHT_BLUE,
                            CellState::Unexplored => egui::Color32::WHITE,
                            CellState::Wall => egui::Color32::BLACK,
                            CellState::Start => egui::Color32::GREEN,
                            CellState::End => egui::Color32::RED,
                            CellState::Path => egui::Color32::YELLOW,
                        };

                        ui.painter()
                            .rect_filled(rect, CELL_CORNER_RADIUS, cell_color);

                        ui.painter().rect_stroke(
                            rect,
                            CELL_CORNER_RADIUS,
                            egui::Stroke::new(CELL_BORDER_WIDTH, CELL_BORDER_COLOR),
                            egui::StrokeKind::Inside,
                        );
                    }

                    ui.end_row();
                }
            });
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.selected_rows)
                    .desired_width(min_width)
                    .hint_text("Rows"),
            );

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
                    self.explored_animation.clear();
                    self.explored_animation_index = 0;
                    self.path_animation.clear();
                    self.path_animation_index = 0;
                    self.next_animation_step = None;
                }
            }
            egui::ComboBox::from_label("Select Algorithm")
                .selected_text(&self.algorithm)
                .show_ui(ui, |ui| {
                    for algorithm in path_finding_algorithms {
                        ui.selectable_value(&mut self.algorithm, algorithm.to_string(), algorithm);
                    }
                });
            if ui.button("Find Path").clicked() {
                let start = find_cell(&self.matrix, CellState::Start);
                let end = find_cell(&self.matrix, CellState::End);

                if let (Some(start), Some(end)) = (start, end) {
                    if self.algorithm == "DFS" {
                        clear_previous_search(&mut self.matrix);
                        let result = DFS.find_path(start, end, &self.matrix);

                        self.explored_animation = result.explored_order;
                        self.explored_animation_index = 0;
                        self.path_animation = result.path.unwrap_or_default();
                        self.path_animation_index = 0;
                        self.next_animation_step = Some(Instant::now());
                        ui.ctx().request_repaint();
                    }
                }
            }

            ui.label("Drawing tool:");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawWall, "Walls");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::EraseWall, "Eraser");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawStart, "Start");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawEnd, "End");
        });
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

fn show_next_dfs_animation_cell(
    explored: &[Position],
    explored_index: &mut usize,
    path: &[Position],
    path_index: &mut usize,
    next_step: &mut Option<Instant>,
    grid: &mut [Vec<CellState>],
    ctx: &egui::Context,
) {
    if *explored_index >= explored.len() && *path_index >= path.len() {
        *next_step = None;
        return;
    }

    let now = Instant::now();
    let scheduled_step = next_step.get_or_insert(now);

    if now < *scheduled_step {
        ctx.request_repaint_after(*scheduled_step - now);
        return;
    }

    if *explored_index < explored.len() {
        let (row, column) = explored[*explored_index];

        if !matches!(grid[row][column], CellState::Start | CellState::End) {
            grid[row][column] = CellState::Explored;
        }

        *explored_index += 1;
    } else {
        let (row, column) = path[*path_index];

        if !matches!(grid[row][column], CellState::Start | CellState::End) {
            grid[row][column] = CellState::Path;
        }

        *path_index += 1;
    }

    *next_step = Some(now + PATH_ANIMATION_DELAY);
    ctx.request_repaint_after(PATH_ANIMATION_DELAY);
}
