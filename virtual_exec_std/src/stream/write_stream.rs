use std::ops::Deref;
use virtual_exec_core::fn_extern::fn_args::FnExternArg::Machine;
use crate::stream::write::{OutputByteStream, OutputByteStreamInner};
use virtual_exec_extern::*;
use virtual_exec_type::base::Native;
use virtual_exec_type::error::{ExecutionError, RecoverableError};
use virtual_exec_type::ext::{SafeLockArcExt, SafeReadArcExt};
use virtual_exec_type::vm_type::{Boolean, Error};

#[fn_extern_wrap]
fn write_stream_sync<'__wrap_internal2, 'a>(Native(stream): Native<OutputByteStream>, data: String, Machine(machine): _) -> Result<Boolean, Error> {
    if machine.lock_arc_safe().machine.lim.saturating_sub(data.len() as u64) == 0 {
        return Err(ExecutionError::Recoverable(RecoverableError::TimeoutError(data.len() as u64)))
    }
    let data: Vec<u8> = data.into_bytes();
    Ok(stream.read_arc_safe().sync_fn.f.deref()(data))
}

#[fn_extern_wrap_async]
async fn write_stream_async<'a, '__wrap_internal2>(Native(stream): Native<OutputByteStream>, data: String, Machine(machine): _) -> Result<Boolean, Error> {
    if machine.lock_arc_safe().machine.lim.saturating_sub(data.len() as u64) == 0 {
        return Err(ExecutionError::Recoverable(RecoverableError::TimeoutError(data.len() as u64)))
    }
    let data: Vec<u8> = data.into_bytes();
    let stream = stream.read().await;
    if let Some(f) = &stream.async_fn {
        Ok(f.f.deref()(data).await)
    } else {
        Ok(stream.sync_fn.f.deref()(data))
    }
}

extern_link!(WriteStream, write_stream_sync, write_stream_async, 2);