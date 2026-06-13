pub mod glyph_paths;
pub mod renderer;

pub use renderer::{RenderOutput, SvgRenderer};

pub fn render(input: &str) -> Result<RenderOutput, String> {
    SvgRenderer::new(16.0, false).render(input)
}
