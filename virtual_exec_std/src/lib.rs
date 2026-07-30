pub mod func;

#[cfg(feature = "sys")]
pub mod sys;

#[cfg(feature = "stream")]
mod stream;
pub mod native;

use crate::func::*;
#[cfg(feature = "sys")]
use crate::sys::*;
#[cfg(feature = "stream")]
use crate::stream::*;
use std::sync::LazyLock;
use virtual_exec_core::fn_extern::{FnExternConstruct, MethodResolver};

use virtual_exec_extern::resolve;

pub static BASIC: LazyLock<MethodResolver> = LazyLock::new(|| {
    resolve!(
        ("push_array", PushArray),
        ("pop_array", PopArray),
        ("arr_get_from_idx", ArrGetFromIdx),
        ("create_array", CreateArray),
        ("arr_get_len", ArrGetLen),
        ("concat", Concat),
        ("create_obj", CreateObj),
        ("dir", Dir),
        #[cfg(feature = "sys")]
        ("print", Print),
        #[cfg(feature = "sys")]
        ("println", PrintLn),
        #[cfg(feature = "stream")]
        ("get_output_stream", GetOutputStream),
        #[cfg(feature = "stream")]
        ("write_stream", WriteStream)
    )
});
