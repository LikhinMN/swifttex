//! Renders a LaTeX math expression to a MathML string.
//!
//! Output is valid MathML 3.0 with xmlns attribute,
//! suitable for embedding in HTML5 documents.
//!
//! # Examples
//! ```
//! use swifttex_renderer_mathml::MathMLRenderer;
//! let out = MathMLRenderer::new(false).render(r"x^2").unwrap();
//! assert!(out.mathml.contains("<math"));
//! ```

use swifttex_parser::ast::*;
use swifttex_parser::Parser;
use swifttex_plugin_api::PluginRegistry;
use std::sync::{Arc, Mutex};

pub struct MathMLRenderer {
    pub display_mode: bool,
    pub registry: Option<Arc<Mutex<PluginRegistry>>>,
}

pub struct MathMLOutput {
    pub mathml: String,
    pub has_error: bool,
}

impl MathMLRenderer {
    pub fn new(display_mode: bool) -> Self {
        Self { display_mode, registry: None }
    }

    pub fn with_registry(mut self, registry: Arc<Mutex<PluginRegistry>>) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn render(&self, input: &str) -> Result<MathMLOutput, String> {
        let mut parser = Parser::with_registry(input, self.registry.clone());
        let nodes = parser.parse();
        self.render_from_nodes(input, &nodes)
    }

    pub fn render_from_nodes(&self, input: &str, nodes: &[Node]) -> Result<MathMLOutput, String> {
        let inner = self.render_nodes(nodes);
        let escaped_input = escape_html(input);
        let display = if self.display_mode { "block" } else { "inline" };
        
        let mathml = format!(
            r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="{}" aria-label="{}">{}</math>"#,
            display, escaped_input, inner
        );
        Ok(MathMLOutput { mathml, has_error: false })
    }

    fn render_nodes(&self, nodes: &[Node]) -> String {
        nodes.iter().map(|n| self.render_node(n)).collect::<Vec<_>>().join("")
    }

