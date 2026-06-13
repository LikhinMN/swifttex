pub mod ast;

use ast::Node;
use swifttex_lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
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
                
                if cmd == "frac" {
                    let numer = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    let denom = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    Some(Node::Fraction {
                        numer: Box::new(numer),
                        denom: Box::new(denom),
                    })
                } else if cmd == "sqrt" {
                    let inner = self.parse_group_or_atom().unwrap_or_else(|| Node::Group(vec![]));
                    Some(Node::SquareRoot { inner: Box::new(inner) })
                } else if let Some(c) = greek_to_char(&cmd) {
                    Some(Node::Symbol(c))
                } else {
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

    fn parse_group_or_atom(&mut self) -> Option<Node> {
        self.skip_whitespace();
        self.parse_atom()
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
