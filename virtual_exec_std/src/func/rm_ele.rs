use virtual_exec_extern::*;
use virtual_exec_type::vm_type::*;
use virtual_exec_type::error::{ExecutionError, NonRecoverableError};
use virtual_exec_type::ext::{SafeReadArcExt, SafeWriteArcExt};

#[fn_extern_wrap]
fn rm_ele<'a>(obj: Object<'a>, key: String) -> Result<AnyPtr<'a>, ExecutionError> {
    let mut lock = obj.write_arc_safe();
    if lock.contains_key(&key) {
        let ptr = lock.remove(&key).unwrap();
        if lock.capacity().saturating_sub(100) > lock.len() ||
            lock.capacity().checked_div(lock.len()).unwrap_or(lock.capacity()) > 2 {
            lock.shrink_to_fit();
        }
        Ok(ptr)
    } else {
        Err(ExecutionError::NonRecoverable(NonRecoverableError::ReferenceNotExistError(key)))
    }
}

extern_link!(RmEle, rm_ele, 2);