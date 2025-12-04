use std::rc::Rc;
use bumpalo::Bump;
use crate::builtin::Mapping;
use crate::error::SandboxExecutionError;

pub type Result<T> = core::result::Result<T, SandboxExecutionError>;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub arena: Rc<Bump>,
    pub ttl: i64,
    pub mapping: Rc<Mapping>
}

impl ExecutionContext {
    pub fn new(arena: Rc<Bump>, ttl: i64, mapping: Rc<Mapping>) -> Self {
        Self {
            arena,
            ttl,
            mapping
        }
    }
    
    pub fn consume_one(&mut self) -> Result<()> {
        self.consume(1)
    }
    
    pub fn consume(&mut self, amount: i64) -> Result<()> {
        if amount > self.ttl {
            return Err(SandboxExecutionError::TimeoutError)
        }
        self.ttl -= amount;
        Ok(())
    }
}

