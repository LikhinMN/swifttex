use swifttex_renderer_mathml::MathMLRenderer;

fn render(input: &str) -> String {
    let renderer = MathMLRenderer::new(false);
    renderer.render(input).unwrap().mathml
}

#[test]
fn test_mathml_text() {
    let mathml = render("x");
    assert!(mathml.contains("<mi>x</mi>"));
}

#[test]
fn test_mathml_number() {
    let mathml = render("1");
    assert!(mathml.contains("<mn>1</mn>"));
}

#[test]
fn test_mathml_fraction() {
    let mathml = render(r"\frac{1}{2}");
    assert!(mathml.contains("<mfrac>"));
    assert!(mathml.contains("<mn>1</mn>"));
    assert!(mathml.contains("<mn>2</mn>"));
}

#[test]
fn test_mathml_superscript() {
    let mathml = render(r"x^2");
    assert!(mathml.contains("<msup>"));
}

#[test]
fn test_mathml_subscript() {
    let mathml = render(r"x_1");
    assert!(mathml.contains("<msub>"));
}

#[test]
fn test_mathml_sqrt() {
    let mathml = render(r"\sqrt{x}");
    assert!(mathml.contains("<msqrt>"));
}

#[test]
fn test_mathml_greek() {
    let mathml = render(r"\alpha");
    assert!(mathml.contains("<mi>α</mi>"));
}

#[test]
fn test_mathml_bigop() {
    let mathml = render(r"\sum_{i=0}^{n}");
    assert!(mathml.contains("<munderover>"));
    assert!(mathml.contains("∑"));
}

#[test]
fn test_mathml_matrix() {
    let mathml = render(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}");
    assert!(mathml.contains("<mtable>"));
    assert!(mathml.contains("<mtr>"));
    assert!(mathml.contains("<mtd>"));
    assert!(mathml.contains("("));
    assert!(mathml.contains(")"));
}

#[test]
fn test_mathml_accent() {
    let mathml = render(r"\hat{x}");
    assert!(mathml.contains("<mover>"));
    assert!(mathml.contains("^"));
}

#[test]
fn test_mathml_textop() {
    let mathml = render(r"\sin x");
    assert!(mathml.contains("mathvariant=\"normal\""));
}

#[test]
fn test_mathml_delimiter() {
    let mathml = render(r"\left( x \right)");
    assert!(mathml.contains("stretchy=\"true\""));
}

#[test]
fn test_mathml_empty() {
    let mathml = render("");
    assert!(mathml.contains("<math"));
    assert!(mathml.contains("</math>"));
}

#[test]
fn test_mathml_unknown() {
    let mathml = render(r"\foo");
    assert!(mathml.contains("<!-- unknown: foo -->"));
}

#[test]
fn test_mathml_xmlns() {
    let mathml = render("x");
    assert!(mathml.contains("xmlns=\"http://www.w3.org/1998/Math/MathML\""));
}
