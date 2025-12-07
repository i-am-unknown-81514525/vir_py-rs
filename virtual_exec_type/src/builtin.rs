use crate::base::{Downcast, Upcast, Value, ValueKind};
use crate::error::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct VirPyInt {
    pub value: i64,
}
impl VirPyInt {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirPyFloat {
    pub value: f64,
}
impl VirPyFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

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
            mapping: Rc::new(RefCell::new(Mapping {
                mapping: HashMap::new(),
            })),
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

impl<'ctx> Default for VirPyObject<'ctx> {
    fn default() -> Self {
        Self::new()
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

impl<'ctx> Upcast<'ctx> for VirPyInt {
    fn from_value(&'ctx self) -> ValueKind<'ctx> {
        ValueKind::Int(*self)
    }
}

impl<'ctx> Upcast<'ctx> for VirPyFloat {
    fn from_value(&'ctx self) -> ValueKind<'ctx> {
        ValueKind::Float(*self)
    }
}

impl Add for VirPyInt {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value + rhs.value))
    }
}
impl Add for VirPyFloat {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value + rhs.value))
    }
}
impl Add<VirPyInt> for VirPyFloat {
    type Output = Result<Self>;
    fn add(self, rhs: VirPyInt) -> Self::Output {
        Ok(Self::new(self.value + (rhs.value as f64)))
    }
}
impl Add<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn add(self, rhs: VirPyFloat) -> Self::Output {
        Ok(VirPyFloat::new(self.value as f64 + rhs.value))
    }
}

impl Sub for VirPyInt {
    type Output = Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value - rhs.value))
    }
}
impl Sub for VirPyFloat {
    type Output = Result<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value - rhs.value))
    }
}
impl Sub<VirPyInt> for VirPyFloat {
    type Output = Result<Self>;
    fn sub(self, rhs: VirPyInt) -> Self::Output {
        Ok(Self::new(self.value - (rhs.value as f64)))
    }
}
impl Sub<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn sub(self, rhs: VirPyFloat) -> Self::Output {
        Ok(VirPyFloat::new(self.value as f64 - rhs.value))
    }
}

impl Mul for VirPyInt {
    type Output = Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value * rhs.value))
    }
}
impl Mul for VirPyFloat {
    type Output = Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self::new(self.value * rhs.value))
    }
}
impl Mul<VirPyInt> for VirPyFloat {
    type Output = Result<Self>;
    fn mul(self, rhs: VirPyInt) -> Self::Output {
        Ok(Self::new(self.value * (rhs.value as f64)))
    }
}
impl Mul<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn mul(self, rhs: VirPyFloat) -> Self::Output {
        Ok(VirPyFloat::new(self.value as f64 * rhs.value))
    }
}

impl Div for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        Ok(VirPyFloat::new((self.value as f64) / (rhs.value as f64)))
    }
}
impl Div for VirPyFloat {
    type Output = Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0f64 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        Ok(Self::new(self.value / rhs.value))
    }
}
impl Div<VirPyInt> for VirPyFloat {
    type Output = Result<Self>;
    fn div(self, rhs: VirPyInt) -> Self::Output {
        if rhs.value == 0 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        Ok(Self::new(self.value / (rhs.value as f64)))
    }
}
impl Div<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn div(self, rhs: VirPyFloat) -> Self::Output {
        if rhs.value == 0f64 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        Ok(VirPyFloat::new(self.value as f64 / rhs.value))
    }
}

impl Rem for VirPyInt {
    type Output = Result<VirPyInt>;
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        let mut v = (self.value) % (rhs.value);
        if v < 0 {
            v += rhs.value;
        }
        Ok(VirPyInt::new(v))
    }
}
impl Rem for VirPyFloat {
    type Output = Result<VirPyFloat>;
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.value == 0f64 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        let mut v = (self.value) % (rhs.value);
        if v < 0f64 {
            v += rhs.value;
        }
        Ok(VirPyFloat::new(v))
    }
}
impl Rem<VirPyInt> for VirPyFloat {
    type Output = Result<VirPyFloat>;
    fn rem(self, rhs: VirPyInt) -> Self::Output {
        if rhs.value == 0 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        let mut v = (self.value) % (rhs.value as f64);
        if v < 0f64 {
            v += rhs.value as f64;
        }
        Ok(VirPyFloat::new(v))
    }
}
impl Rem<VirPyFloat> for VirPyInt {
    type Output = Result<VirPyFloat>;
    fn rem(self, rhs: VirPyFloat) -> Self::Output {
        if rhs.value == 0f64 {
            return Err(crate::error::SandboxExecutionError::DivideByZeroError);
        }
        let mut v = (self.value as f64) % (rhs.value);
        if v < 0f64 {
            v += rhs.value;
        }
        Ok(VirPyFloat::new(v))
    }
}
