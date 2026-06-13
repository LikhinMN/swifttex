use swifttex_renderer_svg::render;

fn is_valid_svg(s: &str) -> bool {
    s.contains("<svg") && s.contains("</svg>") && s.contains("xmlns")
}

#[test]
fn test_integration_1() {
    let out = render("x").unwrap();
    assert!(is_valid_svg(&out.svg));
    assert!(out.width > 5.0 && out.width < 50.0);
}

#[test]
fn test_integration_2() {
    let out_single = render("x").unwrap();
    let out = render("x^2").unwrap();
    assert!(is_valid_svg(&out.svg));
    assert!(out.width > out_single.width);
}

#[test]
fn test_integration_3() {
    let out = render("x_1").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_4() {
    let out = render(r"\frac{1}{2}").unwrap();
    assert!(is_valid_svg(&out.svg));
    assert!(out.height > 20.0);
}

#[test]
fn test_integration_5() {
    let out = render(r"\sqrt{x}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_6() {
    let out = render(r"\alpha").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_7() {
    let out = render(r"\beta + \gamma").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_8() {
    let out = render(r"\frac{x^2}{\sqrt{y}}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_9() {
    let out = render(r"\frac{\alpha}{\beta}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_10() {
    let out = render(r"\sqrt{\frac{1}{2}}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_11() {
    let out = render(r"x^{y^2}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_12() {
    let out = render(r"\frac{1}{2} + \frac{3}{4}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_13() {
    let out = render(r"\alpha^2 + \beta_1").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_14() {
    let out = render(r"\sqrt{\alpha^2 + \beta^2}").unwrap();
    assert!(is_valid_svg(&out.svg));
}

#[test]
fn test_integration_15() {
    let out = render("").unwrap();
    assert!(is_valid_svg(&out.svg));
    assert!(out.width < 1.0);
    assert!(out.height < 1.0);
}
