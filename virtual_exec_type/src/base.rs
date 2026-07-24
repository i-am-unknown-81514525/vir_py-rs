use crate::HashMap;
use crate::config::recurse::{RecurseRestricter, RecursionError};
use crate::error::{ExecutionError, MemoryError};
use crate::ext::*;
use crate::mem::{Allocator, MemoryAllocator, Value, ValuePtr};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Debug;
use std::any::{Any, TypeId};
use async_lock::{Mutex, RwLock};
use cfg_if::cfg_if;
use crate::vm_type::{AnyType, Object};

pub trait IsTruhy {
    fn is_truthy(&self) -> bool;
}

impl IsTruhy for Value<'_> {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Bool(b) => *b,
            Value::None => false,
            Value::String(s) => s.len() > 0,
            Value::Collection(v) => v.read_arc_safe().len() > 0,
            Value::Object(v) => v.read_arc_safe().len() > 0,
            Value::_Scope(_) => false,
            Value::MemoryChunk(_) => false,
            Value::Error(_) => false,
            Value::DPtr(_, _) => true,
            Value::FnPtrExternal(_, _) => true,
            Value::Any(_) => true,
        }
    }
}

impl IsTruhy for ValuePtr<'_> {
    fn is_truthy(&self) -> bool {
        self.read_arc_safe().is_truthy()
    }
}

pub trait ToStringSafe {
    fn to_string_safe(
        &self,
        recurse_restricter: RecurseRestricter,
    ) -> Result<String, RecursionError>;
}

macro_rules! consume_fmt {
    ($rest:expr, $fmt:literal $(, $args:tt)* $(,)?) => {
        {
            let x = format!($fmt $(, $args)*);
            $rest.consume_mem(x.len() as usize)?;
            x
        }
    };
}

impl ToStringSafe for Value<'_> {
    fn to_string_safe(
        &self,
        recurse_restricter: RecurseRestricter,
    ) -> Result<String, RecursionError> {
        recurse_restricter.consume_inst(1)?;
        Ok(match self {
            Value::Int(i) => consume_fmt!(recurse_restricter, "{}", i),
            Value::Float(f) => consume_fmt!(recurse_restricter, "{}", f),
            Value::Bool(b) => consume_fmt!(recurse_restricter, "{}", b),
            Value::String(s) => consume_fmt!(recurse_restricter, "\"{}\"", s),
            Value::Collection(v) => {
                recurse_restricter.consume_mem((v.read_arc_safe().len() + 1) * 2)?;
                format!(
                    "[{}]",
                    v.read_arc_safe()
                        .iter()
                        .map(|v| Ok(v.to_string_safe(recurse_restricter.incr()?)?))
                        .collect::<Result<Vec<String>, RecursionError>>()?
                        .join(", ")
                )
            }
            Value::Object(v) => {
                recurse_restricter.consume_mem((v.read_arc_safe().len() + 1) * 4)?;
                let key_lens: u64 =
                    v.read_arc_safe().iter().map(|v| v.0.len()).sum::<usize>() as u64;
                recurse_restricter.consume_mem(key_lens as usize)?;
                format!(
                    "{{{}}}",
                    v.read_arc_safe()
                        .iter()
                        .map(|v| Ok(format!(
                            "\"{}\": {}",
                            v.0,
                            v.1.to_string_safe(recurse_restricter.incr()?)?
                        )))
                        .collect::<Result<Vec<String>, RecursionError>>()?
                        .join(", ")
                )
            }
            Value::None => consume_fmt!(recurse_restricter, "None"),
            Value::_Scope(_) => consume_fmt!(recurse_restricter, "_Scoped"),
            Value::MemoryChunk(size) => {
                consume_fmt!(recurse_restricter, "_MemChunk(size: {})", size)
            }
            Value::Error(e) => consume_fmt!(recurse_restricter, "_Error({:?})", e),
            Value::DPtr(ptr, size) => consume_fmt!(
                recurse_restricter,
                "DynFuncPtr(loc: {}, arg_len: {})",
                ptr,
                size
            ),
            Value::FnPtrExternal(name, size) => consume_fmt!(
                recurse_restricter,
                "DynExternFuncPtr(loc: {}, arg_len: {})",
                name,
                size
            ),
            Value::Any(t) => {
                let name = (move || {
                    cfg_if! {
                    if #[cfg(feature = "std")] {
                        t.read_blocking().type_name()
                    } else {
                        t.try_read().expect("Deadlock!").type_name()
                    }
                }
                })();
                consume_fmt!(
                    recurse_restricter,
                    "Any({})",
                    name
                )
            }
        })
    }
}

