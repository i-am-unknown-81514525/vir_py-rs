mod state;

use std::fmt::format;
use std::sync::Arc;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use virtual_exec_core::{compile, parse, Machine};
use virtual_exec_type::mem::ValuePtr;
use crate::core::state::StateWrapper;
use crate::types::alloc::AllocatorWrapper;

#[wasm_bindgen]
pub struct MachineWrapper(Machine<'static>);

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

    fn from(machine: Machine<'static>) -> Self {
        Self(machine)
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
    pub fn get_alloc(&self) -> AllocatorWrapper {
        AllocatorWrapper::new(Arc::clone(&self.0.alloc))
    }


}


macro_rules! auto_impl_fn {
    ($(($t:ty, $a:tt $($b:ident)? $(($($v:ident : $it:ty),*))? -> $rt:ty)),+ $(,)?) => {
        $(
            auto_impl_fn!($t, $a $($b)? $(($($v : $it),*))? -> $rt);
        )*
    };
    ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                <$rt>::from(self.0.$name($($(($v).dewrap()),*)?))
            }
        }
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub async fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                <$rt>::from(self.0.$name($($(($v).dewrap()),*)?).await)
            }
        }
    };

}

auto_impl_fn!(
    (MachineWrapper, sync_run_once -> StateWrapper),
    (MachineWrapper, async async_run_once -> StateWrapper),
    (MachineWrapper, sync_run_for(count: u64) -> StateWrapper),
    (MachineWrapper, async async_run_for(count: u64) -> StateWrapper),
    (MachineWrapper, sync_run_all -> StateWrapper),
    (MachineWrapper, async async_run_all -> StateWrapper),
    (MachineWrapper, fork -> MachineWrapper)
);


/// Safety: All data in machine is owned except PhantomData
fn lifetime_transmute_machine<'a, 'b>(ptr: Machine<'a>) -> Machine<'b> {
    unsafe {
        std::mem::transmute(ptr)
    }
}

