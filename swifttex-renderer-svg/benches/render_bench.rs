use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swifttex_renderer_svg::render;

fn bench_simple(c: &mut Criterion) {
    c.bench_function("simple: x^2", |b| {
        b.iter(|| render(black_box("x^2")).unwrap())
    });
}

fn bench_medium(c: &mut Criterion) {
    c.bench_function("medium: frac with superscript", |b| {
        b.iter(|| render(black_box(r"\frac{x^2 + 1}{y_1}")).unwrap())
    });
}

fn bench_complex(c: &mut Criterion) {
    c.bench_function("complex: nested frac sqrt greek", |b| {
        b.iter(|| render(black_box(r"\frac{\alpha^2 + \sqrt{\beta}}{\gamma_{n+1}}")).unwrap())
    });
}

fn bench_repeated(c: &mut Criterion) {
    c.bench_function("stress: 100 renders", |b| {
        b.iter(|| {
            for _ in 0..100 {
                render(black_box(r"\frac{x^2}{\sqrt{y}}")).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_simple, bench_medium, bench_complex, bench_repeated);
criterion_main!(benches);