impl ToStringSafe for ValuePtr<'_> {
    fn to_string_safe(
        &self,
        recurse_restricter: RecurseRestricter,
    ) -> Result<String, RecursionError> {
        self.read_arc_safe().to_string_safe(recurse_restricter)
    }
}

pub trait TypeCast<'a> {
    fn as_int(&self) -> Option<i64>;
    fn as_float(&self) -> Option<f64>;

    fn as_object(&self) -> Option<Arc<RwLock<HashMap<String, ValuePtr<'a>>>>>;

    fn as_collections(&self) -> Option<Arc<RwLock<Vec<ValuePtr<'a>>>>>;

    fn as_string(&self) -> Option<String>;

    fn as_bool(&self) -> Option<bool>;

    fn as_none(&self) -> Option<()>;

    fn as_error(&self) -> Option<ExecutionError>;

    fn as_dptr(&self) -> Option<(u64, usize)>;

    fn as_fn_ptr_extern(&self) -> Option<(String, usize)>;

    fn as_native<T: VmAnyType + Clone>(&self) -> Option<T>;
}

impl<'a> TypeCast<'a> for ValuePtr<'a> {
    fn as_int(&self) -> Option<i64> {
        if let Value::Int(v) = self.read_arc_safe().inner {
            Some(v)
        } else {
            None
        }
    }

    fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self.read_arc_safe().inner {
            Some(b)
        } else {
            None
        }
    }

    fn as_float(&self) -> Option<f64> {
        if let Value::Float(v) = self.read_arc_safe().inner {
            Some(v)
        } else {
            None
        }
    }

    fn as_object(&self) -> Option<Arc<RwLock<HashMap<String, ValuePtr<'a>>>>> {
        if let Value::Object(o) = &self.clone().read_arc_safe().inner {
            Some(o.clone())
        } else {
            None
        }
    }

    fn as_collections(&self) -> Option<Arc<RwLock<Vec<ValuePtr<'a>>>>> {
        if let Value::Collection(c) = &self.clone().read_arc_safe().inner {
            Some(c.clone())
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<String> {
        if let Value::String(s) = &self.read_arc_safe().inner {
            Some(s.to_string())
        } else {
            None
        }
    }

    fn as_none(&self) -> Option<()> {
        let item = &self.read_arc_safe().inner;
        if let Value::None = item {
            Some(())
        } else if let Value::MemoryChunk(_) = item {
            Some(())
        } else if let Value::_Scope(_) = item {
            Some(())
        } else {
            None
        }
    }

    fn as_error(&self) -> Option<ExecutionError> {
        if let Value::Error(e) = &self.read_arc_safe().inner {
            Some(e.clone())
        } else {
            None
        }
    }

    fn as_dptr(&self) -> Option<(u64, usize)> {
        if let Value::DPtr(d, s) = &self.clone().read_arc_safe().inner {
            Some((*d, *s))
        } else {
            None
        }
    }

    fn as_fn_ptr_extern(&self) -> Option<(String, usize)> {
        if let Value::FnPtrExternal(f, s) = &self.clone().read_arc_safe().inner {
            Some((f.to_string(), *s))
        } else {
            None
        }
    }

    fn as_native<T: VmAnyType + Clone>(&self) -> Option<T> {
        let guard = self.read_arc_safe();
        if let Value::Any(a) = &guard.inner { a.cloned_as::<T>() } else { None }
    }
}

pub trait Downcast<'ctx>: Sized {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self>;
}

pub trait Upcast<'ctx>: Sized {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError>;
}

impl<'ctx> Downcast<'ctx> for bool {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_bool()
    }
}

impl<'ctx> Upcast<'ctx> for bool {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Bool(self))
    }
}

impl<'ctx> Downcast<'ctx> for i64 {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_int()
    }
}

impl<'ctx> Upcast<'ctx> for i64 {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Int(self))
    }
}

impl<'ctx> Downcast<'ctx> for f64 {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_float()
    }
}

impl<'ctx> Upcast<'ctx> for f64 {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Float(self))
    }
}

