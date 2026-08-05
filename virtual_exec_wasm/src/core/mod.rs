mod state;
pub mod machine_ref;
mod fn_extern;

use std::fmt::format;
use std::sync::Arc;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::{compile, parse, Machine};
use virtual_exec_type::mem::{OwnedValue, ValuePtr};
use crate::core::state::StateWrapper;
use crate::{auto_impl_fn, Dewrap};
use crate::types::alloc::AllocatorWrapper;
use crate::types::owned::OwnedValueWrapper;

#[wasm_bindgen]
pub struct MachineWrapper(Machine<'static>);

#[wasm_bindgen]
impl MachineWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(memory_lim: usize, inst_limit: u64) -> Result<Self, JsValue> {
        let machine = Machine::new(
            vec![],
            memory_lim,
            inst_limit,
            vec![]
        ).map_err(|e| js_sys::Error::new(&format!("Memory error: {:?}", e)))?;
        Ok(Self(machine))
    }

    #[wasm_bindgen]
    pub fn load_bin(&mut self, code: &Uint8Array) -> Result<(), JsValue> {
        let data: Vec<u8> = code.to_vec();
        let code = virtual_exec_core::binary::import(&data.into())
            .map_err(|e| js_sys::Error::new(&format!("Serialization error: {e:?}")))?;
        self.0.machine.instructions = code;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_code(&mut self, code: &str) -> Result<(), JsValue> {
        let code = virtual_exec_core::compile(
            &parse(code)
                .map_err(|e| js_sys::Error::new(&format!("Parse error: {e:?}")))?
        );
        self.0.machine.instructions = code;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_alloc(&self) -> AllocatorWrapper {
        AllocatorWrapper::new(Arc::clone(&self.0.alloc))
    }


}


impl From<Machine<'static>> for MachineWrapper {
    fn from(value: Machine<'static>) -> Self {
        Self(value)
    }
}



auto_impl_fn!(
    (MachineWrapper, sync_run_once -> StateWrapper),
    (MachineWrapper, async async_run_once -> StateWrapper),
    (MachineWrapper, sync_run_for(count: u64) -> StateWrapper),
    (MachineWrapper, async async_run_for(count: u64) -> StateWrapper),
    (MachineWrapper, sync_run_all -> StateWrapper),
    (MachineWrapper, async async_run_all -> StateWrapper),
    (MachineWrapper, fork -> MachineWrapper),
    (MachineWrapper, eval_sync_all(code: &str) -> Result<OwnedValueWrapper, JsValue> |
        |v| { OwnedValueWrapper::js_conv(v, "Expression evaluation error")}
    ),
    (MachineWrapper, get(name: &str) -> Option<OwnedValueWrapper> |
        |x: Option<OwnedValue>| x.map(|y| y.into())
    )
);


/// Safety: All data in machine is owned except PhantomData
pub(crate) fn lifetime_transmute_machine<'a, 'b>(ptr: Machine<'a>) -> Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

pub(crate) fn lifetime_transmute_machine_ref<'a, 'b, 'c>(ptr: &'c Machine<'a>) ->&'c Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

pub(crate) fn lifetime_transmute_machine_ref_mut<'a, 'b, 'c>(ptr: &'c mut Machine<'a>) ->&'c mut Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

impl Dewrap<Machine<'static>> for MachineWrapper {
    fn dewrap(self) -> Machine<'static> {
        self.0
    }
}