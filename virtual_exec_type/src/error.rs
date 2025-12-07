use crate::base::{Downcast, Value};
//
// #[derive(Clone, Debug, Copy)]
// pub struct LineSpan {
//     pub line: u64,
//     pub column: u64,
//     pub length: u64
// }
//
// // TODO ^ Since I don't actually have the span yet

#[derive(Clone, Debug)]
pub enum SandboxExecutionError {
    TimeoutError,
    ReferenceNotExistError(String),
    DivideByZeroError,
    GenericPanicRewindError,
    UndefinedOperatorMethodError,
    InvalidTypeError,
    InvalidSyntaxError,
}

pub type Result<T> = ::core::result::Result<T, SandboxExecutionError>;

impl<'ctx> Downcast<'ctx> for SandboxExecutionError {
    fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
        value.as_error()
    }
}
