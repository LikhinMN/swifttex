#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Text(char),
    Group(Vec<Node>),
    Fraction { numer: Box<Node>, denom: Box<Node> },
    Superscript { base: Box<Node>, exp: Box<Node> },
    Subscript { base: Box<Node>, sub: Box<Node> },
    SquareRoot { inner: Box<Node> },
    Operator(String),
    Symbol(char),
    Unknown(String),
    Matrix {
        rows: Vec<Vec<Node>>,
        env: MatrixEnv,
    },
    Delimiter {
        open: DelimChar,
        close: DelimChar,
        inner: Box<Node>,
        sizing: DelimSizing,
    },
    BigOp {
        op: BigOpKind,
        lower: Option<Box<Node>>,
        upper: Option<Box<Node>>,
    },
    Accent {
        kind: AccentKind,
        inner: Box<Node>,
    },
    TextOp(String),
    Spacing(f64),
    Style {
        style: TextStyle,
        inner: Box<Node>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextStyle {
    Text,
    Bold,
    Calligraphic,
    Blackboard,
    Roman,
    Italic,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccentKind {
    Hat,
    Bar,
    Vec,
    Dot,
    DDot,
    Tilde,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatrixEnv {
    Plain,      // matrix
    Pmatrix,    // ( )
    Bmatrix,    // [ ]
    Vmatrix,    // | |
    Cases,      // { left only
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DelimChar {
    Paren,      // ( )
    Bracket,    // [ ]
    Brace,      // { }
    Vert,       // | |
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DelimSizing {
    Auto,
    Fixed(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BigOpKind {
    Sum,
    Integral,
    Product,
    Union,
    Intersect,
}
