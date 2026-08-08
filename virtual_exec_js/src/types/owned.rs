use wasm_bindgen::JsValue;
use virtual_exec_type::mem::{get_all_owned_value, OwnedValue};
use crate::error::Error;
use wasm_bindgen::prelude::wasm_bindgen;





#[wasm_bindgen]
pub struct OwnedValueWrapper {
    inner: OwnedValue
}

impl OwnedValueWrapper {
    
    pub fn js_conv<T: Error>(res: Result<OwnedValue, T>, prefix: &str) -> Result<Self, JsValue> {
        match res {
            Ok(v) => Ok(Self::from(v)),
            Err(e) => Err(e.to_js_error(prefix))
        }
    }
}

impl From<OwnedValue> for OwnedValueWrapper {
    fn from(value: OwnedValue) -> Self {
        Self {
            inner: value
        }
    }
}

#[wasm_bindgen]
impl OwnedValueWrapper {
    #[wasm_bindgen]
    pub fn to_js(&self) -> JsValue {
        let all = get_all_owned_value(self.inner.clone());
        todo!()
    }
}