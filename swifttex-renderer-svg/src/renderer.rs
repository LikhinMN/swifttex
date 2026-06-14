use crate::glyph_paths::{glyph_path_or_text, GlyphRender};
use swifttex_layout::{LayoutEngine, MathBox};
use swifttex_parser::Parser;
use swifttex_layout::metrics::glyph_metrics;

use swifttex_layout::style::MathStyle;
use swifttex_plugin_api::PluginRegistry;
use std::sync::{Arc, Mutex};

pub struct SvgRenderer {
    pub font_size: f64,
    pub display_mode: bool,
    pub inline_fonts: bool,
    pub math_style: Option<MathStyle>,
    pub registry: Option<Arc<Mutex<PluginRegistry>>>,
}

pub struct RenderOutput {
    pub svg: String,
    pub width: f64,
    pub height: f64,
}

impl SvgRenderer {
    pub fn new(font_size: f64, display_mode: bool, inline_fonts: bool) -> Self {
        Self { font_size, display_mode, inline_fonts, math_style: None, registry: None }
    }
    
    pub fn with_registry(mut self, registry: Arc<Mutex<PluginRegistry>>) -> Self {
        self.registry = Some(registry);
        self
    }
    
    pub fn with_math_style(mut self, style: MathStyle) -> Self {
        self.math_style = Some(style);
        self
    }

    pub fn render(&self, input: &str) -> Result<RenderOutput, String> {
        let mut parser = Parser::with_registry(input, self.registry.clone());
        let nodes = parser.parse();
        self.render_from_nodes(input, &nodes)
    }

