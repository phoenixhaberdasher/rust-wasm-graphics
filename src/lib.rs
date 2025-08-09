use wasm_bindgen::prelude::*;
use svg::node::element::Circle;
use svg::Document;

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
