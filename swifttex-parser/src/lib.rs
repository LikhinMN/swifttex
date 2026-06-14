//! Parses a tokenized LaTeX math expression into an `Node` tree.
//!
//! # Examples
//! ```
//! use swifttex_parser::parse_to_nodes;
//! let nodes = parse_to_nodes(r"\frac{x^2}{y}", None);
//! assert_eq!(nodes.len(), 1);
//! ```

pub use swifttex_plugin_api::ast;

use ast::{Node, MatrixEnv, DelimChar, DelimSizing, BigOpKind, AccentKind};
use swifttex_lexer::{Lexer, Token};

use std::sync::{Arc, Mutex};
use swifttex_plugin_api::PluginRegistry;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    registry: Option<Arc<Mutex<PluginRegistry>>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self::with_registry(input, None)
    }

    pub fn with_registry(input: &'a str, registry: Option<Arc<Mutex<PluginRegistry>>>) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self { lexer, current_token, registry }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn skip_whitespace(&mut self) {
        while self.current_token == Token::Whitespace {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            self.skip_whitespace();
            if self.current_token == Token::EOF {
                break;
            }
            if self.current_token == Token::RBrace {
                break;
            }
            if let Some(atom) = self.parse_atom() {
                let node = self.parse_postfix(atom);
                nodes.push(node);
            }
        }
        nodes
    }

    fn parse_atom(&mut self) -> Option<Node> {
        self.skip_whitespace();
        match &self.current_token {
            Token::Letter(c) | Token::Digit(c) => {
                let node = Node::Text(*c);
                self.advance();
                Some(node)
            }
            Token::LBrace => {
                self.advance();
                let mut children = Vec::new();
                while self.current_token != Token::EOF && self.current_token != Token::RBrace {
                    self.skip_whitespace();
                    if self.current_token == Token::RBrace || self.current_token == Token::EOF {
                        break;
                    }
                    if let Some(atom) = self.parse_atom() {
                        let node = self.parse_postfix(atom);
                        children.push(node);
                    }
                }
                if self.current_token == Token::RBrace {
                    self.advance();
                }
                Some(Node::Group(children))
            }
            Token::Command(cmd) => {
                let cmd = cmd.clone();
                self.advance();
                
                if cmd == "begin" {
                    self.parse_environment()
                } else if cmd == "left" {
                    self.parse_left_right()
                } else if cmd == "big" || cmd == "Big" || cmd == "bigg" || cmd == "Bigg" {
                    let sizing = match cmd.as_str() {
                        "big" => 1,
                        "Big" => 2,
                        "bigg" => 3,
                        "Bigg" => 4,
                        _ => unreachable!(),
                    };
                    let open = self.parse_delim_char();
                    Some(Node::Delimiter {
                        open,
                        close: ast::DelimChar::None,
                        inner: Box::new(Node::Group(vec![])),
                        sizing: ast::DelimSizing::Fixed(sizing),
                    })
                } else if let Some(kind) = self.parse_accent_kind(&cmd) {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Accent { kind, inner: Box::new(inner) })
                } else if is_text_op(&cmd) {
                    Some(Node::TextOp(cmd.clone()))
                } else if let Some(space) = parse_space_command(&cmd) {
                    Some(Node::Spacing(space))
                } else if let Some(sym) = parse_misc_symbol(&cmd) {
                    Some(Node::Symbol(sym))
                } else if let Some(op) = self.parse_big_op_kind(&cmd) {
                    let mut lower = None;
                    let mut upper = None;
                    self.skip_whitespace();
                    if self.current_token == Token::Underscore {
                        self.advance();
                        lower = Some(Box::new(self.parse_group_or_atom().unwrap_or(Node::Group(vec![]))));
                    } else if self.current_token == Token::Caret {
                        self.advance();
                        upper = Some(Box::new(self.parse_group_or_atom().unwrap_or(Node::Group(vec![]))));
                    }
                    self.skip_whitespace();
                    if self.current_token == Token::Underscore && lower.is_none() {
                        self.advance();
                        lower = Some(Box::new(self.parse_group_or_atom().unwrap_or(Node::Group(vec![]))));
                    } else if self.current_token == Token::Caret && upper.is_none() {
                        self.advance();
                        upper = Some(Box::new(self.parse_group_or_atom().unwrap_or(Node::Group(vec![]))));
                    }
                    Some(Node::BigOp { op, lower, upper })
                } else if cmd == "frac" {
                    let numer = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    let denom = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    Some(Node::Fraction {
                        numer: Box::new(numer),
                        denom: Box::new(denom),
                    })
                } else if cmd == "sqrt" {
                    let inner = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    Some(Node::SquareRoot { inner: Box::new(inner) })
                } else if cmd == "mathbf" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Bold, inner: Box::new(inner) })
                } else if cmd == "mathcal" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Calligraphic, inner: Box::new(inner) })
                } else if cmd == "mathbb" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Blackboard, inner: Box::new(inner) })
                } else if cmd == "mathrm" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Roman, inner: Box::new(inner) })
                } else if cmd == "mathit" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Italic, inner: Box::new(inner) })
                } else if cmd == "text" {
                    let inner = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                    Some(Node::Style { style: ast::TextStyle::Text, inner: Box::new(inner) })
                } else if let Some(c) = greek_to_char(&cmd) {
                    Some(Node::Symbol(c))
                } else {
                    let registry_opt = self.registry.clone();
                    if let Some(registry) = registry_opt {
                        let reg = registry.lock().unwrap();
                        if let Some(c) = reg.resolve_symbol(&cmd) {
                            return Some(Node::Symbol(c));
                        }
                        if reg.has_command(&cmd) {
                            let arity = reg.command_arity(&cmd).unwrap_or(0);
                            drop(reg); // unlock before parsing inner args
                            
                            let mut args = Vec::new();
                            for _ in 0..arity {
                                let arg_node = self.parse_group_or_atom().unwrap_or(Node::Group(vec![]));
                                let inner = match arg_node {
                                    Node::Group(g) => g,
                                    n => vec![n],
                                };
                                args.push(inner);
                            }
                            
                            let reg = registry.lock().unwrap();
                            return Some(reg.resolve_command(&cmd, args).unwrap());
                        }
                    }
                    Some(Node::Unknown(cmd))
                }
            }
            Token::EOF | Token::RBrace => None,
            _ => {
                // Ignore unexpected token like solitary Caret or Underscore
                self.advance();
                None
            }
        }
    }

    fn parse_postfix(&mut self, mut left: Node) -> Node {
        loop {
            self.skip_whitespace();
            if self.current_token == Token::Caret {
                self.advance();
                let exp = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                left = Node::Superscript {
                    base: Box::new(left),
                    exp: Box::new(exp),
                };
            } else if self.current_token == Token::Underscore {
                self.advance();
                let sub = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                left = Node::Subscript {
                    base: Box::new(left),
                    sub: Box::new(sub),
                };
            } else {
                break;
            }
        }
        left
    }


    fn parse_environment(&mut self) -> Option<Node> {
        let env_name = self.parse_env_name()?;
        let is_matrix = matches!(env_name.as_str(), "matrix" | "pmatrix" | "bmatrix" | "vmatrix" | "cases");
        
        if is_matrix {
            let env = match env_name.as_str() {
                "matrix" => MatrixEnv::Plain,
                "pmatrix" => MatrixEnv::Pmatrix,
                "bmatrix" => MatrixEnv::Bmatrix,
                "vmatrix" => MatrixEnv::Vmatrix,
                "cases" => MatrixEnv::Cases,
                _ => unreachable!(),
            };
            
            let mut rows = Vec::new();
            let mut current_row = Vec::new();
            let mut current_cell = Vec::new();
            
            loop {
                self.skip_whitespace();
                match &self.current_token {
                    Token::EOF => break,
                    Token::Command(c) if c == "end" => {
                        self.advance();
                        let _end_env = self.parse_env_name();
                        break;
                    }
                    Token::Ampersand => {
                        self.advance();
                        current_row.push(Node::Group(current_cell));
                        current_cell = Vec::new();
                    }
                    Token::Newline => {
                        self.advance();
                        current_row.push(Node::Group(current_cell));
                        rows.push(current_row);
                        current_row = Vec::new();
                        current_cell = Vec::new();
                    }
                    _ => {
                        if let Some(atom) = self.parse_atom() {
                            let node = self.parse_postfix(atom);
                            current_cell.push(node);
                        } else {
                            self.advance();
                        }
                    }
                }
            }
            
            current_row.push(Node::Group(current_cell));
            rows.push(current_row);
            
            Some(Node::Matrix { rows, env })
        } else {
            let mut inner = Vec::new();
            loop {
                self.skip_whitespace();
                match &self.current_token {
                    Token::EOF => break,
                    Token::Command(c) if c == "end" => {
                        self.advance();
                        let _end_env = self.parse_env_name();
                        break;
                    }
                    _ => {
                        if let Some(atom) = self.parse_atom() {
                            let node = self.parse_postfix(atom);
                            inner.push(node);
                        } else {
                            self.advance();
                        }
                    }
                }
            }
            let registry_opt = self.registry.clone();
            if let Some(registry) = registry_opt {
                let reg = registry.lock().unwrap();
                if reg.has_environment(&env_name) {
                    return reg.resolve_environment(&env_name, inner);
                }
            }
            Some(Node::Group(inner))
        }
    }

    fn parse_env_name(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.current_token == Token::LBrace {
            self.advance();
            let mut name = String::new();
            while let Token::Letter(c) = self.current_token {
                name.push(c);
                self.advance();
            }
            if self.current_token == Token::RBrace {
                self.advance();
            }
            Some(name)
        } else {
            None
        }
    }

    fn parse_left_right(&mut self) -> Option<Node> {
        let open = self.parse_delim_char();
        let mut inner_nodes = Vec::new();
        
        loop {
            self.skip_whitespace();
            if self.current_token == Token::EOF {
                break;
            }
            if let Token::Command(cmd) = &self.current_token {
                if cmd == "right" {
                    self.advance();
                    break;
                }
            }
            if let Some(atom) = self.parse_atom() {
                let node = self.parse_postfix(atom);
                inner_nodes.push(node);
            } else {
                self.advance();
            }
        }
        
        let close = self.parse_delim_char();
        
        Some(Node::Delimiter {
            open,
            close,
            inner: Box::new(Node::Group(inner_nodes)),
            sizing: DelimSizing::Auto,
        })
    }

    fn parse_delim_char(&mut self) -> DelimChar {
        self.skip_whitespace();
        let delim = match &self.current_token {
            Token::Letter('(') => DelimChar::Paren,
            Token::Letter(')') => DelimChar::Paren,
            Token::Letter('[') => DelimChar::Bracket,
            Token::Letter(']') => DelimChar::Bracket,
            Token::LBrace => DelimChar::Brace,
            Token::RBrace => DelimChar::Brace,
            Token::Letter('|') => DelimChar::Vert,
            Token::Command(c) if c == "{" || c == "}" || c == "lbrace" || c == "rbrace" => DelimChar::Brace,
            Token::Command(c) if c == "|" || c == "vert" || c == "Vert" => DelimChar::Vert,
            Token::Letter(c) if *c == '.' => DelimChar::None, // \left.
            _ => {
                // If it's a character that matches delimiter syntax but isn't explicitly checked above
                if let Token::Letter(c) = self.current_token {
                    if c == '.' { return { self.advance(); DelimChar::None }; }
                }
                DelimChar::None
            }
        };
        self.advance();
        delim
    }

    fn parse_big_op_kind(&self, cmd: &str) -> Option<BigOpKind> {
        match cmd {
            "sum" => Some(BigOpKind::Sum),
            "int" => Some(BigOpKind::Integral),
            "prod" => Some(BigOpKind::Product),
            "bigcup" => Some(BigOpKind::Union),
            "bigcap" => Some(BigOpKind::Intersect),
            _ => None,
        }
    }

    fn parse_group_or_atom(&mut self) -> Option<Node> {
        self.skip_whitespace();
        self.parse_atom()
    }
    fn parse_accent_kind(&self, cmd: &str) -> Option<AccentKind> {
        match cmd {
            "hat" => Some(AccentKind::Hat),
            "bar" => Some(AccentKind::Bar),
            "vec" => Some(AccentKind::Vec),
            "dot" => Some(AccentKind::Dot),
            "ddot" => Some(AccentKind::DDot),
            "tilde" => Some(AccentKind::Tilde),
            _ => None,
        }
    }
}

