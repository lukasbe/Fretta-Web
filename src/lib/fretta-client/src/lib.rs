use wasm_bindgen::prelude::*;
use fretta_markdown_parser::parse_markdown;

#[wasm_bindgen]
pub fn parse_markdown_to_json_blocks(markdown: &str) -> JsValue {
    let parsed = parse_markdown(markdown);
    match parsed {
        Ok(blocks) => {
            JsValue::from_serde(&blocks).unwrap()
        },
        Err(err) =>  JsValue::from_str(format!("{{error: {}}}", err).as_str())
    }
}