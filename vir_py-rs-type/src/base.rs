use std::any::Any;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};


pub trait VirPyType: Any + Debug {}


pub trait VirPyTypeMut: VirPyType {}

#[derive(Debug)]
pub struct ValueContainer {
    inner: Box<dyn VirPyTypeMut>,
}

impl ValueContainer {
    pub fn downcast_ref<T: VirPyType>(&self) -> Option<&T> {
        (self.inner.as_ref() as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: VirPyTypeMut>(&mut self) -> Option<&mut T> {
        (self.inner.as_mut() as &mut dyn Any).downcast_mut::<T>()
    }
}

impl Deref for ValueContainer {
    type Target = dyn VirPyTypeMut;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for ValueContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}