use swifttex_renderer_svg::render;

fn print_snapshot(name: &str, svg: &str) {
    eprintln!("--- SNAPSHOT: {} ---\n{}\n--------------------", name, svg);
}

#[test]
fn test_render_single_char() {
    let out = render("x").expect("failed to render");
    print_snapshot("test_render_single_char", &out.svg);
    assert!(out.svg.contains("<text") || out.svg.contains("<path"));
    assert!(out.width > 0.0);
}

#[test]
fn test_render_superscript() {
    let out = render("x^2").expect("failed to render");
    let out_single = render("x").unwrap();
    print_snapshot("test_render_superscript", &out.svg);
    assert!(out.svg.contains("<svg"));
    assert!(out.width > out_single.width);
}

#[test]
fn test_render_fraction() {
    let out = render(r"\frac{1}{2}").expect("failed to render");
    print_snapshot("test_render_fraction", &out.svg);
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
    assert!(out.svg.contains("<svg"));
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