fn is_text_op(cmd: &str) -> bool {
    matches!(cmd, "sin" | "cos" | "tan" | "log" | "ln" | "exp" | "lim" | "max" | "min" | "inf" | "sup" | "det")
}

fn parse_space_command(cmd: &str) -> Option<f64> {
    match cmd {
        "," => Some(0.167),
        ":" => Some(0.222),
        ";" => Some(0.278),
        "!" => Some(-0.167),
        "quad" => Some(1.0),
        "qquad" => Some(2.0),
        _ => None,
    }
}

fn parse_misc_symbol(cmd: &str) -> Option<char> {
    match cmd {
        "infty" => Some('∞'),
        "cdot" => Some('·'),
        "cdots" => Some('⋯'),
        "ldots" => Some('…'),
        "partial" => Some('∂'),
        "nabla" => Some('∇'),
        "pm" => Some('±'),
        "times" => Some('×'),
        "div" => Some('÷'),
        "leq" => Some('≤'),
        "geq" => Some('≥'),
        "neq" => Some('≠'),
        "approx" => Some('≈'),
        "equiv" => Some('≡'),
        "in" => Some('∈'),
        "notin" => Some('∉'),
        "subset" => Some('⊂'),
        "supset" => Some('⊃'),
        "cup" => Some('∪'),
        "cap" => Some('∩'),
        "to" => Some('→'),
        "gets" => Some('←'),
        "Rightarrow" => Some('⇒'),
        "Leftarrow" => Some('⇐'),
        "Leftrightarrow" => Some('⟺'),
        "forall" => Some('∀'),
        "exists" => Some('∃'),
        "emptyset" => Some('∅'),
        _ => None,
    }
}

