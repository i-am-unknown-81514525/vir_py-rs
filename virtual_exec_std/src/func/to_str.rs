use cfg_if::cfg_if;
use virtual_exec_core::fn_extern::fn_args::FnExternArg::Recurse;
use virtual_exec_extern::*;
use virtual_exec_type::vm_type::*;
use virtual_exec_type::base::{ToStringSafe, TypeCast};
use virtual_exec_type::error::{ExecutionError, NonRecoverableError};

#[fn_extern_wrap]
fn to_str<'a>(str: AnyPtr<'a>, Recurse(recurse): _) -> Result<String, Error> {
    if let Some(s) = str.as_string() {
        Ok(s)
    } else {
        Ok(str
            .read_arc_blocking()
            .to_string_safe(recurse)
            .map_err(|e| into!(e, ExecutionError))?)
    }
}

extern_link!(ToStr, to_str, 1);