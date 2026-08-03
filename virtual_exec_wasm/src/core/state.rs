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
    additional_2: Option<Vec<ValuePtrWrapper>>,
    can_continue_executing: bool
}

#[wasm_bindgen]
impl StateWrapper {
    #[wasm_bindgen(getter)]
    pub fn state_enum(&self) -> StateEnum {
        self.state_enum
    }

    #[wasm_bindgen(getter)]
    pub fn can_continue_executing(&self) -> bool {
        self.can_continue_executing
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
                    additional_2: None,
                    can_continue_executing: true
                }
            },
            Ok(State::Ok) =>  Self {
                state_enum: StateEnum::Ok,
                can_continue_executing: true,
                ..Default::default()
            },
            Ok(State::Terminated { end_of_instruction }) => Self {
                state_enum: if end_of_instruction { StateEnum::TerminatedEOI } else { StateEnum::TerminatedNotEOI },
                can_continue_executing: false,
                ..Default::default()
            },
            Ok(State::Interrupt) => Self {
                state_enum: StateEnum::Interrupt,
                can_continue_executing: false,
                ..Default::default()
            },
            Ok(State::Timeout(t)) => Self {
                state_enum: StateEnum::Timeout,
                additional_1: Some(t),
                can_continue_executing: false,
                ..Default::default()
            },
            Ok(State::FnExternInput(fn_name, len)) => Self {
                state_enum: StateEnum::FnExternInput,
                additional_0: Some(fn_name),
                additional_1: Some(len as u64),
                can_continue_executing: false,
                ..Default::default()
            },
            Ok(State::FnExternOutput(fn_name, args)) => Self {
                state_enum: StateEnum::FnExternOutput,
                additional_0: Some(fn_name),
                additional_2: Some(args.into_iter().map(|x| ValuePtrWrapper::from(x)).collect()),
                can_continue_executing: false,
                ..Default::default()
            }
        }
    }
}

impl From<Result<(State<'static>, bool), ExecutionError>> for StateWrapper {
    fn from(value: Result<(State<'static>, bool), ExecutionError>) -> Self {
        match value {
            Err(e) => {
                Self {
                    error: Some(js_sys::Error::new(&format!("ExecutionError: {:?}", e)).into()),
                    state_enum: StateEnum::Error,
                    additional_0: None,
                    additional_1: None,
                    additional_2: None,
                    can_continue_executing: true
                }
            },
            Ok((State::Ok, v)) =>  Self {
                state_enum: StateEnum::Ok,
                can_continue_executing: v,
                ..Default::default()
            },
            Ok((State::Terminated { end_of_instruction }, v)) => Self {
                state_enum: if end_of_instruction { StateEnum::TerminatedEOI } else { StateEnum::TerminatedNotEOI },
                can_continue_executing: v,
                ..Default::default()
            },
            Ok((State::Interrupt, v)) => Self {
                state_enum: StateEnum::Interrupt,
                can_continue_executing: v,
                ..Default::default()
            },
            Ok((State::Timeout(t), v)) => Self {
                state_enum: StateEnum::Timeout,
                additional_1: Some(t),
                can_continue_executing: v,
                ..Default::default()
            },
            Ok((State::FnExternInput(fn_name, len), v)) => Self {
                state_enum: StateEnum::FnExternInput,
                additional_0: Some(fn_name),
                additional_1: Some(len as u64),
                can_continue_executing: v,
                ..Default::default()
            },
            Ok((State::FnExternOutput(fn_name, args), v)) => Self {
                state_enum: StateEnum::FnExternOutput,
                additional_0: Some(fn_name),
                additional_2: Some(args.into_iter().map(|x| ValuePtrWrapper::from(x)).collect()),
                can_continue_executing: v,
                ..Default::default()
            }
        }
    }
}