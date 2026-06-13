pub mod glyph_paths;
pub mod renderer;

pub use renderer::{RenderOutput, SvgRenderer};

pub fn render(input: &str) -> Result<RenderOutput, String> {
    SvgRenderer::new(16.0, true, true).render(input)
}

pub fn render_with_style(input: &str, style: swifttex_layout::style::MathStyle) -> Result<RenderOutput, String> {
    SvgRenderer::new(16.0, true, true).with_math_style(style).render(input)
}
