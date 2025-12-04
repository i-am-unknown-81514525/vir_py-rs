use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Map;
use crate::export::Export;
use std::ops::{Add, Sub, Mul, Div, Deref};
use std::rc::Rc;
use crate::base::VirPyTypeMut;
use crate::{base};
// Definition

#[derive(Debug, Clone, Copy)]
pub struct VirPyInt {
    value: i64,
}

impl VirPyInt {
    pub fn new(value: i64) -> Self {
        VirPyInt { value }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
}

impl base::VirPyType for VirPyInt {}
impl VirPyTypeMut for VirPyInt {}


#[derive(Debug, Clone, Copy)]
pub struct VirPyFloat {
    value: f64,
}

impl VirPyFloat {
    pub fn new(value: f64) -> Self {
        VirPyFloat { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

impl base::VirPyType for VirPyFloat {}
impl base::VirPyTypeMut for VirPyFloat {}

#[derive(Debug)]
pub struct Mapping {
    mapping: HashMap<String, Rc<RefCell<Box<dyn VirPyTypeMut>>>>
}

impl Mapping { // Need memory address system later so the mapping is synced?
    pub fn new(&mut self, mapping: HashMap<String, Box<dyn VirPyTypeMut>>) {
        let mut map = HashMap::new();
        for (k, v) in mapping.iter() {
            map.insert(k.clone(), Rc::new(RefCell::new(v.clone_box_mut())));
        }
        self.mapping = map;
    }
}

impl Clone for Mapping {
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for (k, v) in self.mapping.iter() {
            map.insert(k.clone(), v.clone());
        };
        Self { mapping: map }
    }
}

impl Deref for Mapping {
    type Target = HashMap<String, Rc<RefCell<Box<dyn VirPyTypeMut>>>>;
    fn deref(&self) -> &Self::Target {
        &self.mapping
    }
}

#[derive(Debug, Clone)]
pub struct VirPyObject {
    mapping: Rc<RefCell<Mapping>>
}

impl Into<Mapping> for HashMap<String, Rc<RefCell<Box<dyn VirPyTypeMut>>>> {
    fn into(self) -> Mapping {
        Mapping { mapping: self }
    }
}

impl VirPyObject {
    pub fn new(mapping: Rc<RefCell<Mapping>>) -> Self {
        VirPyObject { mapping }
    }

    pub fn from_raw(&self, mapping: HashMap<String, Rc<RefCell<Box<dyn VirPyTypeMut>>>>) -> Self {
        return Self::new(Rc::new(RefCell::new(mapping.into())))
    }

    pub fn get_value(&self) -> Rc<RefCell<Mapping>> {
        self.mapping.clone()
    }
}

impl base::VirPyType for VirPyObject {}
impl base::VirPyTypeMut for VirPyObject {}

// Export
impl Export<i64> for VirPyInt {
    fn export(&self) -> i64 {
        self.value
    }
}

impl Export<f64> for VirPyFloat {
    fn export(&self) -> f64 {
        self.value
    }
}

// op: Add
impl Add for VirPyInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl Add for VirPyFloat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl Add<VirPyInt> for VirPyFloat {
    type Output = Self;
    fn add(self, rhs: VirPyInt) -> Self::Output {
        Self::new(self.value + rhs.value as f64)
    }
}

impl Add<VirPyFloat> for VirPyInt {
    type Output = VirPyFloat;
    fn add(self, rhs: VirPyFloat) -> Self::Output {
        VirPyFloat::new(self.value as f64 + rhs.value)
    }
}

register_op_add!(VirPyInt, VirPyInt, VirPyInt);
register_op_add!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_add!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_add!(VirPyInt, VirPyFloat, VirPyFloat);

// op: Sub

impl Sub for VirPyInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl Sub for VirPyFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl Sub<VirPyInt> for VirPyFloat {
    type Output = Self;
    fn sub(self, rhs: VirPyInt) -> Self::Output {
        Self::new(self.value - rhs.value as f64)
    }
}

impl Sub<VirPyFloat> for VirPyInt {
    type Output = VirPyFloat;
    fn sub(self, rhs: VirPyFloat) -> Self::Output {
        VirPyFloat::new(self.value as f64 - rhs.value)
    }
}

register_op_sub!(VirPyInt, VirPyInt, VirPyInt);
register_op_sub!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_sub!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_sub!(VirPyInt, VirPyFloat, VirPyFloat);


// op: Mul

impl Mul for VirPyInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl Mul for VirPyFloat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl Mul<VirPyInt> for VirPyFloat {
    type Output = Self;
    fn mul(self, rhs: VirPyInt) -> Self::Output {
        Self::new(self.value * rhs.value as f64)
    }
}

impl Mul<VirPyFloat> for VirPyInt {
    type Output = VirPyFloat;
    fn mul(self, rhs: VirPyFloat) -> Self::Output {
        VirPyFloat::new(self.value as f64 * rhs.value)
    }
}

register_op_mul!(VirPyInt, VirPyInt, VirPyInt);
register_op_mul!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_mul!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_mul!(VirPyInt, VirPyFloat, VirPyFloat);


// op: Div

impl Div for VirPyInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl Div for VirPyFloat {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl Div<VirPyInt> for VirPyFloat {
    type Output = Self;
    fn div(self, rhs: VirPyInt) -> Self::Output {
        Self::new(self.value / rhs.value as f64)
    }
}

impl Div<VirPyFloat> for VirPyInt {
    type Output = VirPyFloat;
    fn div(self, rhs: VirPyFloat) -> Self::Output {
        VirPyFloat::new(self.value as f64 / rhs.value)
    }
}

register_op_div!(VirPyInt, VirPyInt, VirPyInt);
register_op_div!(VirPyFloat, VirPyFloat, VirPyFloat);
register_op_div!(VirPyFloat, VirPyInt, VirPyFloat);
register_op_div!(VirPyInt, VirPyFloat, VirPyFloat);