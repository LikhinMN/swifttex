use crate::boxes::MathBox;
use crate::metrics::glyph_metrics;
use swifttex_parser::ast::Node;

pub struct LayoutEngine {
    pub font_size: f64,
    pub display_mode: bool,
}

impl LayoutEngine {
    pub fn new(font_size: f64, display_mode: bool) -> Self {
        Self { font_size, display_mode }
    }

    pub fn layout_nodes(&self, nodes: &[Node]) -> MathBox {
        let mut children = Vec::new();
        let mut total_width = 0.0;
        let mut max_height: f64 = 0.0;
        let mut max_depth: f64 = 0.0;

        for node in nodes {
            let b = self.layout_node_with_size(node, self.font_size);
            total_width += b.width();
            if b.height() > max_height {
                max_height = b.height();
            }
            if b.depth() > max_depth {
                max_depth = b.depth();
            }
            children.push(b);
        }

        MathBox::HBox {
            children,
            width: total_width,
            height: max_height,
            depth: max_depth,
        }
    }

    #[allow(dead_code)]
    fn layout_node(&self, node: &Node) -> MathBox {
        self.layout_node_with_size(node, self.font_size)
    }

    fn layout_node_with_size(&self, node: &Node, current_font_size: f64) -> MathBox {
        match node {
            Node::Text(ch) | Node::Symbol(ch) => {
                let m = glyph_metrics(*ch);
                MathBox::Glyph {
                    ch: *ch,
                    width: m.width * current_font_size,
                    height: m.height * current_font_size,
                    depth: m.depth * current_font_size,
                }
            }
            Node::Group(children) => {
                let mut laid_children = Vec::new();
                let mut width = 0.0;
                let mut height: f64 = 0.0;
                let mut depth: f64 = 0.0;
                for c in children {
                    let b = self.layout_node_with_size(c, current_font_size);
                    width += b.width();
                    if b.height() > height { height = b.height(); }
                    if b.depth() > depth { depth = b.depth(); }
                    laid_children.push(b);
                }
                MathBox::HBox {
                    children: laid_children,
                    width,
                    height,
                    depth,
                }
            }
            Node::Fraction { numer, denom } => {
                let fs = if !self.display_mode && (current_font_size - self.font_size).abs() < 1e-6 {
                    0.8 * current_font_size
                } else {
                    current_font_size
                };
                let num_box = self.layout_node_with_size(numer, fs);
                let den_box = self.layout_node_with_size(denom, fs);

                let rule_thickness = 0.04 * current_font_size;
                let spacing = 0.1 * current_font_size;

                let max_w = num_box.width().max(den_box.width());
                let rule_box = MathBox::RuleBox {
                    width: max_w,
                    height: rule_thickness,
                    depth: 0.0,
                };

                let num_centered = MathBox::ShiftedBox {
                    inner: Box::new(num_box.clone()),
                    shift_x: (max_w - num_box.width()) / 2.0,
                    shift_y: 0.0,
                    width: max_w,
                    height: num_box.height(),
                    depth: num_box.depth(),
                };

                let den_centered = MathBox::ShiftedBox {
                    inner: Box::new(den_box.clone()),
                    shift_x: (max_w - den_box.width()) / 2.0,
                    shift_y: 0.0,
                    width: max_w,
                    height: den_box.height(),
                    depth: den_box.depth(),
                };

                let children = vec![
                    num_centered,
                    MathBox::Glue { width: spacing }, // height 0
                    rule_box,
                    MathBox::Glue { width: spacing }, // height 0
                    den_centered,
                ];

                let total_height_stack = num_box.height() + num_box.depth()
                    + rule_thickness + den_box.height() + den_box.depth() + 2.0 * spacing;
                
                let axis_height = 0.25 * current_font_size;
                let fraction_height = num_box.height() + num_box.depth() + spacing + rule_thickness / 2.0 + axis_height;
                let fraction_depth = total_height_stack - fraction_height;

                MathBox::VBox {
                    children,
                    width: max_w,
                    height: fraction_height,
                    depth: fraction_depth,
                }
            }
            Node::Superscript { base, exp } => {
                let base_box = self.layout_node_with_size(base, current_font_size);
                let exp_box = self.layout_node_with_size(exp, current_font_size * 0.7);
                let script_rise = base_box.height() * 0.6;
                let shifted_exp = MathBox::ShiftedBox {
                    width: exp_box.width(),
                    height: exp_box.height() + script_rise,
                    depth: (exp_box.depth() - script_rise).max(0.0),
                    shift_x: 0.0,
                    shift_y: script_rise,
                    inner: Box::new(exp_box.clone()),
                };
                let w = base_box.width() + shifted_exp.width();
                let h = base_box.height().max(shifted_exp.height());
                let d = base_box.depth().max(shifted_exp.depth());
                MathBox::HBox {
                    children: vec![base_box, shifted_exp],
                    width: w,
                    height: h,
                    depth: d,
                }
            }
            Node::Subscript { base, sub } => {
                let base_box = self.layout_node_with_size(base, current_font_size);
                let sub_box = self.layout_node_with_size(sub, current_font_size * 0.7);
                let script_drop = base_box.depth() + 0.2 * current_font_size;
                let shifted_sub = MathBox::ShiftedBox {
                    width: sub_box.width(),
                    height: (sub_box.height() - script_drop).max(0.0),
                    depth: sub_box.depth() + script_drop,
                    shift_x: 0.0,
                    shift_y: -script_drop,
                    inner: Box::new(sub_box.clone()),
                };
                let w = base_box.width() + shifted_sub.width();
                let h = base_box.height().max(shifted_sub.height());
                let d = base_box.depth().max(shifted_sub.depth());
                MathBox::HBox {
                    children: vec![base_box, shifted_sub],
                    width: w,
                    height: h,
                    depth: d,
                }
            }
            Node::SquareRoot { inner } => {
                let inner_box = self.layout_node_with_size(inner, current_font_size);
                let radical_w = 0.4 * current_font_size;
                let radical = MathBox::RuleBox {
                    width: radical_w,
                    height: inner_box.height() + inner_box.depth(),
                    depth: 0.0,
                };
                let clearance = 0.15 * current_font_size;
                let rule_thickness = 0.04 * current_font_size;
                
                let overline = MathBox::RuleBox {
                    width: inner_box.width(),
                    height: rule_thickness,
                    depth: 0.0,
                };
                
                let inner_vbox = MathBox::VBox {
                    children: vec![
                        overline,
                        MathBox::Glue { width: clearance },
                        inner_box.clone()
                    ],
                    width: inner_box.width(),
                    height: inner_box.height() + clearance + rule_thickness,
                    depth: inner_box.depth(),
                };

                let w = radical.width() + inner_vbox.width();
                let h = inner_vbox.height();
                let d = inner_vbox.depth();
                MathBox::HBox {
                    children: vec![radical, inner_vbox],
                    width: w,
                    height: h,
                    depth: d,
                }
            }
            Node::Operator(s) => {
                let mut laid_children = Vec::new();
                let mut width = 0.0;
                let mut height: f64 = 0.0;
                let mut depth: f64 = 0.0;
                for ch in s.chars() {
                    let m = glyph_metrics(ch);
                    let b = MathBox::Glyph {
                        ch,
                        width: m.width * current_font_size,
                        height: m.height * current_font_size,
                        depth: m.depth * current_font_size,
                    };
                    width += b.width();
                    if b.height() > height { height = b.height(); }
                    if b.depth() > depth { depth = b.depth(); }
                    laid_children.push(b);
                }
                MathBox::HBox {
                    children: laid_children,
                    width,
                    height,
                    depth,
                }
            }
            Node::Unknown(_) => {
                MathBox::Glue { width: 0.0 }
            }
        }
    }
}
