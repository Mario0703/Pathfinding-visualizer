#[derive(Clone, Copy, Default, PartialEq)]
pub(super) enum DrawingTool {
    #[default]
    DrawWall,
    DrawWeight(u32),
    EraseWall,
    DrawStart,
    DrawEnd,
}
