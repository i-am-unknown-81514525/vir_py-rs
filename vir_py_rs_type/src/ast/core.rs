use std::cell::RefCell;
use std::rc::{Rc};
use crate::exec_ctx::{ExecutionContext, Result};
use proc_macro2::Span;

pub enum TreeData<T> {
    Root(Vec<T>),
    Leaf
}

pub trait ASTNode {
    type Output;
    fn eval(&self, ctx:Rc<RefCell<ExecutionContext>>) -> Result<Self::Output>;

    fn get_callsite(&self) -> Option<Span>;
}


pub struct VarLit {
    name: String,
    span: Option<Span>
}

impl ASTNode for VarLit {
    type Output = String;
    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
        ctx.borrow_mut().consume_one()?;
        return Ok(self.name.clone())
    }

    fn get_callsite(&self) -> Option<Span> {
        self.span
    }
}