use std::ops::Deref;
use virtual_exec_core::fn_extern::fn_args::FnExternArg::{Alloc, Machine};
use crate::stream::read::{InputByteStream, InputByteStreamInner};
use virtual_exec_extern::*;
use virtual_exec_type::base::Native;
use virtual_exec_type::error::{ExecutionError, RecoverableError};
use virtual_exec_type::ext::{SafeLockArcExt, SafeReadArcExt};
use virtual_exec_type::mem::{Allocator, Value};
use virtual_exec_type::vm_type::{AnyPtr, Boolean, Error};

#[fn_extern_wrap]
fn read_stream_sync<'__wrap_internal2, 'a>(Native(stream): Native<InputByteStream>,  Machine(machine): _, Alloc(alloc): _) -> Result<AnyPtr<'a>, Error> {
    let stream = stream.read_arc_safe();
    let data = stream.sync_fn.f.deref()();
    match data {
        Some(v) => {
            alloc.lock_arc_safe().check_alloc_err(v.len())?;
            if machine.lock_arc_safe().machine.lim.saturating_sub(v.len() as u64) == 0 {
                return Err(ExecutionError::Recoverable(RecoverableError::TimeoutError(v.len() as u64)))
            }
            let string = String::from_utf8_lossy(&v);
            alloc.alloc(Value::String(Box::from(string))).map_err(|e| e.into())
        },
        None => {
            alloc.alloc(Value::None).map_err(|e| e.into())
        }
    }
}

#[fn_extern_wrap_async]
async fn read_stream_async<'__wrap_internal2, 'a>(Native(stream): Native<InputByteStream>,  Machine(machine): _, Alloc(alloc): _) -> Result<AnyPtr<'a>, Error> {
    let stream = stream.read().await;
    let data = if let Some(f) = &stream.async_fn {
        f.f.deref()().await
    } else {
        stream.sync_fn.f.deref()()
    };
    match data {
        Some(v) => {
            alloc.lock().await.check_alloc_err(v.len())?;
            if machine.lock().await.machine.lim.saturating_sub(v.len() as u64) == 0 {
                return Err(ExecutionError::Recoverable(RecoverableError::TimeoutError(v.len() as u64)))
            }
            let string = String::from_utf8_lossy(&v);
            alloc.alloc(Value::String(Box::from(string))).map_err(|e| e.into())
        },
        None => {
            alloc.alloc(Value::None).map_err(|e| e.into())
        }
    }
}

extern_link!(ReadStream, read_stream_sync, read_stream_async, 1);