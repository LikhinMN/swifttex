use swifttex_renderer_mathml::MathMLRenderer;

fn ok_ml(expr: &str) {
    let r = MathMLRenderer::new(false).render(expr).unwrap();
    assert!(r.mathml.contains("<math"));
    assert!(r.mathml.contains("</math>"));
}

#[test]
fn test_v1_expressions() {
    ok_ml("x");
    ok_ml("x^2");
    ok_ml("x_1");
    ok_ml(r"\frac{1}{2}");
    ok_ml(r"\sqrt{x}");
    ok_ml(r"\alpha");
    ok_ml(r"\frac{x^2}{\sqrt{y}}");
    ok_ml(r"\sqrt{\alpha^2 + \beta^2}");
}

#[test]
fn test_v2_expressions() {
    ok_ml(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}");
    ok_ml(r"\left( \frac{x}{y} \right)");
    ok_ml(r"\sum_{i=0}^{n} x_i");
    ok_ml(r"\int_0^\infty f(x) dx");
    ok_ml(r"\frac{\frac{a}{b}}{\frac{c}{d}}");
    ok_ml(r"\left[ \sqrt{\frac{1}{n}} \right]");
    ok_ml(r"\begin{vmatrix} a & b \\ c & d \end{vmatrix}");
    ok_ml(r"\prod_{k=1}^{n} k");
}

#[test]
fn test_v3_expressions() {
    ok_ml(r"\hat{x}");
    ok_ml(r"\bar{\alpha}");
    ok_ml(r"\sin^2 x + \cos^2 x = 1");
    ok_ml(r"\lim_{x \to 0} \frac{\sin x}{x}");
    ok_ml(r"x \, y");
    ok_ml(r"x \leq y");
    ok_ml(r"\forall x \in \mathbb{R}");
    ok_ml(r"\nabla \cdot \vec{E} = \frac{\rho}{\epsilon_0}");
}

#[test]
fn test_complex_combined() {
    ok_ml(r"\frac{\sum_{i=0}^{n} x_i^2}{\sqrt{n-1}}");
    ok_ml(r"\left( \begin{pmatrix} a \\ b \end{pmatrix} \right)");
    ok_ml(r"\lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n");
    ok_ml(r"\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}");
    ok_ml(r"\sum_{n=0}^{\infty} \frac{x^n}{n!}");
    ok_ml(r"\frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u");
}
