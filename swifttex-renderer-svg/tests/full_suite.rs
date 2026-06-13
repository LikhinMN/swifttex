use swifttex_renderer_svg::render;

fn ok(expr: &str) {
    assert!(render(expr).is_ok());
}

#[test]
fn test_v1_expressions() {
    ok("x");
    ok("x^2");
    ok("x_1");
    ok(r"\frac{1}{2}");
    ok(r"\sqrt{x}");
    ok(r"\alpha");
    ok(r"\frac{x^2}{\sqrt{y}}");
    ok(r"\sqrt{\alpha^2 + \beta^2}");
}

#[test]
fn test_v2_expressions() {
    ok(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}");
    ok(r"\left( \frac{x}{y} \right)");
    ok(r"\sum_{i=0}^{n} x_i");
    ok(r"\int_0^\infty f(x) dx");
    ok(r"\frac{\frac{a}{b}}{\frac{c}{d}}");
    ok(r"\left[ \sqrt{\frac{1}{n}} \right]");
    ok(r"\begin{vmatrix} a & b \\ c & d \end{vmatrix}");
    ok(r"\prod_{k=1}^{n} k");
}

#[test]
fn test_v3_expressions() {
    ok(r"\hat{x}");
    ok(r"\bar{\alpha}");
    ok(r"\sin^2 x + \cos^2 x = 1");
    ok(r"\lim_{x \to 0} \frac{\sin x}{x}");
    ok(r"x \, y");
    ok(r"x \leq y");
    ok(r"\forall x \in \mathbb{R}");
    ok(r"\nabla \cdot \vec{E} = \frac{\rho}{\epsilon_0}");
}

#[test]
fn test_complex_combined() {
    ok(r"\frac{\sum_{i=0}^{n} x_i^2}{\sqrt{n-1}}");
    ok(r"\left( \begin{pmatrix} a \\ b \end{pmatrix} \right)");
    ok(r"\lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n");
    ok(r"\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}");
    ok(r"\sum_{n=0}^{\infty} \frac{x^n}{n!}");
    ok(r"\frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u");
}
