use swifttex_layout::{layout_nodes, LayoutEngine, MathBox};
use swifttex_parser::Parser;

fn parse(input: &str) -> Vec<swifttex_parser::ast::Node> {
    Parser::new(input).parse()
}

#[test]
fn test_single_char() {
    let nodes = parse("x");
    let layout = layout_nodes(&nodes);
    assert!(layout.width() > 0.0);
    assert!(layout.height() > 0.0);
    match layout {
        MathBox::HBox { children, .. } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                MathBox::Glyph { ch, .. } => assert_eq!(*ch, 'x'),
                _ => panic!("Expected Glyph"),
            }
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_superscript() {
    let nodes = parse("x^2");
    let layout = layout_nodes(&nodes);
    
    let base_width = match &nodes[0] {
        swifttex_parser::ast::Node::Superscript { base, .. } => {
            let base_layout = LayoutEngine::new(16.0, true).layout_nodes(&[*(base.clone())]);
            base_layout.width()
        }
        _ => panic!("Expected Superscript"),
    };
    
    assert!(layout.width() > base_width);
}

#[test]
fn test_fraction() {
    let nodes = parse(r"\frac{1}{2}");
    let layout = layout_nodes(&nodes);
    
    match &layout {
        MathBox::HBox { children, .. } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                MathBox::VBox { height, depth, .. } => {
                    assert!(*height > 0.0);
                    assert!(*depth > 0.0);
                }
                _ => panic!("Expected VBox"),
            }
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_sqrt() {
    let nodes = parse(r"\sqrt{x}");
    let layout = layout_nodes(&nodes);
    match &layout {
        MathBox::HBox { children, .. } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                MathBox::HBox { children: inner_children, height, .. } => {
                    assert_eq!(inner_children.len(), 2);
                    let inner_node = match &nodes[0] {
                        swifttex_parser::ast::Node::SquareRoot { inner } => *(inner.clone()),
                        _ => panic!("Expected SquareRoot"),
                    };
                    let inner_layout = LayoutEngine::new(16.0, true).layout_nodes(&[inner_node]);
                    assert!(*height > inner_layout.height());
                }
                _ => panic!("Expected HBox"),
            }
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_group() {
    let nodes = parse("{ab}");
    let layout = layout_nodes(&nodes);
    match &layout {
        MathBox::HBox { children, width, .. } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                MathBox::HBox { children: group_children, .. } => {
                    assert_eq!(group_children.len(), 2);
                    let mut sum_width = 0.0;
                    for c in group_children {
                        sum_width += c.width();
                    }
                    assert!((*width - sum_width).abs() < 1e-6);
                }
                _ => panic!("Expected HBox for group"),
            }
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_display_mode() {
    let nodes = parse(r"\frac{1}{2}");
    let layout_display = LayoutEngine::new(16.0, true).layout_nodes(&nodes);
    let layout_inline = LayoutEngine::new(16.0, false).layout_nodes(&nodes);
    
    assert!(layout_display.height() > layout_inline.height());
}

#[test]
fn test_unknown_node() {
    let nodes = parse(r"\foo");
    let layout = layout_nodes(&nodes);
    assert!(layout.width() < 1e-6);
}

#[test]
fn test_empty_node_list() {
    let nodes = parse("");
    let layout = layout_nodes(&nodes);
    assert!(layout.width() < 1e-6);
    assert!(layout.height() < 1e-6);
    assert!(layout.depth() < 1e-6);
}
