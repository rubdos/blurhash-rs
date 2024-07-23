#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgba_image: &[u8],
) -> Result<String, js_sys::Error> {
    crate::encode(components_x, components_y, width, height, rgba_image)
        .map_err(|err| js_sys::Error::new(&err.to_string()).into())
}

#[wasm_bindgen]
pub fn decode(
    blurhash: &str,
    width: u32,
    height: u32,
    punch: f32,
) -> Result<Vec<u8>, js_sys::Error> {
    crate::decode(blurhash, width, height, punch)
        .map_err(|err| js_sys::Error::new(&err.to_string()).into())
}
