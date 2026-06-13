pub mod glyph_paths;
pub mod renderer;

pub use renderer::{RenderOutput, SvgRenderer};

pub fn render(input: &str) -> Result<RenderOutput, String> {
    SvgRenderer::new(16.0, true, true).render(input)
}

pub fn render_with_style(input: &str, style: swifttex_layout::style::MathStyle) -> Result<RenderOutput, String> {
    SvgRenderer::new(16.0, true, true).with_math_style(style).render(input)
}

pub struct AccessibleOutput {
    pub svg: String,
    pub mathml: String,
    pub width: f64,
    pub height: f64,
    pub aria_label: String,
}

pub fn render_accessible(input: &str) -> Result<AccessibleOutput, String> {
    let nodes = swifttex_parser::parse_to_nodes(input);
    
    let svg_renderer = SvgRenderer::new(16.0, true, true);
    let svg_out = svg_renderer.render_from_nodes(input, &nodes)?;
    
    let mathml_renderer = swifttex_renderer_mathml::MathMLRenderer::new(true);
    let mathml_out = mathml_renderer.render_from_nodes(input, &nodes)?;
    
    let aria_label = renderer::generate_aria_label(&nodes);
    
    Ok(AccessibleOutput {
        svg: svg_out.svg,
        mathml: mathml_out.mathml,
        width: svg_out.width,
        height: svg_out.height,
        aria_label,
    })
}
