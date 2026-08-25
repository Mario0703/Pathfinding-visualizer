use crate::drawing::{drawing_manager::DrawingManager, drawing_tool::DrawingTool};
use eframe::egui;

use super::grid_drawing::{
    COST_5_COLOR, COST_10_COLOR, COST_15_COLOR, HIGH_TERRAIN_WEIGHT, LOW_TERRAIN_WEIGHT,
    MEDIUM_TERRAIN_WEIGHT,
};

impl DrawingManager {
    pub fn show_weight_controls(&mut self, ui: &mut egui::Ui, weights_enabled: bool) {
        if !weights_enabled {
            return;
        }

        ui.label("Terrain weights:");
        self.show_weight_button(ui, LOW_TERRAIN_WEIGHT, COST_5_COLOR);
        self.show_weight_button(ui, MEDIUM_TERRAIN_WEIGHT, COST_10_COLOR);
        self.show_weight_button(ui, HIGH_TERRAIN_WEIGHT, COST_15_COLOR);
        ui.end_row();
    }

    pub fn show_tool_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Drawing tool:");
        ui.selectable_value(&mut self.tool, DrawingTool::DrawWall, "Walls");
        ui.selectable_value(&mut self.tool, DrawingTool::EraseWall, "Eraser");
        ui.selectable_value(&mut self.tool, DrawingTool::DrawStart, "Start");
        ui.selectable_value(&mut self.tool, DrawingTool::DrawEnd, "End");
        ui.end_row();
    }

    fn show_weight_button(&mut self, ui: &mut egui::Ui, weight: u32, color: egui::Color32) {
        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new(format!("Cost {weight}")).color(egui::Color32::BLACK),
                )
                .fill(color)
                .selected(self.tool == DrawingTool::DrawWeight(weight)),
            )
            .on_hover_text(format!("Paint cells with a traversal cost of {weight}"))
            .clicked()
        {
            self.tool = DrawingTool::DrawWeight(weight);
        }
    }
}
