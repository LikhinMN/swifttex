use crate::glyph_paths::{glyph_path_or_text, GlyphRender};
use swifttex_layout::{LayoutEngine, MathBox};
use swifttex_parser::Parser;
use swifttex_layout::metrics::glyph_metrics;

pub struct SvgRenderer {
    pub font_size: f64,
    pub display_mode: bool,
}

pub struct RenderOutput {
    pub svg: String,
    pub width: f64,
    pub height: f64,
}

impl SvgRenderer {
    pub fn new(font_size: f64, display_mode: bool) -> Self {
        Self { font_size, display_mode }
    }

    pub fn render(&self, input: &str) -> Result<RenderOutput, String> {
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        
        let engine = LayoutEngine::new(self.font_size, self.display_mode);
        let root_box = engine.layout_nodes(&nodes);
        
        let mut buf = String::new();
        let total_width = root_box.width();
        let total_height = root_box.height() + root_box.depth();
        
        let start_x = 0.0;
        let start_y = root_box.height();
        
        self.render_box(&root_box, start_x, start_y, &mut buf);
        
        let escaped_input = escape_html(input);
        
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w:.3}px" height="{h:.3}px" viewBox="0 0 {w:.3} {h:.3}" role="img" aria-label="{aria}">
  <title>{title}</title>
  {inner}
</svg>"#,
            w = f64::max(total_width, 0.1),
            h = f64::max(total_height, 0.1),
            aria = escaped_input,
            title = escaped_input,
            inner = buf
        );
        
        Ok(RenderOutput {
            svg,
            width: total_width,
            height: total_height,
        })
    }

    fn render_box(&self, mb: &MathBox, x: f64, y: f64, buf: &mut String) {
        match mb {
            MathBox::Glyph { ch, width, height: _, depth: _ } => {
                let m = glyph_metrics(*ch);
                let local_font_size = if m.width > 0.0 {
                    width / m.width
                } else {
                    self.font_size
                };
                
                match glyph_path_or_text(*ch) {
                    GlyphRender::Path(d) => {
                        let scale = local_font_size / 1000.0;
                        buf.push_str(&format!(
                            r#"<path d="{d}" transform="translate({x:.3},{y:.3}) scale({scale:.3})"/>"#,
                            d = d,
                            x = x,
                            y = y,
                            scale = scale
                        ));
                    }
                    GlyphRender::TextFallback(c) => {
                        buf.push_str(&format!(
                            r#"<text x="{x:.3}" y="{y:.3}" font-family="KaTeX_Math,serif" font-size="{fs:.3}px" fill="currentColor">{c}</text>"#,
                            x = x,
                            y = y,
                            fs = local_font_size,
                            c = escape_char(c)
                        ));
                    }
                }
            }
            MathBox::HBox { children, .. } => {
                let mut cursor_x = x;
                for child in children {
                    self.render_box(child, cursor_x, y, buf);
                    cursor_x += child.width();
                }
            }
            MathBox::VBox { children, height, .. } => {
                let mut cursor_y = y - height;
                for child in children {
                    cursor_y += child.height();
                    self.render_box(child, x, cursor_y, buf);
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
                self.render_box(inner, x + shift_x, y - shift_y, buf);
            }
            MathBox::Glue { .. } => {}
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
