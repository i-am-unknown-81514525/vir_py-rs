use crate::base;
use crate::export::Export;

#[derive(Debug)]
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
impl Export<i64> for VirPyInt {
    fn export(&self) -> i64 {
        self.value
    }
}