use crate::boxes::MathBox;
use crate::metrics::glyph_metrics;
use swifttex_parser::ast::Node;

use crate::style::MathStyle;


pub struct LayoutEngine {
    pub font_size: f64,
    pub style: MathStyle,
    pub display_mode: bool,
}

impl LayoutEngine {
    pub fn new(font_size: f64, display_mode: bool) -> Self {
        Self {
            font_size,
            style: if display_mode { MathStyle::Display } else { MathStyle::Text },
            display_mode,
        }
    }

    pub fn with_style(&self, style: MathStyle) -> Self {
        Self {
            font_size: self.font_size * style.font_size_factor() / self.style.font_size_factor(),
            style,
            display_mode: self.display_mode,
        }
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
                let num_box = self.with_style(self.style.numerator_style()).layout_node_with_size(numer, self.with_style(self.style.numerator_style()).font_size);
                let den_box = self.with_style(self.style.denominator_style()).layout_node_with_size(denom, self.with_style(self.style.denominator_style()).font_size);

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
                let exp_engine = self.with_style(self.style.superscript_style());
                let exp_box = exp_engine.layout_node_with_size(exp, exp_engine.font_size);
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
                let sub_engine = self.with_style(self.style.subscript_style());
                let sub_box = sub_engine.layout_node_with_size(sub, sub_engine.font_size);
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
                let inner_engine = self.with_style(self.style.cramped());
                let inner_box = inner_engine.layout_node_with_size(inner, inner_engine.font_size);
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
            Node::Matrix { rows, env } => {
                let mut col_widths: Vec<f64> = Vec::new();
                let mut row_heights: Vec<f64> = Vec::new();
                let mut laid_out_cells = Vec::new();

                for row in rows {
                    let mut layout_row = Vec::new();
                    let mut max_h = 0.0_f64;
                    for (c_idx, cell) in row.iter().enumerate() {
                        let cell_box = self.layout_nodes(std::slice::from_ref(cell));
                        let w = cell_box.width();
                        let h = cell_box.height() + cell_box.depth();
                        if c_idx >= col_widths.len() {
                            col_widths.push(w);
                        } else if w > col_widths[c_idx] {
                            col_widths[c_idx] = w;
                        }
                        if h > max_h {
                            max_h = h;
                        }
                        layout_row.push(cell_box);
                    }
                    row_heights.push(max_h);
                    laid_out_cells.push(layout_row);
                }

                let col_spacing = self.font_size * 0.5;
                let row_spacing = self.font_size * 0.3;
                let total_width = col_widths.iter().sum::<f64>() + col_spacing * (col_widths.len().saturating_sub(1) as f64);
                let total_height = row_heights.iter().sum::<f64>() + row_spacing * (row_heights.len().saturating_sub(1) as f64);

                MathBox::Matrix {
                    cells: laid_out_cells,
                    col_widths,
                    row_heights,
                    total_width,
                    total_height,
                    env: env.clone(),
                }
            }
            Node::Delimiter { open, close, inner, sizing } => {
                let inner_box = self.layout_nodes(std::slice::from_ref(inner));
                let inner_height = inner_box.height() + inner_box.depth();
                let delim_height = match sizing {
                    swifttex_parser::ast::DelimSizing::Auto => inner_height,
                    swifttex_parser::ast::DelimSizing::Fixed(s) => self.font_size * (*s as f64) * 0.8,
                };
                
                // Assume each delimiter takes some width based on height
                // Just an approximation here, exact metrics handled in renderer
                let delim_w = if *open != swifttex_parser::ast::DelimChar::None { self.font_size * 0.4 } else { 0.0 };
                let delim_w2 = if *close != swifttex_parser::ast::DelimChar::None { self.font_size * 0.4 } else { 0.0 };
                
                MathBox::Delim {
                    open: open.clone(),
                    close: close.clone(),
                    height: inner_box.height(),
                    depth: inner_box.depth(),
                    width: delim_w + inner_box.width() + delim_w2,
                    delim_height,
                    inner: Box::new(inner_box),
                }
            }
            Node::BigOp { op, lower, upper } => {
                let op_char = match op {
                    swifttex_parser::ast::BigOpKind::Sum => '∑',
                    swifttex_parser::ast::BigOpKind::Integral => '∫',
                    swifttex_parser::ast::BigOpKind::Product => '∏',
                    swifttex_parser::ast::BigOpKind::Union => '⋃',
                    swifttex_parser::ast::BigOpKind::Intersect => '⋂',
                };
                let op_fs = if self.display_mode { self.font_size * 1.2 } else { self.font_size };
                let op_engine = LayoutEngine::new(op_fs, self.display_mode);
                let op_box = op_engine.layout_node_with_size(&Node::Symbol(op_char), op_fs);
                
                let mut lower_box = None;
                if let Some(l) = lower {
                    let lower_engine = self.with_style(MathStyle::Script);
                    lower_box = Some(Box::new(lower_engine.layout_nodes(std::slice::from_ref(l))));
                }
                let mut upper_box = None;
                if let Some(u) = upper {
                    let upper_engine = self.with_style(MathStyle::Script);
                    upper_box = Some(Box::new(upper_engine.layout_nodes(std::slice::from_ref(u))));
                }
                
                // In a real layout engine, we'd place them accurately. 
                // We'll let the MathBox store them and Renderer put them in place.
                let mut w = op_box.width();
                if let Some(ref l) = lower_box { w = w.max(l.width()); }
                if let Some(ref u) = upper_box { w = w.max(u.width()); }
                
                let mut h = op_box.height();
                if let Some(ref u) = upper_box { h += u.height() + u.depth() + self.font_size * 0.1; }
                
                let mut d = op_box.depth();
                if let Some(ref l) = lower_box { d += l.height() + l.depth() + self.font_size * 0.1; }

                MathBox::BigOp {
                    op_box: Box::new(op_box),
                    lower: lower_box,
                    upper: upper_box,
                    width: w,
                    height: h,
                    depth: d,
                }
            }

            Node::Accent { kind, inner } => {
                let inner_box = self.with_style(self.style.cramped()).layout_node_with_size(inner, self.with_style(self.style.cramped()).font_size);
                
                let accent_char = match kind {
                    swifttex_parser::ast::AccentKind::Hat => '^',
                    swifttex_parser::ast::AccentKind::Bar => '‾',
                    swifttex_parser::ast::AccentKind::Vec => '⃗',
                    swifttex_parser::ast::AccentKind::Dot => '˙',
                    swifttex_parser::ast::AccentKind::DDot => '¨',
                    swifttex_parser::ast::AccentKind::Tilde => '˜',
                };
                
                let accent_glyph = MathBox::Glyph {
                    ch: accent_char,
                    width: inner_box.width(), // we will stretch it or center it in renderer
                    height: current_font_size * 0.2, // approximation
                    depth: 0.0,
                };
                
                let clearance = 0.1 * current_font_size;
                
                let w = inner_box.width();
                let h = inner_box.height() + clearance + accent_glyph.height();
                let d = inner_box.depth();
                
                MathBox::VBox {
                    children: vec![accent_glyph, MathBox::Glue { width: clearance }, inner_box],
                    width: w,
                    height: h,
                    depth: d,
                }
            }
            Node::TextOp(name) => {
                let mut width = 0.0;
                let mut height: f64 = 0.0;
                let mut depth: f64 = 0.0;
                for ch in name.chars() {
                    let m = glyph_metrics(ch);
                    width += m.width * current_font_size;
                    let h = m.height * current_font_size;
                    let d = m.depth * current_font_size;
                    if h > height { height = h; }
                    if d > depth { depth = d; }
                }
                let spacing = 0.167 * current_font_size;
                
                MathBox::HBox {
                    width: width + spacing,
                    height,
                    depth,
                    children: vec![
                        MathBox::TextOp {
                            text: name.clone(),
                            width,
                            height,
                            depth,
                        },
                        MathBox::Glue { width: spacing },
                    ],
                }
            }
            Node::Spacing(em) => {
                MathBox::Glue { width: em * current_font_size }
            }

            Node::Unknown(_) => {
                MathBox::Glue { width: 0.0 }
            }
            Node::Style { style, inner } => {
                let inner_box = self.layout_node_with_size(inner, current_font_size);
                MathBox::Style {
                    style: style.clone(),
                    width: inner_box.width(),
                    height: inner_box.height(),
                    depth: inner_box.depth(),
                    inner: Box::new(inner_box),
                }
            }
        }
    }
}
