use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RenderOptions {
    #[serde(default = "default_font_size")]
    pub font_size: f64,
    #[serde(default)]
    pub display_mode: bool,
}
fn default_font_size() -> f64 { 16.0 }

#[derive(Serialize)]
pub struct RenderResult {
    pub svg: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize)]
pub struct RenderError {
    pub error: String,
}

#[wasm_bindgen]
pub fn render(input: &str, opts: JsValue) -> JsValue {
    let options: RenderOptions = serde_wasm_bindgen::from_value(opts)
        .unwrap_or(RenderOptions { font_size: 16.0, display_mode: false });

    let renderer = swifttex_renderer_svg::SvgRenderer::new(options.font_size, options.display_mode);

    match renderer.render(input) {
        Ok(out) => serde_wasm_bindgen::to_value(&RenderResult {
            svg: out.svg,
            width: out.width,
            height: out.height,
        }).unwrap_or(JsValue::NULL),
        Err(e) => serde_wasm_bindgen::to_value(&RenderError { error: e })
            .unwrap_or(JsValue::NULL),
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
