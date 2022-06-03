mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn blake3_hash(input: &str) -> String {
    utils::set_panic_hook();
    let hash = blake3::hash(input.as_bytes());
    hash.to_hex().to_string()
}
