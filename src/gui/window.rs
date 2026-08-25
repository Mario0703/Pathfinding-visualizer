use crate::CellState;
use crate::algorithms::{
    AStar, BFS, BidirectionalBFS, DFS, Dijkstra, GreedyBestFirstSearch, PathfindingAlgorithm,
    Position,
};
use crate::{animations::AnimationManager, drawing::DrawingManager};
use eframe::egui;

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
struct PathFinderVisualizerApp {
    rows: usize,
    columns: usize,
    selected_rows: String,
    selected_columns: String,
    matrix: Vec<Vec<CellState>>,
    weights: Vec<Vec<u32>>,
    algorithm: String,
    drawing: DrawingManager,
    animations: AnimationManager,
}

impl Default for PathFinderVisualizerApp {
    fn default() -> Self {
        let starting_row = 10;
        let starting_column = 10;

        Self {
            rows: starting_row,
            columns: starting_column,
            selected_rows: starting_row.to_string(),
            selected_columns: starting_column.to_string(),
            matrix: vec![vec![CellState::Unexplored; starting_column]; starting_row],
            weights: vec![vec![1; starting_column]; starting_row],
            algorithm: String::from("Select Algorithm"),
            drawing: DrawingManager::default(),
            animations: AnimationManager::default(),
        }
    }
}

impl eframe::App for PathFinderVisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.animations.update(&mut self.matrix, ui.ctx());

        let selected_algorithm = self.algorithm.as_str();

        let supports_weights = if selected_algorithm == "Dijkstra" || selected_algorithm == "A*" {
            true
        } else {
            false
        };

        self.drawing.set_weights_enabled(supports_weights);
        let path_finding_algorithms = [
            "Dijkstra",
            "A*",
            "Greedy Best-First",
            "BFS",
            "Bidirectional BFS",
            "DFS",
        ];

        self.drawing.show_grid(
            ui,
            &mut self.matrix,
            &mut self.weights,
            &mut self.animations,
        );

        egui::Grid::new("controls_grid")
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label("Rows:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.selected_rows)
                        .desired_width(50.0)
                        .hint_text("Rows"),
                );

                ui.label("Columns:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.selected_columns)
                        .desired_width(50.0)
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
                        self.animations.clear();
                    }
                }

                if ui.button("Reset Grid").clicked() {
                    self.matrix = vec![vec![CellState::Unexplored; self.columns]; self.rows];
                    self.weights = vec![vec![1; self.columns]; self.rows];
                    self.animations.clear();
                    self.drawing.reset();
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

                self.drawing.show_weight_controls(ui, supports_weights);

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

                        let has_search_result = result.is_some();

                        if has_search_result {
                            let search_result =
                                result.expect("A search result should be available");

                            self.animations.start_search(search_result, ui.ctx());
                        }
                    }
                }
                ui.end_row();

                self.drawing.show_tool_controls(ui);
            });
    }
}

fn find_cell(grid: &[Vec<CellState>], target_state: CellState) -> Option<Position> {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, &cell_state) in row.iter().enumerate() {
            if cell_state == target_state {
                return Some((row_index, column_index));
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
