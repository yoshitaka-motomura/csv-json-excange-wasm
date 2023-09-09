mod utils;
mod core;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
}

#[wasm_bindgen]
pub fn csvtojson(data: &str)-> Result<String, JsValue> {
   let input = data.as_bytes().to_vec();
   match core::csv_to_json(input) {
       Ok(json) => Ok(json),
       Err(e) => {
        let err = format!("Error: {}, If you want to use binaries, use `csvtojson_binary`.", e);
        Err(JsValue::from_str(&err))
       },
   }
}

#[wasm_bindgen]
pub fn csvtojson_binary(data: &[u8])-> Result<String, JsValue> {
   match core::csv_to_json(data.to_vec()) {
       Ok(json) => Ok(json),
       Err(e) => {
        let err = format!("Error: {}, If you want to use text, use `csvtojson`.", e);
        Err(JsValue::from_str(&err))
       },
   }
}

#[wasm_bindgen(start)]
pub fn start() {
    info("CSV to JSON wasm module loaded");
    utils::set_panic_hook();
}