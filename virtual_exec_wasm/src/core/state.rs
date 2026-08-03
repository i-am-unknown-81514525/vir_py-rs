use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::types::ValuePtrWrapper;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum StateEnum {
    Ok,
    TerminatedEOI,
    TerminatedNotEOI,
    Interrupt,
    FnExternInput,
    FnExternOutput
}

#[wasm_bindgen]
pub struct StateWrapper {
    error: Option<JsValue>,
    state_enum: StateEnum,
    /// FnExternInput or FnExternOutput function length
    additional_0: Option<String>,
    /// Timeout value or the argument length for FnExternInput
    additional_1: Option<u64>,
    /// FnExternOutput argument values
    additional_2: Option<Vec<ValuePtrWrapper>>
}

#[wasm_bindgen]
impl StateWrapper {
    #[wasm_bindgen(getter)]
    pub fn state_enum(&self) -> StateEnum {
        self.state_enum
    }
}