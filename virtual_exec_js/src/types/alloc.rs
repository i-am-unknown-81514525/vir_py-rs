use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::Machine;
use virtual_exec_type::ext::SafeLockArcExt;
use virtual_exec_type::mem::{MemoryAllocator, OwnedValue, Allocator};
use crate::auto_impl_fn;
use crate::error::Error;
use crate::types::owned::OwnedValueWrapper;
use crate::types::ValuePtrWrapper;

#[wasm_bindgen]
pub struct AllocatorWrapper(MemoryAllocator<'static>);

impl AllocatorWrapper {
    pub fn new(alloc: MemoryAllocator<'static>) -> Self {
        Self::from(alloc)
    }
}

impl From<MemoryAllocator<'static>> for AllocatorWrapper {
    fn from(value: MemoryAllocator<'static>) -> Self {
        Self(value)
    }
}

#[wasm_bindgen]
impl AllocatorWrapper {
    #[wasm_bindgen]
    pub fn get_owned(&mut self, value: ValuePtrWrapper) -> Result<OwnedValueWrapper, JsValue> {
        self.0.lock_arc_safe().get_owned(&value.0)
            .map_err(|e| e.to_js_error("Memory out of bound"))
            .map(|v| v.into())
    }
}

auto_impl_fn!(
    (AllocatorWrapper, curr -> usize),
    (AllocatorWrapper, max -> usize)
);