use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::Machine;
use virtual_exec_core::machine::PtrAliveCheck;
use crate::core::MachineWrapper;

#[wasm_bindgen]
pub struct MachineRef(*mut Machine<'static>, PtrAliveCheck);

pub struct CheckedMachineRef<'a>(&'a mut Machine<'static>);

impl<'a> From<&'a mut Machine<'static>> for MachineRef {
    fn from(value: &'a mut Machine<'static>) -> Self {
        let check  = value.ptr_alive_check.clone();
        let static_ref = value as *mut Machine<'static>;
        Self(static_ref, check)
    }
}

impl MachineRef {
    pub fn with_guarded<'a, T>(&'a mut self, closure: impl FnOnce(CheckedMachineRef<'a>) -> T) -> Option<T> {
        if !self.1.is_alive() || self.0.is_null() {
            None
        } else {
            let machine_ref = unsafe {
                self.0.as_mut()?
            };
            let refence = CheckedMachineRef(&mut *machine_ref);
            Some(closure(refence))
        }
    }
}

impl<'a> Deref for CheckedMachineRef<'a> {
    type Target = Machine<'static>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DerefMut for CheckedMachineRef<'a> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}