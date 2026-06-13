#[derive(Debug, Clone)]
pub enum MathBox {
    Glyph {
        ch: char,
        width: f64,
        height: f64,
        depth: f64,
    },
    HBox {
        children: Vec<MathBox>,
        width: f64,
        height: f64,
        depth: f64,
    },
    VBox {
        children: Vec<MathBox>,
        width: f64,
        height: f64,
        depth: f64,
    },
    RuleBox {
        width: f64,
        height: f64,
        depth: f64,
    },
    ShiftedBox {
        inner: Box<MathBox>,
        shift_x: f64,
        shift_y: f64,
        width: f64,
        height: f64,
        depth: f64,
    },
    Glue {
        width: f64,
    },
}

impl MathBox {
    pub fn width(&self) -> f64 {
        match self {
            Self::Glyph { width, .. } => *width,
            Self::HBox { width, .. } => *width,
            Self::VBox { width, .. } => *width,
            Self::RuleBox { width, .. } => *width,
            Self::ShiftedBox { width, .. } => *width,
            Self::Glue { width } => *width,
        }
    }

    pub fn height(&self) -> f64 {
        match self {
            Self::Glyph { height, .. } => *height,
            Self::HBox { height, .. } => *height,
            Self::VBox { height, .. } => *height,
            Self::RuleBox { height, .. } => *height,
            Self::ShiftedBox { height, .. } => *height,
            Self::Glue { .. } => 0.0,
        }
    }

    pub fn depth(&self) -> f64 {
        match self {
            Self::Glyph { depth, .. } => *depth,
            Self::HBox { depth, .. } => *depth,
            Self::VBox { depth, .. } => *depth,
            Self::RuleBox { depth, .. } => *depth,
            Self::ShiftedBox { depth, .. } => *depth,
            Self::Glue { .. } => 0.0,
        }
    }
}
