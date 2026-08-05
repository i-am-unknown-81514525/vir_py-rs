use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::fn_extern::{FnExtern, FnExternConstruct};
use virtual_exec_core::Machine;
use virtual_exec_type::error::{CriticalError, ExecutionError, NonRecoverableError};
use virtual_exec_type::mem::{ValueInnerPtr, ValuePtr};
use crate::core::{lifetime_transmute_machine_ref_mut};
use crate::core::machine_ref::MachineRef;
use crate::types::{extend_ptr, ValuePtrWrapper};

#[wasm_bindgen]
pub struct JsExternFuncSync(Option<js_sys::Function>);

impl FnExternConstruct for JsExternFuncSync {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self(None)
    }
}

impl From<js_sys::Function> for JsExternFuncSync {
    fn from(value: js_sys::Function) -> Self {
        Self(Some(value))
    }
}

impl FnExtern for JsExternFuncSync {
    fn fn_extern_sync<'a, 'b>(&self, machine: &'b mut Machine<'a>, values: Vec<ValuePtr<'a>>) -> Result<ValuePtr<'a>, ExecutionError> {
        if let Some(func) = &self.0 {
            let mut machine_ref = MachineRef::from(lifetime_transmute_machine_ref_mut(machine));
            let values_conv: Vec<ValuePtrWrapper> = values.into_iter().map(extend_ptr).map(ValuePtrWrapper::from).collect();
            let result = unsafe {
                machine_ref.with_guarded(|m| {
                    let a = JsValue::from(m);
                    let b = JsValue::from(values_conv);
                    func.call2(&JsValue::NULL, &a, &b)
                })
            };
            let result = match result {
                Some(Ok(v)) => Ok(v),
                Some(Err(e)) => Err(ExecutionError::NonRecoverable(NonRecoverableError::GenericError)),
                None => Err(ExecutionError::Critical(CriticalError::UnexpectedStateError))
            }?;
            todo!()
        } else {
            Err(ExecutionError::NonRecoverable(NonRecoverableError::UnexpectedFunctionCall))
        }
    }

    fn get_size(&self) -> usize {
        1024
    }
}