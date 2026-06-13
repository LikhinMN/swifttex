use swifttex_parser::ast::{MatrixEnv, DelimChar};

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
    Matrix {
        cells: Vec<Vec<MathBox>>,
        col_widths: Vec<f64>,
        row_heights: Vec<f64>,
        total_width: f64,
        total_height: f64,
        env: MatrixEnv,
    },
    Delim {
        open: DelimChar,
        close: DelimChar,
        inner: Box<MathBox>,
        delim_height: f64,
        width: f64,
        height: f64,
        depth: f64,
    },
    BigOp {
        op_box: Box<MathBox>,
        lower: Option<Box<MathBox>>,
        upper: Option<Box<MathBox>>,
        width: f64,
        height: f64,
        depth: f64,
    },
    TextOp {
        text: String,
        width: f64,
        height: f64,
        depth: f64,
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
            Self::Matrix { total_width, .. } => *total_width,
            Self::Delim { width, .. } => *width,
            Self::BigOp { width, .. } => *width,
            Self::TextOp { width, .. } => *width,
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
            Self::Matrix { total_height, .. } => *total_height,
            Self::Delim { height, .. } => *height,
            Self::BigOp { height, .. } => *height,
            Self::TextOp { height, .. } => *height,
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
            Self::Matrix { .. } => 0.0,
            Self::Delim { depth, .. } => *depth,
            Self::BigOp { depth, .. } => *depth,
            Self::TextOp { depth, .. } => *depth,
        }
    }
}
