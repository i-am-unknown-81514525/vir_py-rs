use crate::builtin::{VirPyFloat, VirPyInt, VirPyObject};
use bumpalo::Bump;
use std::fmt::Debug;

pub type Value<'ctx> = &'ctx ValueContainer<'ctx>;

#[derive(Debug)]
pub enum ValueKind<'ctx> {
    Int(VirPyInt),
    Float(VirPyFloat),
    Object(VirPyObject<'ctx>),
}

pub trait Downcast<'ctx>: Sized {
    fn from_value(value: Value<'ctx>) -> Option<&'ctx Self>;
}

#[derive(Debug)]
pub struct ValueContainer<'ctx> {
    pub kind: ValueKind<'ctx>,
}

impl<'ctx> ValueContainer<'ctx> {
    pub fn new(kind: ValueKind<'ctx>, arena: &'ctx Bump) -> Value<'ctx> {
        arena.alloc(ValueContainer { kind })
    }

    pub fn clone_in_arena(&self, arena: &'ctx Bump) -> Value<'ctx> {
        let new_kind = match &self.kind {
            ValueKind::Int(i) => ValueKind::Int(i.clone()),
            ValueKind::Float(f) => ValueKind::Float(f.clone()),
            ValueKind::Object(o) => ValueKind::Object(o.clone()),
        };
        ValueContainer::new(new_kind, arena)
    }

    pub fn as_int(&self) -> Option<&VirPyInt> {
        match &self.kind {
            ValueKind::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<&VirPyFloat> {
        match &self.kind {
            ValueKind::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&VirPyObject<'ctx>> {
        match &self.kind {
            ValueKind::Object(o) => Some(o),
            _ => None,
        }
    }
}
