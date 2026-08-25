use super::drawing_tool::DrawingTool;

#[derive(Default)]
pub struct DrawingManager {
    pub(super) tool: DrawingTool,
}

impl DrawingManager {
    pub fn reset(&mut self) {
        self.tool = DrawingTool::DrawWall;
    }

    pub fn set_weights_enabled(&mut self, enabled: bool) {
        if !enabled && matches!(self.tool, DrawingTool::DrawWeight(_)) {
            self.tool = DrawingTool::DrawWall;
        }
    }
}
