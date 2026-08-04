mod state;

use std::fmt::format;
use std::sync::Arc;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::{compile, parse, Machine};
use virtual_exec_type::mem::ValuePtr;
use crate::core::state::StateWrapper;
use crate::types::alloc::AllocatorWrapper;

#[wasm_bindgen]
pub struct MachineWrapper {
    machine: Machine<'static>
}

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
        Ok(Self {
            machine
        })
    }

    #[wasm_bindgen]
    pub fn load_bin(&mut self, code: &Uint8Array) -> Result<(), JsValue> {
        let data: Vec<u8> = code.to_vec();
        let code = virtual_exec_core::binary::import(&data.into())
            .map_err(|e| js_sys::Error::new(&format!("Serialization error: {e:?}")))?;
        self.machine.machine.instructions = code;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_code(&mut self, code: &str) -> Result<(), JsValue> {
        let code = virtual_exec_core::compile(
            &parse(code)
                .map_err(|e| js_sys::Error::new(&format!("Parse error: {e:?}")))?
        );
        self.machine.machine.instructions = code;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_alloc(&self) -> AllocatorWrapper {
        AllocatorWrapper::new(Arc::clone(&self.machine.alloc))
    }

    #[wasm_bindgen]
    pub fn sync_run_once(&mut self) -> StateWrapper {
        StateWrapper::from(self.machine.sync_run_once())
    }
}


/// Safety: All data in machine is owned except PhantomData
fn lifetime_transmute_machine<'a, 'b>(ptr: Machine<'a>) -> Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

