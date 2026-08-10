mod state;
pub mod machine_ref;
mod fn_extern;

use std::fmt::format;
use std::sync::Arc;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::{compile, parse, Machine};
use virtual_exec_core::fn_extern::{FnExtern, MethodResolver};
use virtual_exec_core::sequential::ParseError;
use virtual_exec_type::error::{CriticalError, ExecutionError, RecoverableError};
use virtual_exec_type::ext::SafeWriteArcExt;
use virtual_exec_type::HashMap;
use virtual_exec_type::mem::{Allocator, OwnedValue, Value, ValuePtr};
use crate::core::state::StateWrapper;
use crate::{auto_impl_fn, Dewrap};
use crate::core::fn_extern::JsExternFuncSync;
use crate::error::Error;
use crate::types::alloc::AllocatorWrapper;
use crate::types::owned::OwnedValueWrapper;
use crate::stdlib::MethodResolverWrapper;
use crate::types::ValuePtrWrapper;


#[wasm_bindgen]
pub struct MachineWrapper(pub(crate) Machine<'static>);

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
    pub fn push_fn(&mut self, name: String, func: js_sys::Function, arg_len: usize) -> Result<(), JsValue> {
        let mut map: HashMap<String, Arc<dyn FnExtern + Send + Sync>> = HashMap::new();
        map.insert(name.clone(), Arc::new(JsExternFuncSync::from(func)));
        self.0.resolvers.insert(0, MethodResolver::new(map));
        let extern_ptr = self.0.alloc.alloc(Value::FnPtrExternal(name.clone().into_boxed_str(), arg_len))
            .map_err(|e| e.to_js_error("Memory allocation failed on external function creation"))?;
        let fn_stack_ref = self.0.machine.fn_stack_frame.get_mut(0)
            .ok_or(ExecutionError::Critical(CriticalError::FnStackUnderflowError))
            .map_err(|e| e.to_js_error("Missing function stack"))?;
        fn_stack_ref.mapping.write_arc_safe().insert(name, extern_ptr);
        Ok(())
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
    ),
    (MachineWrapper, push_code(code: &str) -> Result<(), JsValue> |
        |v: Result<(), ParseError>| {
            v.map_err(|e| e.to_js_error("Parse Error"))
        }
    ),
    (MachineWrapper, push_resolver(resolver: MethodResolverWrapper) -> ()),
    (MachineWrapper, get_alloc -> AllocatorWrapper),
    (MachineWrapper, set_root(key: String, ptr: ValuePtrWrapper) -> Option<JsValue> |
        |v: Result<(), ExecutionError>| v.map_err(|e| e.to_js_error("Value set error")).err()
    ),
    (MachineWrapper, set_top(key: String, ptr: ValuePtrWrapper) -> Option<JsValue> |
        |v: Result<(), ExecutionError>| v.map_err(|e| e.to_js_error("Value set error")).err()
    ),
    (MachineWrapper, grant_lim(additional: u64) -> ()),
    (MachineWrapper, reduce_lim(size: u64) -> Option<JsValue> | 
        |e: Result<(), RecoverableError>| e.map_err(|e| e.to_js_error("Failed to reduce machine limit")).err()
    ),
    (MachineWrapper, check_use(size: u64) -> bool)
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