//! Converts a parsed `Node` tree into a `MathBox` layout tree.
//!
//! Uses a TeX-inspired box model with KaTeX font metrics.
//! Supports math style cascade (Display/Text/Script/ScriptScript).

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
