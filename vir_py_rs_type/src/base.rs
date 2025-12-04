use std::any::Any;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use bumpalo::Bump;
use crate::export::Export;

pub trait DynClone {
    fn clone_box<'a>(&self) -> Box<dyn VirPyType + 'a> where Self: 'a;
}

impl<T> DynClone for T
where
    T: Clone + VirPyType + 'static,
{
    fn clone_box<'a>(&self) -> Box<dyn VirPyType + 'a> where Self: 'a {
        Box::new(self.clone())
    }

}

pub trait DynCloneMut {
    fn clone_box_mut<'a>(&self) -> Box<dyn VirPyTypeMut + 'a> where Self: 'a;
}

impl<T> DynCloneMut for T
where 
    T: Clone + VirPyTypeMut + 'static,
{
    fn clone_box_mut<'a>(&self) -> Box<dyn VirPyTypeMut + 'a> where Self: 'a {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn VirPyType + 'a> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<'a> Clone for Box<dyn VirPyTypeMut + 'a> {
    fn clone(&self) -> Self {
        self.clone_box_mut()
    }
}

pub trait VirPyType: Any + Debug + DynClone {}


pub trait VirPyTypeMut: VirPyType + DynCloneMut {}

pub struct ValueContainer<'ctx> {
    inner: &'ctx mut dyn VirPyTypeMut,
}

impl<'ctx> ValueContainer<'ctx> {
    pub fn new<T>(value: T, arena: &'ctx Bump) -> Self
    where
        T: VirPyTypeMut + 'ctx,
    {
        Self { inner: arena.alloc(value) }
    }

    pub fn downcast_ref<T: VirPyType>(&self) -> Option<&T> {
        (self.inner as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: VirPyTypeMut>(&mut self) -> Option<&mut T> {
        (self.inner as &mut dyn Any).downcast_mut::<T>()
    }

    pub fn export<U, T>(&self) -> Option<T>
    where
        U: VirPyType + Export<T>,
    {
        self.downcast_ref::<U>().map(|val| val.export())
    }
}

impl<'ctx> Debug for ValueContainer<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueContainer")
            .field("inner", &*self.inner)
            .finish()
    }
}

impl<'ctx> Deref for ValueContainer<'ctx> {
    type Target = dyn VirPyTypeMut;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'ctx> DerefMut for ValueContainer<'ctx> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}