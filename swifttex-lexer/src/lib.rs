//! Tokenizes a LaTeX math string into a flat stream of [`Token`]s.
//!
//! The lexer handles command sequences, grouping delimiters,
//! script markers, and whitespace collapsing.
//!
//! # Examples
//! ```
//! use swifttex_lexer::{Lexer, Token};
//! let mut lex = Lexer::new(r"\frac{1}{2}");
//! assert!(matches!(lex.next_token(), Token::Command(_)));
//! ```

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Letter(char),
    Digit(char),
    Command(String),
    LBrace,
    RBrace,
    Caret,
    Underscore,
    Ampersand,
    Newline,
    Whitespace,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self, c: char) {
        self.pos += c.len_utf8();
    }

    pub fn next_token(&mut self) -> Token {
        let c = match self.peek_char() {
            Some(c) => c,
            None => return Token::EOF,
        };

        if c.is_ascii_whitespace() {
            self.advance(c);
            while let Some(next_c) = self.peek_char() {
                if next_c.is_ascii_whitespace() {
                    self.advance(next_c);
                } else {
                    break;
                }
            }
            return Token::Whitespace;
        }

        if c == '\\' {
            self.advance(c);
            let next_c = match self.peek_char() {
                Some(nc) => nc,
                None => return Token::Command("".to_string()),
            };
            
            if next_c == '\\' {
                self.advance(next_c);
                return Token::Newline;
            }
            
            let mut cmd = String::new();
            if next_c.is_ascii_alphabetic() {
                while let Some(nc) = self.peek_char() {
                    if nc.is_ascii_alphabetic() {
                        cmd.push(nc);
                        self.advance(nc);
                    } else {
                        break;
                    }
                }
            } else {
                cmd.push(next_c);
                self.advance(next_c);
            }
            return Token::Command(cmd);
        }

        self.advance(c);
        match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '^' => Token::Caret,
            '_' => Token::Underscore,
            '&' => Token::Ampersand,
            x if x.is_ascii_alphabetic() => Token::Letter(x),
            x if x.is_ascii_digit() => Token::Digit(x),
            x => Token::Letter(x),
        }
    }
}
