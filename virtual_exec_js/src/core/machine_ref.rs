use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use async_lock::RwLock;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::Machine;
use virtual_exec_core::machine::PtrAliveCheck;
use virtual_exec_type::error::ExecutionError;
use virtual_exec_type::ext::{SafeReadArcExt, SafeWriteArcExt};
use virtual_exec_type::mem::OwnedValue;
use crate::auto_impl_fn;
use crate::core::MachineWrapper;
use crate::core::state::StateWrapper;
use crate::error::Error;
use crate::types::alloc::AllocatorWrapper;
use crate::types::owned::OwnedValueWrapper;
use crate::types::ValuePtrWrapper;

#[wasm_bindgen]
pub struct MachineRef(*mut Machine<'static>, PtrAliveCheck);

#[wasm_bindgen]
pub struct CheckedMachineRef(*mut Machine<'static>, Arc<RwLock<bool>>);

impl<'a> From<&'a mut Machine<'static>> for MachineRef {
    fn from(value: &'a mut Machine<'static>) -> Self {
        let check  = value.ptr_alive_check.clone();
        let static_ref = value as *mut Machine<'static>;
        Self(static_ref, check)
    }
}

impl MachineRef {
    pub unsafe fn with_guarded<T>(&mut self, closure: impl FnOnce(CheckedMachineRef) -> T) -> Option<T> {
        if !self.1.is_alive() || self.0.is_null() {
            None
        } else {
            let lock = Arc::new(RwLock::new(true));
            let refence = CheckedMachineRef(self.0.clone(), Arc::clone(&lock));
            let v = closure(refence);
            let _ = std::mem::replace(lock.write_arc_safe().deref_mut(), false);
            Some(v)
        }
    }
}

impl CheckedMachineRef {
    pub fn state(&self) -> bool {
        *self.1.read_arc_safe()
    }
}

macro_rules! auto_impl_fn_injected {
    ($(($t:ty, $a:tt $($b:ident)? $(($($v:ident : $it:ty),*))? -> $rt:ty $(| $f:expr)?)),+ $(,)?) => {
        $(
            auto_impl_fn_injected!($t, $a $($b)? $(($($v : $it),*))? -> $rt $(| $f)?);
        )*
    };
    ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                if !self.state() {
                    panic!("Out of scope");
                }
                use $crate::Dewrap;
                if self.0.is_null() {
                    panic!("Null pointer");
                }
                let machine = unsafe {
                    self.0.as_mut().expect("Unable to get mutable pointer")
                };
                ($f)(machine.$name($($(($v).dewrap()),*)?))
            }
        }
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub async fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                if !self.state() {
                    panic!("Out of scope");
                }
                use $crate::Dewrap;
                if self.0.is_null() {
                    panic!("Null pointer");
                }
                let machine = unsafe {
                    self.0.as_mut().expect("Unable to get mutable pointer")
                };
                ($f)(machine.$name($($(($v).dewrap()),*)?).await)
            }
        }
    };

     ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
         #[allow(non_camel_case_types)]
         const _:() = {
             type __internal = $rt;
             auto_impl_fn_injected!($t, $name $(($($v : $it),*))? -> $rt | __internal::from);
         };
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
        #[allow(non_camel_case_types)]
        const _:() = {
            type __internal = $rt;
            auto_impl_fn_injected!($t, async $name $(($($v : $it),*))? -> $rt | __internal::from);
        };
    };
}

auto_impl_fn_injected!(
    (CheckedMachineRef, sync_run_once -> StateWrapper),
    (CheckedMachineRef, async async_run_once -> StateWrapper),
    (CheckedMachineRef, sync_run_for(count: u64) -> StateWrapper),
    (CheckedMachineRef, async async_run_for(count: u64) -> StateWrapper),
    (CheckedMachineRef, sync_run_all -> StateWrapper),
    (CheckedMachineRef, async async_run_all -> StateWrapper),
    (CheckedMachineRef, fork -> MachineWrapper),
    (CheckedMachineRef, eval_sync_all(code: &str) -> Result<OwnedValueWrapper, JsValue> |
        |v| { OwnedValueWrapper::js_conv(v, "Expression evaluation error")}
    ),
    (CheckedMachineRef, get(name: &str) -> Option<OwnedValueWrapper> |
        |x: Option<OwnedValue>| x.map(|y| y.into())
    ),
    (CheckedMachineRef, get_alloc -> AllocatorWrapper),
    (CheckedMachineRef, set_root(key: String, ptr: ValuePtrWrapper) -> Option<JsValue> |
        |v: Result<(), ExecutionError>| v.map_err(|e| e.to_js_error("Value set error")).err()
    ),
    (CheckedMachineRef, set_top(key: String, ptr: ValuePtrWrapper) -> Option<JsValue> |
        |v: Result<(), ExecutionError>| v.map_err(|e| e.to_js_error("Value set error")).err()
    )
);