mod state;

use std::fmt::format;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::{compile, parse, Machine};
use virtual_exec_type::mem::ValuePtr;
use crate::types::alloc::AllocatorWrapper;

#[wasm_bindgen]
pub struct MachineWrapper {
    machine: Machine<'static>
}

#[wasm_bindgen]
impl MachineWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(code: &str, memory_lim: usize, inst_limit: u64) -> Result<Self, JsValue> {
        let inst = compile(
            &parse(code)
                .map_err(|e| js_sys::Error::new(&format!("Parse Error: {:?}", e)))?
        );
        let machine = Machine::new(
            inst,
            memory_lim,
            inst_limit,
            vec![]
        ).map_err(|e| js_sys::Error::new(&format!("Memory error: {:?}", e)))?;
        Ok(Self {
            machine
        })
    }

    #[wasm_bindgen]
    pub fn get_alloc(&self) -> AllocatorWrapper {
        AllocatorWrapper::new(Arc::clone(&self.machine.alloc))
    }

}


/// Safety: All data in machine is owned except PhantomData
fn lifetime_transmute_machine<'a, 'b>(ptr: Machine<'a>) -> Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

