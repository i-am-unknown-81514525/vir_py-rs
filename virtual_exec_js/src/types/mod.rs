pub mod alloc;
pub mod owned;

use std::ops::Deref;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_type::error::MemoryOutOfBoundError;
use virtual_exec_type::ext::{SafeLockArcExt, SafeReadArcExt};
use virtual_exec_type::mem::{MemoryAllocator, Value, ValuePtr};
use crate::Dewrap;
use crate::types::owned::OwnedValueWrapper;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
pub struct ValuePtrWrapper(ValuePtr<'static>);

impl Dewrap<ValuePtr<'static>> for ValuePtrWrapper {
    fn dewrap(self) -> ValuePtr<'static> {
        self.0
    }
}

/// This will potentially destroy the lifetime data corresponded to the machine, which could allow
/// a different machine allocator manage the current machine data
/// Safety: ValuePtr owned all data except PhantomData
pub(crate) fn extend_ptr<'a>(ptr: ValuePtr<'a>) -> ValuePtr<'static> {
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
    ValuePtrWrapper::from(extend_ptr(ptr))
}

impl From<ValuePtr<'static>> for ValuePtrWrapper {
    fn from(ptr: ValuePtr<'static>) -> ValuePtrWrapper {
        Self(ptr)
    }
}

#[wasm_bindgen]
impl ValuePtrWrapper {
    #[wasm_bindgen]
    pub fn to_owned(&self) -> Option<OwnedValueWrapper> {
        let alloc = self.0.read_arc_safe().get_alloc();
        alloc.map(|alloc| alloc.lock_arc_safe().get_owned(&self.0).ok())
            .flatten()
            .map(OwnedValueWrapper::from)
    }

    #[wasm_bindgen]
    pub fn get_kind(&self) -> ValueKindWrapper {
        match self.0.read_arc_safe().deref().inner {
            Value::Int(_) => ValueKindWrapper::Int,
            Value::Float(_) => ValueKindWrapper::Float,
            Value::Bool(_) => ValueKindWrapper::Bool,
            Value::None => ValueKindWrapper::None,
            Value::String(_) => ValueKindWrapper::String,
            Value::Collection(_) => ValueKindWrapper::Collection,
            Value::Object(_) => ValueKindWrapper::Object,
            Value::_Scope(_) => ValueKindWrapper::_Scope,
            Value::MemoryChunk(_) => ValueKindWrapper::MemoryChunk,
            Value::Error(_) => ValueKindWrapper::Error,
            Value::DPtr(_, _) => ValueKindWrapper::DPtr,
            Value::FnPtrExternal(_, _) => ValueKindWrapper::FnPtrExternal,
            Value::Any(_) => ValueKindWrapper::Any,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueKindWrapper {
    Int,
    Float,
    Bool,
    String,
    None,
    Collection,
    Object,
    Error,
    DPtr,
    FnPtrExternal,
    _Scope,
    MemoryChunk,
    Any,
}