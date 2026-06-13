pub struct GlyphMetrics {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub italic_correction: f64,
}

use crate::katex_metrics::{katex_glyph_metrics, FontStyle};

pub fn glyph_metrics(ch: char) -> GlyphMetrics {
    let style = if ch.is_ascii_alphabetic() || ('α'..='ω').contains(&ch) || ('Α'..='Ω').contains(&ch) {
        FontStyle::MathItalic
    } else {
        FontStyle::MainRegular
    };

    if let Some(km) = katex_glyph_metrics(ch, style) {
        return GlyphMetrics {
            width: km.width,
            height: km.height,
            depth: km.depth,
            italic_correction: km.italic_correction,
        };
    }

    match ch {
        'a'..='z' => GlyphMetrics { width: 0.55, height: 0.45, depth: 0.0, italic_correction: 0.02 },
        'A'..='Z' => GlyphMetrics { width: 0.70, height: 0.65, depth: 0.0, italic_correction: 0.02 },
        '0'..='9' => GlyphMetrics { width: 0.55, height: 0.65, depth: 0.0, italic_correction: 0.0 },
        'α'..='ω' => GlyphMetrics { width: 0.58, height: 0.48, depth: 0.02, italic_correction: 0.03 },
        'Α'..='Ω' => GlyphMetrics { width: 0.72, height: 0.65, depth: 0.0, italic_correction: 0.02 },
        '+' | '-' | '=' => GlyphMetrics { width: 0.77, height: 0.28, depth: 0.08, italic_correction: 0.0 },
        _ => GlyphMetrics { width: 0.55, height: 0.45, depth: 0.0, italic_correction: 0.0 },
    }
}
