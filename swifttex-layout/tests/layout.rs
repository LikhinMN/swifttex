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

#[test]
fn test_katex_metrics_differ_from_old() {
    let m = swifttex_layout::metrics::glyph_metrics('x');
    assert!((m.width - 0.55).abs() > 0.001);
}

#[test]
fn test_katex_font_styles_differ() {
    let m_math = swifttex_layout::metrics::glyph_metrics('x');
    let m_main = swifttex_layout::metrics::glyph_metrics('1');
    assert!((m_math.width - m_main.width).abs() > 0.001);
}

#[test]
fn test_matrix_layout() {
    let nodes = swifttex_parser::Parser::new(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}").parse();
    let layout = swifttex_layout::layout_nodes(&nodes);
    let inner = match &layout {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    match inner {
        swifttex_layout::MathBox::Matrix { col_widths, row_heights, total_width, .. } => {
            assert_eq!(col_widths.len(), 2);
            assert_eq!(row_heights.len(), 2);
            let single_col_width = col_widths[0];
            assert!(*total_width > single_col_width);
        }
        _ => panic!("Expected MatrixBox"),
    }
}

#[test]
fn test_big_op_layout() {
    let nodes_display = swifttex_parser::Parser::new(r"\sum_{i=0}^{n}").parse();
    let layout_display = swifttex_layout::LayoutEngine::new(16.0, true).layout_nodes(&nodes_display);
    
    let nodes_inline = swifttex_parser::Parser::new(r"\sum_{i=0}^{n}").parse();
    let layout_inline = swifttex_layout::LayoutEngine::new(16.0, false).layout_nodes(&nodes_inline);
    
    let inner_d = match &layout_display {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    let inner_i = match &layout_inline {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    
    match (inner_d, inner_i) {
        (swifttex_layout::MathBox::BigOp { op_box: op_d, .. }, swifttex_layout::MathBox::BigOp { op_box: op_i, .. }) => {
            assert!(op_d.height() > op_i.height());
        }
        _ => panic!("Expected BigOp boxes"),
    }
}


#[test]
fn test_style_cascade_fraction() {
    let nodes = parse(r"\frac{a}{b}");
    let engine = LayoutEngine::new(16.0, true);
    let layout = engine.layout_nodes(&nodes);
    let inner = match &layout {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    match inner {
        swifttex_layout::MathBox::VBox { children, .. } => {
            let num = match &children[0] {
                swifttex_layout::MathBox::ShiftedBox { inner, .. } => inner,
                _ => panic!("Expected ShiftedBox"),
            };
            let num_h_box = match &**num {
                swifttex_layout::MathBox::HBox { children, .. } => &children[0],
                _ => panic!("Expected HBox in numerator"),
            };
            let height = num_h_box.height();
            assert!(height > 5.0 && height < 16.0);
        }
        _ => panic!("Expected VBox"),
    }
}

#[test]
fn test_superscript_display() {
    let nodes = parse("x^2");
    let layout = LayoutEngine::new(16.0, true).layout_nodes(&nodes);
    let inner = match &layout {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    match inner {
        swifttex_layout::MathBox::HBox { children, .. } => {
            let exp = match &children[1] {
                swifttex_layout::MathBox::ShiftedBox { inner, .. } => inner,
                _ => panic!("Expected ShiftedBox"),
            };
            let exp_height = match &**exp {
                swifttex_layout::MathBox::Glyph { height, .. } => *height,
                _ => panic!("Expected Glyph for exp"),
            };
            assert!(exp_height < 10.0);
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_text_op_layout() {
    let nodes = parse(r"\sin x");
    let layout = LayoutEngine::new(16.0, true).layout_nodes(&nodes);
    let inner = match &layout {
        swifttex_layout::MathBox::HBox { children, .. } => &children[0],
        _ => panic!("Expected HBox"),
    };
    match inner {
        swifttex_layout::MathBox::HBox { children, .. } => {
            match &children[0] {
                swifttex_layout::MathBox::TextOp { text, .. } => assert_eq!(text, "sin"),
                _ => panic!("Expected TextOp box"),
            }
        }
        _ => panic!("Expected HBox"),
    }
}

#[test]
fn test_spacing_layout() {
    let nodes = parse(r"x \quad y");
    let layout = LayoutEngine::new(16.0, true).layout_nodes(&nodes);
    let inner = match &layout {
        swifttex_layout::MathBox::HBox { children, .. } => children,
        _ => panic!("Expected HBox"),
    };
    match &inner[1] {
        swifttex_layout::MathBox::Glue { width } => assert!((*width - 16.0).abs() < 1e-6),
        _ => panic!("Expected Glue"),
    }
}