    fn render_node(&self, node: &Node) -> String {
        match node {
            Node::Text(ch) => {
                let escaped = escape_char(*ch);
                if ch.is_alphabetic() {
                    format!("<mi>{}</mi>", escaped)
                } else if ch.is_ascii_digit() {
                    format!("<mn>{}</mn>", escaped)
                } else {
                    format!("<mo>{}</mo>", escaped)
                }
            }
            Node::Symbol(ch) => {
                let escaped = escape_char(*ch);
                if ch.is_uppercase() && ch.is_alphabetic() {
                    format!("<mi mathvariant=\"normal\">{}</mi>", escaped)
                } else if ch.is_lowercase() && ch.is_alphabetic() {
                    format!("<mi>{}</mi>", escaped)
                } else {
                    format!("<mo>{}</mo>", escaped)
                }
            }
            Node::Group(children) => {
                format!("<mrow>{}</mrow>", self.render_nodes(children))
            }
            Node::Fraction { numer, denom } => {
                format!("<mfrac><mrow>{}</mrow><mrow>{}</mrow></mfrac>", self.render_node(numer), self.render_node(denom))
            }
            Node::Superscript { base, exp } => {
                format!("<msup>{}<mrow>{}</mrow></msup>", self.render_node(base), self.render_node(exp))
            }
            Node::Subscript { base, sub } => {
                format!("<msub>{}<mrow>{}</mrow></msub>", self.render_node(base), self.render_node(sub))
            }
            Node::SquareRoot { inner } => {
                format!("<msqrt><mrow>{}</mrow></msqrt>", self.render_node(inner))
            }
            Node::Operator(s) => {
                format!("<mo>{}</mo>", escape_html(s))
            }
            Node::TextOp(name) => {
                format!("<mi mathvariant=\"normal\">{}</mi>", escape_html(name))
            }
            Node::Spacing(em) => {
                format!("<mspace width=\"{}em\"/>", em)
            }
            Node::Unknown(s) => {
                format!("<!-- unknown: {} -->", escape_html(s))
            }
            Node::Style { style, inner } => {
                let variant = match style {
                    TextStyle::Text => "normal",
                    TextStyle::Bold => "bold",
                    TextStyle::Calligraphic => "script",
                    TextStyle::Blackboard => "double-struck",
                    TextStyle::Roman => "normal",
                    TextStyle::Italic => "italic",
                };
                format!("<mstyle mathvariant=\"{}\"><mrow>{}</mrow></mstyle>", variant, self.render_node(inner))
            }
            Node::Accent { kind, inner } => {
                let accent_char = match kind {
                    AccentKind::Hat => '^',
                    AccentKind::Bar => '¯',
                    AccentKind::Vec => '→',
                    AccentKind::Dot => '˙',
                    AccentKind::DDot => '¨',
                    AccentKind::Tilde => '˜',
                };
                format!("<mover><mrow>{}</mrow><mo>{}</mo></mover>", self.render_node(inner), accent_char)
            }
            Node::Matrix { rows, env } => {
                let mut table = String::new();
                table.push_str("<mtable>");
                for row in rows {
                    table.push_str("<mtr>");
                    for cell in row {
                        table.push_str("<mtd><mrow>");
                        table.push_str(&self.render_node(cell));
                        table.push_str("</mrow></mtd>");
                    }
                    table.push_str("</mtr>");
                }
                table.push_str("</mtable>");
                
                match env {
                    MatrixEnv::Plain => table,
                    MatrixEnv::Pmatrix => format!("<mrow><mo>(</mo>{}<mo>)</mo></mrow>", table),
                    MatrixEnv::Bmatrix => format!("<mrow><mo>[</mo>{}<mo>]</mo></mrow>", table),
                    MatrixEnv::Vmatrix => format!("<mrow><mo>|</mo>{}<mo>|</mo></mrow>", table),
                    MatrixEnv::Cases => format!("<mrow><mo>{{</mo>{}</mrow>", table),
                }
            }
            Node::Delimiter { open, close, inner, .. } => {
                let open_str = match open {
                    DelimChar::Paren => "(",
                    DelimChar::Bracket => "[",
                    DelimChar::Brace => "{",
                    DelimChar::Vert => "|",
                    DelimChar::None => "",
                };
                let close_str = match close {
                    DelimChar::Paren => ")",
                    DelimChar::Bracket => "]",
                    DelimChar::Brace => "}",
                    DelimChar::Vert => "|",
                    DelimChar::None => "",
                };
                let mut out = String::from("<mrow>");
                if !open_str.is_empty() {
                    out.push_str(&format!("<mo stretchy=\"true\">{}</mo>", open_str));
                }
                out.push_str(&self.render_node(inner));
                if !close_str.is_empty() {
                    out.push_str(&format!("<mo stretchy=\"true\">{}</mo>", close_str));
                }
                out.push_str("</mrow>");
                out
            }
            Node::BigOp { op, lower, upper } => {
                let op_char = match op {
                    BigOpKind::Sum => '∑',
                    BigOpKind::Integral => '∫',
                    BigOpKind::Product => '∏',
                    BigOpKind::Union => '⋃',
                    BigOpKind::Intersect => '⋂',
                };
                let op_mo = format!("<mo>{}</mo>", op_char);
                match (lower, upper) {
                    (Some(l), Some(u)) => format!("<munderover>{}<mrow>{}</mrow><mrow>{}</mrow></munderover>", op_mo, self.render_node(l), self.render_node(u)),
                    (Some(l), None) => format!("<munder>{}<mrow>{}</mrow></munder>", op_mo, self.render_node(l)),
                    (None, Some(u)) => format!("<mover>{}<mrow>{}</mrow></mover>", op_mo, self.render_node(u)),
                    (None, None) => op_mo,
                }
            }
        }
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn escape_char(c: char) -> String {
    match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#39;".to_string(),
        _ => c.to_string(),
    }
}