    pub fn render_from_nodes(&self, input: &str, nodes: &[swifttex_parser::ast::Node]) -> Result<RenderOutput, String> {
        let mut engine = LayoutEngine::new(self.font_size, self.display_mode);
        if let Some(s) = self.math_style {
            engine = engine.with_style(s);
        }
        let root_box = engine.layout_nodes(nodes);
        
        let mut buf = String::new();
        let total_width = root_box.width();
        let total_height = root_box.height() + root_box.depth();
        
        let start_x = 0.0;
        let start_y = root_box.height();
        
        self.render_box(&root_box, start_x, start_y, &mut buf, None);
        
        let escaped_input = escape_html(input);
        let aria_label_text = escape_html(&generate_aria_label(nodes));
        
        let defs = if self.inline_fonts {
            "<defs>\n    <style>\n      @import url('https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css');\n    </style>\n  </defs>\n  "
        } else {
            ""
        };
        
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w:.3}px" height="{h:.3}px" viewBox="0 0 {w:.3} {h:.3}" role="img" aria-label="{aria}">
  {defs}<title>{title}</title>
  {inner}
</svg>"#,
            w = f64::max(total_width, 0.1),
            h = f64::max(total_height, 0.1),
            aria = aria_label_text,
            title = escaped_input,
            defs = defs,
            inner = buf
        );
        
        Ok(RenderOutput {
            svg,
            width: total_width,
            height: total_height,
        })
    }

    fn render_box(&self, mb: &MathBox, x: f64, y: f64, buf: &mut String, current_text_style: Option<&swifttex_parser::ast::TextStyle>) {
        match mb {
            MathBox::Glyph { ch, width, height: _, depth: _ } => {
                let m = glyph_metrics(*ch);
                let local_font_size = if m.width > 0.0 {
                    width / m.width
                } else {
                    self.font_size
                };
                
                let mut font_family = if ch.is_ascii_alphabetic() {
                    "KaTeX_Math"
                } else if ('α'..='ω').contains(ch) || ('Α'..='Ω').contains(ch) {
                    "KaTeX_Math"
                } else {
                    "KaTeX_Main"
                };
                let mut font_style = if font_family == "KaTeX_Math" && ch.is_ascii_alphabetic() {
                    "italic"
                } else {
                    "normal"
                };
                let mut font_weight = "normal";

                if let Some(ts) = current_text_style {
                    match ts {
                        swifttex_parser::ast::TextStyle::Text => {
                            font_family = "KaTeX_Main";
                            font_style = "normal";
                        }
                        swifttex_parser::ast::TextStyle::Bold => {
                            font_family = "KaTeX_Main";
                            font_style = "normal";
                            font_weight = "bold";
                        }
                        swifttex_parser::ast::TextStyle::Calligraphic => {
                            font_family = "KaTeX_Caligraphic";
                            font_style = "normal";
                        }
                        swifttex_parser::ast::TextStyle::Blackboard => {
                            font_family = "KaTeX_AMS";
                            font_style = "normal";
                        }
                        swifttex_parser::ast::TextStyle::Roman => {
                            font_family = "KaTeX_Main";
                            font_style = "normal";
                        }
                        swifttex_parser::ast::TextStyle::Italic => {
                            font_family = "KaTeX_Main";
                            font_style = "italic";
                        }
                    }
                }
                
                let GlyphRender::Text(c) = glyph_path_or_text(*ch);
                buf.push_str(&format!(
                    r#"<text x="{x:.3}" y="{y:.3}" font-family="{family}, serif" font-size="{fs:.3}px" font-style="{style}" font-weight="{weight}" fill="currentColor">{c}</text>"#,
                    x = x,
                    y = y,
                    family = font_family,
                    fs = local_font_size,
                    style = font_style,
                    weight = font_weight,
                    c = escape_char(c)
                ));
            }
            MathBox::HBox { children, .. } => {
                let mut cursor_x = x;
                for child in children {
                    self.render_box(child, cursor_x, y, buf, current_text_style);
                    cursor_x += child.width();
                }
            }
            MathBox::VBox { children, height, .. } => {
                let mut cursor_y = y - height;
                for child in children {
                    cursor_y += child.height();
                    self.render_box(child, x, cursor_y, buf, current_text_style);
                    cursor_y += child.depth();
                }
            }
            MathBox::RuleBox { width, height, .. } => {
                let rect_y = y - height;
                buf.push_str(&format!(
                    r#"<rect x="{x:.3}" y="{rect_y:.3}" width="{w:.3}" height="{h:.3}" fill="currentColor"/>"#,
                    x = x,
                    rect_y = rect_y,
                    w = width,
                    h = height
                ));
            }
            MathBox::ShiftedBox { inner, shift_x, shift_y, .. } => {
                self.render_box(inner, x + shift_x, y - shift_y, buf, current_text_style);
            }
            MathBox::Glue { .. } => {}
            MathBox::Matrix { cells, col_widths, row_heights, total_width: _, total_height, env } => {
                let mut cursor_y = y - total_height;
                
                // Draw open delimiter if needed
                let delim_fs = match total_height {
                    h if *h < 1.2 * self.font_size => self.font_size,
                    h if *h < 1.8 * self.font_size => self.font_size * 1.2,
                    h if *h < 2.4 * self.font_size => self.font_size * 1.8,
                    _ => self.font_size * 2.4,
                };
                let delim_font = match total_height {
                    h if *h < 1.2 * self.font_size => "KaTeX_Main",
                    h if *h < 1.8 * self.font_size => "KaTeX_Size1",
                    h if *h < 2.4 * self.font_size => "KaTeX_Size2",
                    _ => "KaTeX_Size3",
                };

                let open_char = match env {
                    swifttex_parser::ast::MatrixEnv::Plain => None,
                    swifttex_parser::ast::MatrixEnv::Pmatrix => Some('('),
                    swifttex_parser::ast::MatrixEnv::Bmatrix => Some('['),
                    swifttex_parser::ast::MatrixEnv::Vmatrix => Some('|'),
                    swifttex_parser::ast::MatrixEnv::Cases => Some('{'),
                };
                
                let close_char = match env {
                    swifttex_parser::ast::MatrixEnv::Plain => None,
                    swifttex_parser::ast::MatrixEnv::Pmatrix => Some(')'),
                    swifttex_parser::ast::MatrixEnv::Bmatrix => Some(']'),
                    swifttex_parser::ast::MatrixEnv::Vmatrix => Some('|'),
                    swifttex_parser::ast::MatrixEnv::Cases => None,
                };

                let mut content_x = x;
                
                if let Some(c) = open_char {
                    let mut dy = y - total_height / 2.0 + self.font_size * 0.3;
                    if c == '{' { dy -= self.font_size * 0.1; }
                    buf.push_str(&format!(
                        r#"<text x="{x:.3}" y="{y:.3}" font-family="{family}, serif" font-size="{fs:.3}px" font-style="normal" fill="currentColor">{c}</text>"#,
                        x = x,
                        y = dy,
                        family = delim_font,
                        fs = delim_fs,
                        c = escape_char(c)
                    ));
                    content_x += self.font_size * 0.5;
                }

                let col_spacing = self.font_size * 0.5;
                let row_spacing = self.font_size * 0.3;

                for (r_idx, row) in cells.iter().enumerate() {
                    cursor_y += row_heights[r_idx];
                    let mut cursor_x = content_x;
                    for (c_idx, cell) in row.iter().enumerate() {
                        let cell_offset_x = cursor_x + (col_widths[c_idx] - cell.width()) / 2.0; // Center in col
                        self.render_box(cell, cell_offset_x, cursor_y, buf, current_text_style);
                        cursor_x += col_widths[c_idx] + col_spacing;
                    }
                    cursor_y += row_spacing;
                }
                
                if let Some(c) = close_char {
                    let close_x = content_x + col_widths.iter().sum::<f64>() + col_spacing * (col_widths.len().saturating_sub(1) as f64);
                    let dy = y - total_height / 2.0 + self.font_size * 0.3;
                    buf.push_str(&format!(
                        r#"<text x="{x:.3}" y="{y:.3}" font-family="{family}, serif" font-size="{fs:.3}px" font-style="normal" fill="currentColor">{c}</text>"#,
                        x = close_x,
                        y = dy,
                        family = delim_font,
                        fs = delim_fs,
                        c = escape_char(c)
                    ));
                }
            }
            MathBox::Delim { open, close, inner, delim_height, width: _, height: _, depth: _ } => {
                let delim_fs = match delim_height {
                    h if *h < 1.2 * self.font_size => self.font_size,
                    h if *h < 1.8 * self.font_size => self.font_size * 1.2,
                    h if *h < 2.4 * self.font_size => self.font_size * 1.8,
                    _ => self.font_size * 2.4,
                };
                let delim_font = match delim_height {
                    h if *h < 1.2 * self.font_size => "KaTeX_Main",
                    h if *h < 1.8 * self.font_size => "KaTeX_Size1",
                    h if *h < 2.4 * self.font_size => "KaTeX_Size2",
                    _ => "KaTeX_Size3",
                };

                let open_char = match open {
                    swifttex_parser::ast::DelimChar::Paren => Some('('),
                    swifttex_parser::ast::DelimChar::Bracket => Some('['),
                    swifttex_parser::ast::DelimChar::Brace => Some('{'),
                    swifttex_parser::ast::DelimChar::Vert => Some('|'),
                    swifttex_parser::ast::DelimChar::None => None,
                };

                let close_char = match close {
                    swifttex_parser::ast::DelimChar::Paren => Some(')'),
                    swifttex_parser::ast::DelimChar::Bracket => Some(']'),
                    swifttex_parser::ast::DelimChar::Brace => Some('}'),
                    swifttex_parser::ast::DelimChar::Vert => Some('|'),
                    swifttex_parser::ast::DelimChar::None => None,
                };

                let mut cursor_x = x;
                
                if let Some(c) = open_char {
                    let dy = y - delim_height / 2.0 + self.font_size * 0.3;
                    buf.push_str(&format!(
                        r#"<text x="{x:.3}" y="{y:.3}" font-family="{family}, serif" font-size="{fs:.3}px" font-style="normal" fill="currentColor">{c}</text>"#,
                        x = cursor_x,
                        y = dy,
                        family = delim_font,
                        fs = delim_fs,
                        c = escape_char(c)
                    ));
                    cursor_x += self.font_size * 0.4;
                }

                self.render_box(inner, cursor_x, y, buf, current_text_style);
                cursor_x += inner.width();

                if let Some(c) = close_char {
                    let dy = y - delim_height / 2.0 + self.font_size * 0.3;
                    buf.push_str(&format!(
                        r#"<text x="{x:.3}" y="{y:.3}" font-family="{family}, serif" font-size="{fs:.3}px" font-style="normal" fill="currentColor">{c}</text>"#,
                        x = cursor_x,
                        y = dy,
                        family = delim_font,
                        fs = delim_fs,
                        c = escape_char(c)
                    ));
                }
            }
            MathBox::TextOp { text, .. } => {
                let text_elem = format!(
                    r#"<text x="{x:.3}" y="{y:.3}" font-family="KaTeX_Main, serif" font-size="{fs:.3}px" font-style="normal" fill="currentColor">{c}</text>"#,
                    x = x, y = y, fs = self.font_size, c = escape_html(text)
                );
                buf.push_str(&text_elem);
            }
            MathBox::BigOp { op_box, lower, upper, width, height: _, depth: _ } => {
                // In display mode, lower is below, upper is above
                // In inline mode, they are shifted to the right (we use display mode as requested for below/above)
                let op_x = x + (width - op_box.width()) / 2.0;
                self.render_box(op_box, op_x, y, buf, current_text_style);
                
                if let Some(l) = lower {
                    let l_x = x + (width - l.width()) / 2.0;
                    let l_y = y + l.height() + self.font_size * 0.1;
                    self.render_box(l, l_x, l_y, buf, current_text_style);
                }
                
                if let Some(u) = upper {
                    let u_x = x + (width - u.width()) / 2.0;
                    let u_y = y - op_box.height() - u.depth() - self.font_size * 0.1;
                    self.render_box(u, u_x, u_y, buf, current_text_style);
                }
            }
            MathBox::Style { style, inner, width: _, height: _, depth: _ } => {
                self.render_box(inner, x, y, buf, Some(style));
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


pub fn generate_aria_label(nodes: &[swifttex_parser::ast::Node]) -> String {
    let mut label = String::new();
    for node in nodes {
        match node {
            swifttex_parser::ast::Node::Text(ch) | swifttex_parser::ast::Node::Symbol(ch) => {
                label.push(*ch);
                label.push(' ');
            }
            swifttex_parser::ast::Node::Fraction { numer, denom } => {
                label.push_str(&format!("fraction with numerator {} over {} ", generate_aria_label(&[numer.as_ref().clone()]), generate_aria_label(&[denom.as_ref().clone()])));
            }
            swifttex_parser::ast::Node::Superscript { base, exp } => {
                label.push_str(&format!("{} to the power of {} ", generate_aria_label(&[base.as_ref().clone()]), generate_aria_label(&[exp.as_ref().clone()])));
            }
            swifttex_parser::ast::Node::Subscript { base, sub } => {
                label.push_str(&format!("{} subscript {} ", generate_aria_label(&[base.as_ref().clone()]), generate_aria_label(&[sub.as_ref().clone()])));
            }
            swifttex_parser::ast::Node::SquareRoot { inner } => {
                label.push_str(&format!("square root of {} ", generate_aria_label(&[inner.as_ref().clone()])));
            }
            swifttex_parser::ast::Node::Group(children) => {
                label.push_str(&generate_aria_label(children));
            }
            swifttex_parser::ast::Node::BigOp { op, lower, upper } => {
                match op {
                    swifttex_parser::ast::BigOpKind::Sum => label.push_str("sum "),
                    swifttex_parser::ast::BigOpKind::Integral => label.push_str("integral "),
                    swifttex_parser::ast::BigOpKind::Product => label.push_str("product "),
                    swifttex_parser::ast::BigOpKind::Union => label.push_str("union "),
                    swifttex_parser::ast::BigOpKind::Intersect => label.push_str("intersection "),
                }
                if let Some(l) = lower {
                    label.push_str(&format!("from {} ", generate_aria_label(&[l.as_ref().clone()])));
                }
                if let Some(u) = upper {
                    label.push_str(&format!("to {} ", generate_aria_label(&[u.as_ref().clone()])));
                }
            }
            swifttex_parser::ast::Node::TextOp(name) => {
                label.push_str(name);
                label.push(' ');
            }
            swifttex_parser::ast::Node::Unknown(_) => {
                label.push_str("unknown command ");
            }
            swifttex_parser::ast::Node::Operator(s) => {
                label.push_str(s);
                label.push(' ');
            }
            swifttex_parser::ast::Node::Spacing(_) => {
                label.push(' ');
            }
            swifttex_parser::ast::Node::Accent { inner, .. } => {
                label.push_str(&generate_aria_label(&[inner.as_ref().clone()]));
            }
            swifttex_parser::ast::Node::Delimiter { inner, .. } => {
                label.push_str(&generate_aria_label(&[inner.as_ref().clone()]));
            }
            swifttex_parser::ast::Node::Matrix { rows, .. } => {
                label.push_str("matrix ");
                for row in rows {
                    for cell in row {
                        label.push_str(&generate_aria_label(std::slice::from_ref(cell)));
                    }
                }
            }
            swifttex_parser::ast::Node::Style { inner, .. } => {
                label.push_str(&generate_aria_label(&[inner.as_ref().clone()]));
            }
        }
    }
    label = label.replace("  ", " ").trim().to_string();
    if label.len() > 200 {
        label.truncate(197);
        label.push_str("...");
    }
    label
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
