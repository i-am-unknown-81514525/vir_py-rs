use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::Machine;
use virtual_exec_type::mem::MemoryAllocator;

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