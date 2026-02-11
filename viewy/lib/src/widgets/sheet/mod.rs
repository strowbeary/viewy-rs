use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum SheetEdge {
    Right,
    Left,
    Bottom,
}

impl Display for SheetEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SheetEdge::Right => write!(f, "right"),
            SheetEdge::Left => write!(f, "left"),
            SheetEdge::Bottom => write!(f, "bottom"),
        }
    }
}
