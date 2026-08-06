use std::alloc::alloc;
use std::sync::{Arc};
use async_lock::RwLock;
use virtual_exec_core::fn_extern::fn_args::FnExternArg::Alloc;
use virtual_exec_extern::*;
use virtual_exec_type::base::TypeCast;
use virtual_exec_type::error::{ExecutionError, NonRecoverableError};
use virtual_exec_type::HashMap;
use virtual_exec_type::mem::{Allocator, MemoryAllocator, Value, ValuePtr};
use virtual_exec_type::vm_type::*;
use num_traits::{FromPrimitive, ToPrimitive};

#[fn_extern_wrap]
fn int_sync<'a>(v: ValuePtr<'a>) -> Result<Integer, ExecutionError> {
    if let Some(v) = v.as_int() {
        Ok(v)
    } else if let Some(v) = v.as_float() {
        if let Some(i) = v.to_i64() {
            Ok(i)
        } else {
            Err(ExecutionError::NonRecoverable(NonRecoverableError::OverflowError))
        }
    } else {
        Err(ExecutionError::NonRecoverable(NonRecoverableError::InvalidTypeError))
    }
}

extern_link!(Int, int_sync, 1);