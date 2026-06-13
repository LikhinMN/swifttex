pub enum GlyphRender {
    Path(&'static str),
    TextFallback(char),
}

pub fn glyph_path(ch: char) -> Option<&'static str> {
    match ch {
        '0' => Some("M 50 50 L 500 50 L 500 600 L 50 600 Z"),
        '1' => Some("M 250 50 L 250 600"),
        '2' => Some("M 50 50 L 500 50 L 500 300 L 50 300 L 50 600 L 500 600"),
        '3' => Some("M 50 50 L 500 50 L 500 300 L 50 300 M 500 300 L 500 600 L 50 600"),
        '4' => Some("M 50 50 L 50 300 L 500 300 M 500 50 L 500 600"),
        '5' => Some("M 500 50 L 50 50 L 50 300 L 500 300 L 500 600 L 50 600"),
        '6' => Some("M 500 50 L 50 50 L 50 600 L 500 600 L 500 300 L 50 300"),
        '7' => Some("M 50 50 L 500 50 L 500 600"),
        '8' => Some("M 50 50 L 500 50 L 500 600 L 50 600 Z M 50 300 L 500 300"),
        '9' => Some("M 500 600 L 500 50 L 50 50 L 50 300 L 500 300"),
        '+' => Some("M 250 100 L 300 100 L 300 250 L 450 250 L 450 300 L 300 300 L 300 450 L 250 450 L 250 300 L 100 300 L 100 250 L 250 250 Z"),
        '-' => Some("M 100 250 L 450 250 L 450 300 L 100 300 Z"),
        '=' => Some("M 100 200 L 450 200 L 450 250 L 100 250 Z M 100 350 L 450 350 L 450 400 L 100 400 Z"),
        '·' => Some("M 250 250 L 300 250 L 300 300 L 250 300 Z"),
        _ => None,
    }
}

pub fn glyph_path_or_text(ch: char) -> GlyphRender {
    match glyph_path(ch) {
        Some(path) => GlyphRender::Path(path),
        None => GlyphRender::TextFallback(ch),
    }
}
