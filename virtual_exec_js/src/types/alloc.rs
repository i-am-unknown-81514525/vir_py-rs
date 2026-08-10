use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::Machine;
use virtual_exec_type::ext::SafeLockArcExt;
use virtual_exec_type::mem::{MemoryAllocator, OwnedValue, Allocator};
use crate::{auto_impl_fn, Dewrap};
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

impl Dewrap<MemoryAllocator<'static>> for AllocatorWrapper {
    fn dewrap(self) -> MemoryAllocator<'static> {
        self.0
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

macro_rules! auto_impl_fn_inner {
    ($(($t:ty, $a:tt $($b:ident)? $(($($v:ident : $it:ty),*))? -> $rt:ty $(| $f:expr)?)),+ $(,)?) => {
        $(
            auto_impl_fn_inner!($t, $a $($b)? $(($($v : $it),*))? -> $rt $(| $f)?);
        )*
    };
    ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub fn $name(&self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                ($f)(self.0.lock_arc_safe().$name($($(($v).dewrap()),*)?))
            }
        }
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub async fn $name(&self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                ($f)(self.0.lock_arc_safe().$name($($(($v).dewrap()),*)?).await)
            }
        }
    };

     ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
         #[allow(non_camel_case_types)]
         const _:() = {
             type __internal = $rt;
             auto_impl_fn_inner!($t, $name $(($($v : $it),*))? -> $rt | __internal::from);
         };
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
        #[allow(non_camel_case_types)]
        const _:() = {
            type __internal = $rt;
            auto_impl_fn_inner!($t, async $name $(($($v : $it),*))? -> $rt | __internal::from);
        };
    };
}

auto_impl_fn!(
    (AllocatorWrapper, curr -> usize),
    (AllocatorWrapper, max -> usize),
);

auto_impl_fn_inner!(
    (AllocatorWrapper, gc_weak -> ()),
    (AllocatorWrapper, obj_count -> usize),
    (AllocatorWrapper, check_alloc(max: usize) -> bool)
);

#[wasm_bindgen]
pub fn deconstruct_owned_value(value: OwnedValueWrapper, alloc: AllocatorWrapper) -> Result<ValuePtrWrapper, JsValue> {
    virtual_exec_type::mem::deconstruct_owned_value(value.dewrap(), alloc.dewrap())
        .map(ValuePtrWrapper::from)
        .map_err(|e| e.to_js_error("Memory out of bound"))
}