mod utils;

use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello!");
}

#[wasm_bindgen]
pub fn evaluate() {
    alert(format!("Evaluating {}!", unsafe { add_one(1) }).as_str());
}
