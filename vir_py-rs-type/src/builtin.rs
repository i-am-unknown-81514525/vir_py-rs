use crate::base;

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
}

impl base::VirPyType for VirPyInt {}