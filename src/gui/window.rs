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
#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Explored,
    Unexplored,
}

#[derive(Clone, Copy, PartialEq)]
enum DrawingTool {
    DrawWall,
    EraseWall,
}

struct PathFinderVisualizerApp {
    rows: usize,
    columns: usize,
    selected_rows: String,
    selected_columns: String,
    matrix: Vec<Vec<CellState>>,
    _status: String,
    Algorithm: String,
    drawing_tool: DrawingTool,
    is_drawing: bool,
}

impl Default for PathFinderVisualizerApp {
    fn default() -> Self {
        Self {
            rows: 10,
            columns: 10,
            selected_rows: "10".to_owned(),
            selected_columns: "10".to_owned(),
            matrix: Vec::new(),
            _status: String::from("Lets get started!"),
            Algorithm: String::from("Select Algorithm"),
            drawing_tool: DrawingTool::DrawWall,
            is_drawing: false,
        }
    }
}

impl eframe::App for PathFinderVisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let min_width = 50.0;
        let min_height = 50.0;
        let cell_width = 30.0;
        let cell_height = 30.0;
        let horizontal_gap = -9.0;
        let vertical_gap = 0.0;
        let path_finding_algorithms = ["Dijkstra", "A*", "BFS", "DFS"];
        const CELL_CORNER_RADIUS: f32 = 0.0;
        const CELL_BORDER_WIDTH: f32 = 1.0;
        const CELL_BACKGROUND_COLOR: egui::Color32 = egui::Color32::WHITE;
        const CELL_BORDER_COLOR: egui::Color32 = egui::Color32::BLACK;

        egui::Grid::new("pathfinding_grid")
            .spacing([horizontal_gap, vertical_gap])
            .show(ui, |ui| {
                for row in 0..self.rows {
                    for column in 0..self.columns {
                        let cell_size = egui::vec2(cell_width, cell_height);

                        let (rect, response) =
                            ui.allocate_exact_size(cell_size, egui::Sense::hover());

                        ui.painter().rect_filled(rect, CELL_CORNER_RADIUS, CELL_BACKGROUND_COLOR);

                        ui.painter().rect_stroke(
                            rect,
                            CELL_CORNER_RADIUS,
                            egui::Stroke::new(CELL_BORDER_WIDTH, CELL_BORDER_COLOR),
                            egui::StrokeKind::Inside,
                        );

                        response.on_hover_text(format!("Row: {}, Column: {}", row, column));
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
                }
            }
            egui::ComboBox::from_label("Select Algorithm")
                .selected_text("Select Algorithm")
                .show_ui(ui, |ui| {
                    for algorithm in path_finding_algorithms.iter() {
                        ui.selectable_value(
                            &mut self.Algorithm,
                            algorithm.to_string(),
                            algorithm.to_string(),
                        );
                    }
                });
            ui.label("Drawing tool:");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::DrawWall, "Walls");
            ui.selectable_value(&mut self.drawing_tool, DrawingTool::EraseWall, "Eraser");
        });
    }
}
