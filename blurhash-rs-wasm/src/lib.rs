#![no_std]
mod utils;

use wasm_bindgen::prelude::*;

extern crate alloc;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, blurhash-rs-wasm!");
}

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[wasm_bindgen]
pub fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgba_image: &[u8],
) -> Result<String, js_sys::Error> {
    blurhash::encode(components_x, components_y, width, height, rgba_image)
        .map_err(|err| js_sys::Error::new(&err.to_string()).into())
}

#[wasm_bindgen]
pub fn decode(
    blurhash: &str,
    width: u32,
    height: u32,
    punch: Option<f32>,
) -> Result<Vec<u8>, js_sys::Error> {
    let punch = punch.unwrap_or(1.0);
    blurhash::decode(blurhash, width, height, punch)
        .map_err(|err| js_sys::Error::new(&err.to_string()).into())
}
