use mktoc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_toc(content: &str) -> String {
    mktoc::add_toc(
        content.to_string(),
        mktoc::generate_toc(content.to_string(), mktoc::Config::default()),
    )
}
#[wasm_bindgen]
pub fn make_toc_only(content: &str) -> String {
    mktoc::generate_toc(content.to_string(), mktoc::Config::default())
}
