use swifttex_renderer_svg::render;

fn main() {
    let out = render(r"\frac{x^2}{\sqrt{y}}").unwrap();
    println!("{}", out.svg);
}
