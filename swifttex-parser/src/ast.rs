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
}