fn greek_to_char(name: &str) -> Option<char> {
    match name {
        "alpha" => Some('α'),
        "beta" => Some('β'),
        "gamma" => Some('γ'),
        "delta" => Some('δ'),
        "epsilon" => Some('ε'),
        "zeta" => Some('ζ'),
        "eta" => Some('η'),
        "theta" => Some('θ'),
        "iota" => Some('ι'),
        "kappa" => Some('κ'),
        "lambda" => Some('λ'),
        "mu" => Some('μ'),
        "nu" => Some('ν'),
        "xi" => Some('ξ'),
        "omicron" => Some('ο'),
        "pi" => Some('π'),
        "rho" => Some('ρ'),
        "sigma" => Some('σ'),
        "tau" => Some('τ'),
        "upsilon" => Some('υ'),
        "phi" => Some('φ'),
        "chi" => Some('χ'),
        "psi" => Some('ψ'),
        "omega" => Some('ω'),
        "Alpha" => Some('Α'),
        "Beta" => Some('Β'),
        "Gamma" => Some('Γ'),
        "Delta" => Some('Δ'),
        "Epsilon" => Some('Ε'),
        "Zeta" => Some('Ζ'),
        "Eta" => Some('Η'),
        "Theta" => Some('Θ'),
        "Iota" => Some('Ι'),
        "Kappa" => Some('Κ'),
        "Lambda" => Some('Λ'),
        "Mu" => Some('Μ'),
        "Nu" => Some('Ν'),
        "Xi" => Some('Ξ'),
        "Omicron" => Some('Ο'),
        "Pi" => Some('Π'),
        "Rho" => Some('Ρ'),
        "Sigma" => Some('Σ'),
        "Tau" => Some('Τ'),
        "Upsilon" => Some('Υ'),
        "Phi" => Some('Φ'),
        "Chi" => Some('Χ'),
        "Psi" => Some('Ψ'),
        "Omega" => Some('Ω'),
        _ => None,
    }
}


pub fn parse_to_nodes(input: &str, registry: Option<Arc<Mutex<PluginRegistry>>>) -> Vec<Node> {
    let mut parser = Parser::with_registry(input, registry);
    parser.parse()
}
