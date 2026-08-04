use std::fmt::{Debug, Display};
use wasm_bindgen::JsValue;

pub trait Error {
    fn to_js_error(&self, prefix: &str) -> JsValue;
}

impl<T: Debug> Error for T {
    fn to_js_error(&self, prefix: &str) -> JsValue {
        js_sys::Error::new(&format!("{prefix}: {:?}", self)).into()
    }
}
