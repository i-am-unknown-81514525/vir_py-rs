use crate::base::{Downcast, Value};
use crate::builtin::VirPyInt;

#[derive(Clone, Debug)]
pub enum SandboxExecutionError {
    TimeoutError,
    ReferenceNotExistError(String),
    DivideByZeroError
}

pub type Result<T> = ::core::result::Result<T, SandboxExecutionError>;

impl<'ctx> Downcast<'ctx> for SandboxExecutionError {
    fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
        value.as_error()
    }
}