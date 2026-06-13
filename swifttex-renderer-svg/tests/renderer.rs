use swifttex_renderer_svg::{render, SvgRenderer};

fn print_snapshot(name: &str, svg: &str) {
    eprintln!("--- SNAPSHOT: {} ---\n{}\n--------------------", name, svg);
}

#[test]
fn test_render_single_char() {
    let out = render("x").expect("failed to render");
    print_snapshot("test_render_single_char", &out.svg);
    assert!(out.svg.contains("<text"));
    assert!(out.svg.contains("KaTeX"));
    assert!(out.width > 0.0);
}

#[test]
fn test_render_superscript() {
    let out = render("x^2").expect("failed to render");
    let out_single = render("x").unwrap();
    print_snapshot("test_render_superscript", &out.svg);
    assert!(out.svg.contains("<svg"));
    assert!(out.svg.contains("KaTeX_Main")); // The '2'
    assert!(out.svg.contains("KaTeX_Math")); // The 'x'
    assert!(out.width > out_single.width);
}

#[test]
fn test_render_fraction() {
    let out = render(r"\frac{1}{2}").expect("failed to render");
    print_snapshot("test_render_fraction", &out.svg);
    assert!(out.svg.contains("KaTeX_Main"));
    assert!(out.svg.contains("<rect")); // Fraction line
    assert!(out.height > out.width);
}

#[test]
fn test_render_sqrt() {
    let out = render(r"\sqrt{x}").expect("failed to render");
    print_snapshot("test_render_sqrt", &out.svg);
    assert!(!out.svg.is_empty());
}

#[test]
fn test_render_alpha() {
    let out = render(r"\alpha").expect("failed to render");
    print_snapshot("test_render_alpha", &out.svg);
    assert!(out.svg.contains("KaTeX_Math"));
    assert!(out.svg.contains("<svg"));
}

#[test]
fn test_inline_fonts_true() {
    let renderer = SvgRenderer::new(16.0, false, true);
    let out = renderer.render("x").unwrap();
    assert!(out.svg.contains("@import url"));
}

#[test]
fn test_inline_fonts_false() {
    let renderer = SvgRenderer::new(16.0, false, false);
    let out = renderer.render("x").unwrap();
    assert!(!out.svg.contains("@import url"));
}

#[test]
fn test_render_empty() {
    let out = render("").expect("failed to render");
    print_snapshot("test_render_empty", &out.svg);
    assert!(out.width < 1.0);
    assert!(out.height < 1.0);
}

#[test]
fn test_render_complex() {
    let out = render(r"\frac{\alpha^2}{\beta_1}").expect("failed to render");
    print_snapshot("test_render_complex", &out.svg);
    assert!(out.svg.contains("<svg"));
}

