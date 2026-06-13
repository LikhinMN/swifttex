use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swifttex_renderer_svg::render;

fn bench_v1_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("v1");
    group.bench_function("simple: x^2", |b| {
        b.iter(|| render(black_box("x^2")).unwrap())
    });
    group.bench_function("fraction: \\frac{1}{2}", |b| {
        b.iter(|| render(black_box(r"\frac{1}{2}")).unwrap())
    });
    group.bench_function("sqrt: \\sqrt{x^2+y^2}", |b| {
        b.iter(|| render(black_box(r"\sqrt{x^2+y^2}")).unwrap())
    });
    group.bench_function("greek: \\alpha + \\beta", |b| {
        b.iter(|| render(black_box(r"\alpha + \beta")).unwrap())
    });
    group.finish();
}

fn bench_v2_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2");
    group.bench_function("matrix: 2x2 pmatrix", |b| {
        b.iter(|| render(black_box(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}")).unwrap())
    });
    group.bench_function("delimiter: \\left( \\frac{x}{y} \\right)", |b| {
        b.iter(|| render(black_box(r"\left( \frac{x}{y} \right)")).unwrap())
    });
    group.bench_function("bigop: \\sum_{i=0}^{n} x_i", |b| {
        b.iter(|| render(black_box(r"\sum_{i=0}^{n} x_i")).unwrap())
    });
    group.bench_function("style cascade: nested fractions", |b| {
        b.iter(|| render(black_box(r"\frac{\frac{a}{b}}{\frac{c}{d}}")).unwrap())
    });
    group.finish();
}

fn bench_v3_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("v3");
    group.bench_function("mathml: \\frac{x^2}{\\sqrt{y}}", |b| {
        b.iter(|| {
            use swifttex_renderer_mathml::MathMLRenderer;
            MathMLRenderer::new(false).render(black_box(r"\frac{x^2}{\sqrt{y}}")).unwrap()
        })
    });
    group.bench_function("accessible render: both outputs", |b| {
        b.iter(|| {
            use swifttex_renderer_svg::render_accessible;
            render_accessible(black_box(r"\sum_{i=0}^{n} \frac{x_i^2}{\sigma^2}"), None).unwrap()
        })
    });
    group.finish();
}

fn bench_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress");
    group.bench_function("1000x simple render", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                render(black_box("x^2")).unwrap();
            }
        })
    });
    group.bench_function("complex expression 100x", |b| {
        b.iter(|| {
            for _ in 0..100 {
                render(black_box(r"\frac{\alpha^2 + \sqrt{\beta}}{\gamma_{n+1}}")).unwrap();
            }
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_v1_expressions,
    bench_v2_expressions,
    bench_v3_expressions,
    bench_stress
);
criterion_main!(benches);
