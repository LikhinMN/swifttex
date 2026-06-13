pub enum GlyphRender {
    Text(char),
}

pub fn glyph_path_or_text(ch: char) -> GlyphRender {
    GlyphRender::Text(ch)
}
