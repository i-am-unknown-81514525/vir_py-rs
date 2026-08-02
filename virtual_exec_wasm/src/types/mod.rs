use wasm_bindgen;
use virtual_exec_type::error::MemoryOutOfBoundError;
use virtual_exec_type::ext::SafeLockArcExt;
use virtual_exec_type::mem::{MemoryAllocator, ValuePtr};



#[wasm_bindgen::prelude::wasm_bindgen]
pub struct ValuePtrWrapper {
    inner: ValuePtr<'static>
}

/// This will potentially destroy the lifetime data corresponded to the machine, which could allow
/// a different machine allocator manage the current machine data
/// Safety: ValuePtr owned all data except PhantomData
fn extend_ptr<'a>(ptr: ValuePtr<'a>) -> ValuePtr<'static> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

/// The function will only return a result when the data is presence in the allocator (prevent cross-machine value transfer)
/// Safety: ValuePtr owned all data except PhantomData
fn shorten_ptr<'a>(ptr: ValuePtr<'static>, alloc: MemoryAllocator<'a>) -> Result<ValuePtr<'a>, MemoryOutOfBoundError> {
    let ptr: ValuePtr<'a> = unsafe {
        std::mem::transmute(ptr)
    };
    alloc.lock_arc_safe().get_eq_obj(&ptr)
}

pub fn wrap_ptr(ptr: ValuePtr) -> ValuePtrWrapper {
    ValuePtrWrapper {
        inner: extend_ptr(ptr)
    }
}