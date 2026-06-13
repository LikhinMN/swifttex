pub mod boxes;
pub mod katex_metrics;
pub mod layout;
pub mod metrics;
pub mod style;

pub use boxes::MathBox;
pub use layout::LayoutEngine;

pub fn layout_nodes(nodes: &[swifttex_parser::ast::Node]) -> MathBox {
    let engine = LayoutEngine::new(16.0, true);
    engine.layout_nodes(nodes)
}
