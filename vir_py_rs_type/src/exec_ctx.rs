use crate::base::Value;
use crate::builtin::Mapping;
use crate::error::SandboxExecutionError;
use bumpalo::Bump;
use std::cell::RefCell;
use std::rc::Rc;

pub type Result<T> = core::result::Result<T, SandboxExecutionError>;

#[derive(Debug, Clone)]
pub struct ExecutionContext<'ctx> {
    pub arena: Rc<RefCell<Bump>>,
    pub ttl: i64,
    pub mapping: Vec<Rc<RefCell<Mapping<'ctx>>>>, // Top layer ([0]): most local scope
}

// By implementing RefUnwindSafe, we are asserting that even if a panic
// occurs during a method call that mutates ExecutionContext through a
// shared reference, the context is left in a state that won't cause
// undefined behavior *for the caller*. This is acceptable for our interpreter
// because the API contract of Module::eval is that if a panic is caught,
// the entire ExecutionContext must be discarded by the caller.
impl<'ctx> std::panic::RefUnwindSafe for ExecutionContext<'ctx> {}

impl<'ctx> ExecutionContext<'ctx> {
    pub fn new(
        arena: Rc<RefCell<Bump>>,
        ttl: i64,
        mapping: Vec<Rc<RefCell<Mapping<'ctx>>>>,
    ) -> Self {
        Self {
            arena,
            ttl,
            mapping,
        }
    }

    pub fn consume_one(&mut self) -> Result<()> {
        self.consume(1)
    }

    pub fn consume(&mut self, amount: i64) -> Result<()> {
        if amount > self.ttl {
            return Err(SandboxExecutionError::TimeoutError);
        }
        self.ttl -= amount;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Rc<RefCell<Value<'ctx>>>> {
        let mut r: Option<Rc<RefCell<Value<'ctx>>>> = None;
        for mapping in self.mapping.clone() {
            if mapping.borrow().mapping.contains_key(name) {
                r = Some(mapping.borrow().mapping.get(name).unwrap().clone());
            }
        }
        match r {
            Some(v) => Ok(v),
            None => Err(SandboxExecutionError::ReferenceNotExistError(
                name.to_string(),
            )),
        }
    }
}
