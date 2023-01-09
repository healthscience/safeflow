mod utils;

use wasm_bindgen::prelude::*;

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
    alert("SafeFLow-ECS WASM enabled");
}

#[wasm_bindgen]
pub fn add_heading() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document on window");
    let body = document.body().expect("no body on document");

    let heading = document.create_element("h1")?;
    heading.set_inner_html("SafeFlow-ECS connected to WASM");

    body.append_child(&heading)?;

    Ok(())
}