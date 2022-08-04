use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Info {
    id: u32,
    name: String,
}

// 通过 Serialize struct 返回
#[wasm_bindgen]
pub fn get_json_from_wasm_by_struct() -> JsValue {
    JsValue::from_serde(&Info {
        id: 1,
        name: "rust".to_string(),
    })
    .unwrap()
}

// 通过纯字符串返回
#[wasm_bindgen]
pub fn get_json_from_wasm_by_str() -> JsValue {
    JsValue::from_serde(
        &serde_json::from_str::<serde_json::Value>(r#"{"a":1,"b":{"c":[1,2,3]},"d":false}"#)
            .unwrap(),
    )
    .unwrap()
}

// 通过拼接变量返回
#[wasm_bindgen]
pub fn get_json_from_wasm_by_splice_var() -> JsValue {
    let name = "rust";
    let id = 1;

    JsValue::from_serde(&serde_json::json!({
        "from": "wasm",
        "name": name,
        "id": id
    }))
    .unwrap()
}

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
