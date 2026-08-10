pub mod core;
pub mod types;
mod error;
pub mod stdlib;

pub trait Dewrap<T> {
    fn dewrap(&self) -> T;
}

impl<T: Clone> Dewrap<T> for T {
    fn dewrap(&self) -> T {
        self.clone()
    }
}

impl Dewrap<String> for &str {
    fn dewrap(&self) -> String {
        self.to_string()
    }
}

macro_rules! auto_impl_fn {
    ($(($t:ty, $a:tt $($b:ident)? $(($($v:ident : $it:ty),*))? -> $rt:ty $(| $f:expr)?)),+ $(,)?) => {
        $(
            $crate::auto_impl_fn!($t, $a $($b)? $(($($v : $it),*))? -> $rt $(| $f)?);
        )*
    };
    ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                ($f)(self.0.$name($($(($v).dewrap()),*)?))
            }
        }
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty | $f:expr) => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen]
            #[allow(unused_imports)]
            pub async fn $name(&mut self, $($($v : $it),*)?) -> $rt {
                use $crate::Dewrap;
                ($f)(self.0.$name($($(($v).dewrap()),*)?).await)
            }
        }
    };

     ($t:ty, $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
         #[allow(non_camel_case_types)]
         const _:() = {
             type __internal = $rt;
             $crate::auto_impl_fn!($t, $name $(($($v : $it),*))? -> $rt | __internal::from);
         };
    };

    ($t:ty, async $name:ident $(($($v:ident : $it:ty),*))? -> $rt:ty) => {
        #[allow(non_camel_case_types)]
        const _:() = {
            type __internal = $rt;
            $crate::auto_impl_fn!($t, async $name $(($($v : $it),*))? -> $rt | __internal::from);
        };
    };
}

use wasm_bindgen::JsValue;
pub(crate) use auto_impl_fn;
use virtual_exec_core::parse;
use virtual_exec_core::sequential::ParseError;
use crate::error::Error;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn syntax_check_err(code: &str) -> Option<JsValue> {
    parse(code).map_err(|x| x.to_js_error("Parse error: ")).err()
}

#[wasm_bindgen]
pub fn syntax_check(code: &str) -> bool {
    parse(code).is_ok()
}