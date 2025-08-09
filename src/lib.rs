use wasm_bindgen::prelude::*;
use svg::node::element::Circle;
use svg::Document;
use console_error_panic_hook;

/// This function is called automatically when the WASM module loads.
/// It sets up panic hooks so Rust panics show up in the browser console.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// This function generates an SVG string with a red circle.
/// It is exposed to JavaScript via wasm-bindgen.
#[wasm_bindgen]
pub fn render_svg() -> String {
    let circle = Circle::new()
        .set("cx", 50)
        .set("cy", 50)
        .set("r", 40)
        .set("fill", "red");

    let document = Document::new().add(circle);
    document.to_string()
}