impl<'ctx> Downcast<'ctx> for Arc<RwLock<Vec<ValuePtr<'ctx>>>> {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_collections()
    }
}

impl<'ctx> Upcast<'ctx> for Arc<RwLock<Vec<ValuePtr<'ctx>>>> {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Collection(self))
    }
}

impl<'ctx> Downcast<'ctx> for String {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_string()
    }
}

impl<'ctx> Upcast<'ctx> for String {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::String(self.into_boxed_str()))
    }
}

impl<'ctx> Downcast<'ctx> for () {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_none()
    }
}

impl<'ctx> Upcast<'ctx> for () {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::None)
    }
}

impl<'ctx> Downcast<'ctx> for ExecutionError {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_error()
    }
}

impl<'ctx> Upcast<'ctx> for ExecutionError {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Error(self))
    }
}

impl<'ctx> Downcast<'ctx> for ValuePtr<'ctx> {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        Some(value)
    }
}

impl<'ctx> Upcast<'ctx> for ValuePtr<'ctx> {
    fn from_value(self, _alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        Ok(self)
    }
}

impl<'ctx> Downcast<'ctx> for Object<'ctx> {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_object()
    }
}

impl<'ctx> Upcast<'ctx> for Object<'ctx> {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Object(self))
    }
}

pub trait VmAnyType : Send + Sync + Debug + Any {
    fn get_size(&self) -> usize;

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<'ctx> Downcast<'ctx> for AnyType {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        if let Value::Any(v) = value.read_arc_safe().inner.clone() {
            Some(v)
        } else {
            None
        }
    }
}

impl<'ctx> Upcast<'ctx> for AnyType {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Any(self))
    }
}

macro_rules! read_any {
    ($v:expr) => {{
        cfg_if! {
            if #[cfg(feature = "std")] { $v.read_blocking() }
            else { $v.try_read().expect("Deadlock!") }
        }
    }};
}

pub trait AnyCast {
    fn is_type<T: VmAnyType>(&self) -> bool;
    fn with_type<T: VmAnyType, R>(&self, f: impl FnOnce(&T) -> R) -> Option<R>;
    fn cloned_as<T: VmAnyType + Clone>(&self) -> Option<T>;
}

impl AnyCast for AnyType {
    fn is_type<T: VmAnyType>(&self) -> bool {
        let guard = read_any!(self);
        (&*guard as &dyn Any).is::<T>()
    }

    fn with_type<T: VmAnyType, R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        let guard = read_any!(self);
        let any: &dyn Any = &*guard;
        any.downcast_ref::<T>().map(f)
    }

    fn cloned_as<T: VmAnyType + Clone>(&self) -> Option<T> {
        self.with_type::<T, _>(|v| v.clone())
    }
}

/// Note that data will be cloned out when downcast, so an arc pointer should be used when try to reference the data
pub struct Native<T>(pub T);

impl<'ctx, T: VmAnyType + Clone> Downcast<'ctx> for Native<T> {
    fn from_value(value: ValuePtr<'ctx>) -> Option<Self> {
        value.as_native::<T>().map(Native)
    }
}

impl<'ctx, T: VmAnyType> Upcast<'ctx> for Native<T> {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Any(Arc::new(RwLock::new(self.0))))
    }
}


impl<'ctx, T: VmAnyType> Upcast<'ctx> for T {
    fn from_value(self, alloc: &MemoryAllocator<'ctx>) -> Result<ValuePtr<'ctx>, MemoryError> {
        alloc.alloc(Value::Any(Arc::new(RwLock::new(self))))
    }
}

impl<T> From<T> for Native<T> {
    fn from(value: T) -> Self {
        Native(value)
    }
}

impl<T: VmAnyType> VmAnyType for Arc<RwLock<T>> {
    fn get_size(&self) -> usize {
        self.read_arc_safe().get_size()
    }
}

impl<T: VmAnyType> VmAnyType for Arc<Mutex<T>> {
    fn get_size(&self) -> usize {
        self.lock_arc_safe().get_size()
    }
}

impl<T: VmAnyType> VmAnyType for Arc<std::sync::RwLock<T>> {
    fn get_size(&self) -> usize {
        self.read().unwrap().get_size()
    }
}

impl<T: VmAnyType> VmAnyType for Arc<std::sync::Mutex<T>> {
    fn get_size(&self) -> usize {
        self.lock().unwrap().get_size()
    }
}