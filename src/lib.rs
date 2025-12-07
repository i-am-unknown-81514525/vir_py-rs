use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use bumpalo::Bump;
use vir_py_rs_parser::error::ParseError;
use vir_py_rs_parser::parser;
use vir_py_rs_type::ast::core::ASTNode;
use vir_py_rs_type::builtin::Mapping;
use vir_py_rs_type::exec_ctx::{ExecutionContext, RsValue};
use vir_py_rs_type::error::SandboxExecutionError;

/// The unified error type for the `vir_py-rs` library.
#[derive(Debug)]
pub enum ExecError {
    /// An error that occurred during the parsing phase.
    Parse(ParseError),
    /// An error that occurred during the execution phase.
    Execution(SandboxExecutionError),
}

impl From<ParseError> for ExecError {
    fn from(e: ParseError) -> Self {
        ExecError::Parse(e)
    }
}

impl From<SandboxExecutionError> for ExecError {
    fn from(e: SandboxExecutionError) -> Self {
        ExecError::Execution(e)
    }
}

/// Executes a string of Python-like code in a sandboxed environment.
///
/// # Arguments
///
/// * `code` - A string slice containing the code to execute.
/// * `ttl` - A time-to-live value representing the maximum number of operations allowed.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(HashMap<String, PyValue>)` - A dictionary of the final state of all variables.
/// * `Err(Error)` - An error that occurred during parsing or execution.
pub fn exec(code: &str, ttl: i64) -> Result<HashMap<String, RsValue>, ExecError> {
    // 1. Parse the code into an AST.
    let module = parser::parse(code)?;

    let arena = Rc::new(RefCell::new(Bump::new()));
    let global_scope = Rc::new(RefCell::new(Mapping { mapping: HashMap::new() }));
    let mapping = vec![global_scope];
    let ctx = Rc::new(RefCell::new(ExecutionContext::new(arena, ttl, mapping)));

    module.eval(ctx.clone())?;

    let final_state = ctx.borrow().to_hashmap();
    Ok(final_state)
}
