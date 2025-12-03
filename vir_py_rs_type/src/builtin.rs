use crate::{base, register_op_add, register_op_sub};
use crate::export::Export;
use std::ops::{Add, Sub};

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
impl base::VirPyTypeMut for VirPyInt {}


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
