use std::collections::HashMap;
use bumpalo::Bump;
use crate::base::{Value, Downcast, ValueKind};
use std::ops::Add;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub struct VirPyInt { pub value: i64 }
impl VirPyInt { pub fn new(value: i64) -> Self { Self { value } } }

#[derive(Debug, Clone, Copy)]
pub struct VirPyFloat { pub value: f64 }
impl VirPyFloat { pub fn new(value: f64) -> Self { Self { value } } }

#[derive(Debug, Clone)]
pub struct Mapping<'ctx> {
    pub mapping: HashMap<String, Rc<RefCell<Value<'ctx>>>>,
}

#[derive(Debug, Clone)]
pub struct VirPyObject<'ctx> {
    pub mapping: Rc<RefCell<Mapping<'ctx>>>,
}

impl<'ctx> VirPyObject<'ctx> {
    pub fn new() -> Self {
        Self {
            mapping: Rc::new(RefCell::new(Mapping { mapping: HashMap::new() })),
        }
    }
    pub fn get(&self, key: &str) -> Option<Rc<RefCell<Value<'ctx>>>> {
        self.mapping.borrow().mapping.get(key).cloned()
    }
    pub fn set(&self, key: String, value: Value<'ctx>) {
        let value_cell = Rc::new(RefCell::new(value));
        self.mapping.borrow_mut().mapping.insert(key, value_cell);
    }
    pub fn clone(&self) -> Self {
        Self {
            mapping: Rc::clone(&self.mapping),
        }
    }
}

impl<'ctx> Downcast<'ctx> for VirPyInt {
    fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
        value.as_int()
    }
}

impl<'ctx> Downcast<'ctx> for VirPyFloat {
    fn from_value(value: Value<'ctx>) -> Option<&'ctx Self> {
        value.as_float()
    }
}

impl Add for VirPyInt { type Output = Self; fn add(self, rhs: Self) -> Self { Self::new(self.value + rhs.value) } }
impl Add for VirPyFloat { type Output = Self; fn add(self, rhs: Self) -> Self { Self::new(self.value + rhs.value) } }


register_op_add!(VirPyInt, VirPyInt, ValueKind::Int);
register_op_add!(VirPyFloat, VirPyFloat, ValueKind::Float);