#[test]
fn test_svg_contains_xmlns() {
    let out = render("x").expect("failed to render");
    assert!(out.svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
}

#[test]
fn test_svg_contains_title() {
    let out = render("x").expect("failed to render");
    assert!(out.svg.contains("<title>x</title>"));
}

#[test]
fn test_render_deterministic() {
    let input = r"\frac{\alpha^2}{\beta_1}";
    let first_out = render(input).expect("failed to render");
    for _ in 0..100 {
        let out = render(input).unwrap();
        assert_eq!(first_out.svg, out.svg);
    }
}

#[test]
fn test_performance_simple() {
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..1000 {
        render("x^2").unwrap();
    }
    let avg = start.elapsed().as_micros() / 1000;
    eprintln!("simple avg: {}µs", avg);
    if avg > 1000 {
        eprintln!("WARNING: simple render exceeded 1ms target ({}µs)", avg);
    }
}

#[test]
fn test_performance_complex() {
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..100 {
        render(r"\frac{\alpha^2 + \sqrt{\beta}}{\gamma_{n+1}}").unwrap();
    }
    let avg = start.elapsed().as_micros() / 100;
    eprintln!("complex avg: {}µs", avg);
    if avg > 10000 {
        eprintln!("WARNING: complex render exceeded 10ms target ({}µs)", avg);
    }
}

#[test]
fn test_render_matrix() {
    let out = render(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}").expect("failed to render");
    assert!(out.svg.contains("<svg"));
    assert!(out.svg.contains("("));
    assert!(out.svg.contains(")"));
    assert!(out.svg.contains("KaTeX_"));
}

#[test]
fn test_render_left_right() {
    let out = render(r"\left( \frac{\frac{x}{z}}{y} \right)").expect("failed to render");
    assert!(out.svg.contains("<svg"));
    assert!(out.svg.contains("("));
    assert!(out.svg.contains(")"));
    assert!(out.svg.contains("KaTeX_Size"));
}

#[test]
fn test_render_big_op_sum() {
    let out = render(r"\sum_{i=0}^{n} x_i").expect("failed to render");
    assert!(out.svg.contains("<svg"));
    assert!(out.svg.contains("∑"));
}

#[test]
fn test_render_big_op_int() {
    let out = render(r"\int_0^\infty f(x)").expect("failed to render");
    assert!(out.svg.contains("<svg"));
    assert!(out.svg.contains("∫"));
}


#[test]
fn test_render_sin() {
    let out = render(r"\sin(x)").expect("failed to render");
    assert!(out.svg.contains("KaTeX_Main"));
}

#[test]
fn test_render_hat() {
    let out = render(r"\hat{x}").expect("failed to render");
    assert!(out.svg.contains("<svg"));
}

#[test]
fn test_render_lim() {
    let out = render(r"\lim_{x \to 0} f(x)").expect("failed to render");
    assert!(out.svg.contains("<svg"));
}

#[test]
fn test_render_leq() {
    let out = render(r"x \leq y").expect("failed to render");
    assert!(out.svg.contains("≤"));
}

#[test]
fn test_render_forall() {
    let out = render(r"\forall x \in \mathbb{R}").unwrap_or_else(|_| render(r"\forall x \in R").unwrap());
    assert!(out.svg.contains("<svg"));
}

#[test]
fn test_render_math_style_script() {
    let out_display = swifttex_renderer_svg::render_with_style(r"x", swifttex_layout::style::MathStyle::Display).unwrap();
    let out_script = swifttex_renderer_svg::render_with_style(r"x", swifttex_layout::style::MathStyle::Script).unwrap();
    assert!(out_display.width > out_script.width);
}

#[test]
fn test_render_spacing() {
    let out_no_space = render(r"xy").unwrap();
    let out_space = render(r"x \, y").unwrap();
    assert!(out_space.width > out_no_space.width);
}

#[test]
fn test_aria_label_superscript() {
    let out = swifttex_renderer_svg::render_accessible("x^2", None).unwrap();
    assert!(out.aria_label.contains("to the power of"));
}

#[test]
fn test_aria_label_fraction() {
    let out = swifttex_renderer_svg::render_accessible(r"\frac{1}{2}", None).unwrap();
    assert!(out.aria_label.contains("fraction"));
}

#[test]
fn test_aria_label_sqrt() {
    let out = swifttex_renderer_svg::render_accessible(r"\sqrt{x}", None).unwrap();
    assert!(out.aria_label.contains("square root"));
}

use swifttex_plugin_api::{PluginRegistry, Plugin, CommandHandler, Node, SymbolHandler};
use std::sync::{Arc, Mutex};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn name(&self) -> &str { "physics" }
    fn symbols(&self) -> Vec<Box<dyn SymbolHandler>> {
        struct HbarSymbol;
        impl SymbolHandler for HbarSymbol {
            fn name(&self) -> &str { "hbar" }
            fn resolve(&self) -> char { 'ℏ' }
        }
        vec![Box::new(HbarSymbol)]
    }
    fn commands(&self) -> Vec<Box<dyn CommandHandler>> {
        struct BraCommand;
        impl CommandHandler for BraCommand {
            fn name(&self) -> &str { "bra" }
            fn arity(&self) -> usize { 1 }
            fn expand(&self, args: Vec<Vec<Node>>) -> Node {
                let inner = args.into_iter().next().unwrap_or_default();
                Node::Group(vec![Node::Symbol('⟨'), Node::Group(inner), Node::Symbol('|')])
            }
        }
        vec![Box::new(BraCommand)]
    }
}

#[test]
fn test_svg_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(PhysicsPlugin));
    let renderer = swifttex_renderer_svg::SvgRenderer::new(16.0, false, false)
        .with_registry(Arc::new(Mutex::new(registry)));
    
    let out = renderer.render(r"\bra{x}").unwrap();
    assert!(out.svg.contains("⟨"));
    assert!(out.svg.contains("|"));
}

#[test]
fn test_svg_no_plugin() {
    let renderer = swifttex_renderer_svg::SvgRenderer::new(16.0, false, false);
    let out = renderer.render(r"\hbar").unwrap();
    // unknown fallback logic
    assert!(out.svg.contains("hbar"));
}
