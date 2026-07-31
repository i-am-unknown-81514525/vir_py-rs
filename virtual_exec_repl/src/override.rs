use std::fmt::Write;
use std::sync::{LazyLock, Mutex as StdMutex};
use virtual_exec_core::fn_extern::MethodResolver;
use virtual_exec_core::fn_extern::fn_args::FnExternArg::Recurse;
use virtual_exec_extern::*;
use virtual_exec_type::base::{Native, ToStringSafe, TypeCast, VmAnyType};
use virtual_exec_type::vm_type::*;
use std::sync::{Arc};
use async_lock::RwLock;
use virtual_exec_std::stream::{OutputByteStream, OutputByteStreamInner};
use virtual_exec_extern::*;
use virtual_exec_type::vm_type::Error;

pub static PRINT_BUFFER: StdMutex<String> = StdMutex::new(String::new());


#[fn_extern_wrap]
fn is_none<'a>(obj: AnyPtr<'a>) -> Result<Boolean, Error> {
    Ok(obj.as_none().is_some())
}

extern_link!(IsNone, is_none, 1);




fn print_sync(vec: Vec<u8>) -> bool {
    match PRINT_BUFFER.try_lock() {
        Ok(mut lock) => lock.write_str(&String::from_utf8_lossy(&vec)).is_ok(),
        Err(_) => false
    }
}


#[fn_extern_wrap]
fn get_output_stream() -> Result<Native<OutputByteStream>, Error> {
    Ok(Native::from(Arc::new(RwLock::new(OutputByteStreamInner::new_sync(print_sync)))))
}


extern_link!(GetOutputStream, get_output_stream, 0);

pub static OVERRIDE: LazyLock<MethodResolver> =
    LazyLock::new(|| resolve!(("get_output_stream", GetOutputStream), ("is_none", IsNone)));



