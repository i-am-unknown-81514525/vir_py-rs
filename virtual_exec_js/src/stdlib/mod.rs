use std::ops::Deref;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::fn_extern::{FnExtern, MethodResolver};
use crate::Dewrap;

#[wasm_bindgen]
#[derive(Clone)]
pub struct MethodResolverWrapper(MethodResolver);

impl Dewrap<MethodResolver> for MethodResolverWrapper {
    fn dewrap(&self) -> MethodResolver {
        self.0.clone()
    }
}

impl From<MethodResolver> for MethodResolverWrapper {
    fn from(value: MethodResolver) -> Self {
        Self(value)
    }
}

#[wasm_bindgen]
pub struct Builtin();

#[wasm_bindgen]
impl Builtin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self()
    }

    #[wasm_bindgen]
    pub fn basic(&self) -> MethodResolverWrapper {
        virtual_exec_std::BASIC.deref().clone().into()
    }

    #[wasm_bindgen]
    pub fn default(&self) -> MethodResolverWrapper {
        virtual_exec_std::DEFAULT.deref().clone().into()
    }
}