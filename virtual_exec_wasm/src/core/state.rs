use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::sequential::exec::State;
use virtual_exec_type::error::ExecutionError;
use crate::types::ValuePtrWrapper;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum StateEnum {
    Ok,
    TerminatedEOI,
    TerminatedNotEOI,
    Interrupt,
    FnExternInput,
    FnExternOutput,
    Timeout,
    Error
}

impl Default for StateEnum {
    fn default() -> Self {
        Self::Ok
    }
}

#[wasm_bindgen]
#[derive(Default)]
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

impl From<Result<State<'static>, ExecutionError>> for StateWrapper {
    fn from(value: Result<State<'static>, ExecutionError>) -> Self {
        match value {
            Err(e) => {
                Self {
                    error: Some(js_sys::Error::new(&format!("ExecutionError: {:?}", e)).into()),
                    state_enum: StateEnum::Error,
                    additional_0: None,
                    additional_1: None,
                    additional_2: None
                }
            },
            Ok(State::Ok) =>  Self {
                state_enum: StateEnum::Ok,
                ..Default::default()
            },
            Ok(State::Terminated { end_of_instruction }) => Self {
                state_enum: if end_of_instruction { StateEnum::TerminatedEOI } else { StateEnum::TerminatedNotEOI },
                ..Default::default()
            },
            Ok(State::Interrupt) => Self {
                state_enum: StateEnum::Interrupt,
                ..Default::default()
            },
            Ok(State::Timeout(t)) => Self {
                state_enum: StateEnum::Timeout,
                additional_1: Some(t),
                ..Default::default()
            },
            Ok(State::FnExternInput(fn_name, len)) => Self {
                state_enum: StateEnum::FnExternInput,
                additional_0: Some(fn_name),
                additional_1: Some(len as u64),
                ..Default::default()
            },
            Ok(State::FnExternOutput(fn_name, args)) => Self {
                state_enum: StateEnum::FnExternOutput,
                additional_0: Some(fn_name),
                additional_2: Some(args.into_iter().map(|x| ValuePtrWrapper::from(x)).collect()),
                ..Default::default()
            }
        }
    }
}