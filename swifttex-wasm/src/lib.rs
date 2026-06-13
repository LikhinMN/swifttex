use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RenderOptions {
    #[serde(default = "default_font_size")]
    pub font_size: f64,
    #[serde(default)]
    pub display_mode: bool,
    #[serde(default = "default_true")]
    pub inline_fonts: bool,
    #[serde(default)]
    pub math_style: Option<String>,
    #[serde(default)]
    pub output: OutputFormat,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Svg,
    MathML,
    Both,
}

fn default_font_size() -> f64 { 16.0 }
fn default_true() -> bool { true }

#[derive(Serialize)]
pub struct RenderResult {
    pub svg: Option<String>,
    pub mathml: Option<String>,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize)]
pub struct RenderError {
    pub error: String,
}

use std::sync::{Arc, Mutex, OnceLock};
use swifttex_plugin_api::PluginRegistry;

static REGISTRY: OnceLock<Arc<Mutex<PluginRegistry>>> = OnceLock::new();

fn get_registry() -> Arc<Mutex<PluginRegistry>> {
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(PluginRegistry::new()))).clone()
}

#[wasm_bindgen]
pub fn register_symbol(name: &str, unicode_char: &str) {
    let ch = unicode_char.chars().next().unwrap_or('?');
    let registry = get_registry();
    let mut reg = registry.lock().unwrap();
    reg.register_symbol_direct(name.to_string(), ch);
}

#[wasm_bindgen]
pub fn list_plugins() -> JsValue {
    let registry = get_registry();
    let reg = registry.lock().unwrap();
    serde_wasm_bindgen::to_value(&reg.registered_plugins()).unwrap()
}

#[wasm_bindgen]
pub fn reset_registry() {
    let registry = get_registry();
    let mut reg = registry.lock().unwrap();
    *reg = PluginRegistry::new();
}

#[wasm_bindgen]
pub fn render(input: &str, opts: JsValue) -> JsValue {
    let options: RenderOptions = serde_wasm_bindgen::from_value(opts)
        .unwrap_or(RenderOptions { font_size: 16.0, display_mode: false, inline_fonts: true, math_style: None, output: OutputFormat::Svg });

    let style = match options.math_style.as_deref() {
        Some("script")       => swifttex_layout::style::MathStyle::Script,
        Some("scriptscript") => swifttex_layout::style::MathStyle::ScriptScript,
        Some("text")         => swifttex_layout::style::MathStyle::Text,
        _                    => if options.display_mode { swifttex_layout::style::MathStyle::Display }
                                else { swifttex_layout::style::MathStyle::Text },
    };

    let registry = get_registry();

    let renderer_svg = swifttex_renderer_svg::SvgRenderer::new(options.font_size, options.display_mode, options.inline_fonts)
        .with_math_style(style)
        .with_registry(registry.clone());
    
    let renderer_mathml = swifttex_renderer_mathml::MathMLRenderer::new(options.display_mode)
        .with_registry(registry.clone());

    match options.output {
        OutputFormat::Svg => {
            match renderer_svg.render(input) {
                Ok(out) => serde_wasm_bindgen::to_value(&RenderResult {
                    svg: Some(out.svg),
                    mathml: None,
                    width: out.width,
                    height: out.height,
                }).unwrap_or(JsValue::NULL),
                Err(e) => serde_wasm_bindgen::to_value(&RenderError { error: e }).unwrap_or(JsValue::NULL),
            }
        }
        OutputFormat::MathML => {
            match renderer_mathml.render(input) {
                Ok(out) => serde_wasm_bindgen::to_value(&RenderResult {
                    svg: None,
                    mathml: Some(out.mathml),
                    width: 0.0,
                    height: 0.0,
                }).unwrap_or(JsValue::NULL),
                Err(e) => serde_wasm_bindgen::to_value(&RenderError { error: e }).unwrap_or(JsValue::NULL),
            }
        }
        OutputFormat::Both => {
            // Ideally we parse once, but for simplicity we can just render both.
            let svg_res = renderer_svg.render(input);
            let mathml_res = renderer_mathml.render(input);
            match (svg_res, mathml_res) {
                (Ok(s), Ok(m)) => serde_wasm_bindgen::to_value(&RenderResult {
                    svg: Some(s.svg),
                    mathml: Some(m.mathml),
                    width: s.width,
                    height: s.height,
                }).unwrap_or(JsValue::NULL),
                (Err(e), _) | (_, Err(e)) => serde_wasm_bindgen::to_value(&RenderError { error: e }).unwrap_or(JsValue::NULL),
            }
        }
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
