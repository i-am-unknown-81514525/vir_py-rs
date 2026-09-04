use core::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[macro_export]
macro_rules! multistate_tree_new {
    ($x:expr $(,)?) => {
        $crate::multistate::Tree {
            left: $x,
            right: ()
        }
    };
    ($left:expr, $($tail:expr),+ $(,)?) => {
        $crate::multistate::Tree {
            left: $left,
            right: $crate::multistate_tree_new!($($tail),+),
        }
    }
}

pub struct Succ<T>(PhantomData<T>);


pub struct Tree<L, R> {
    pub left: L,
    pub right: R,
}

pub trait Select<Target, Index> {
    fn select(self) -> Target;
    fn select_ref(&self) -> &Target;
    fn select_mut(&mut self) -> &mut Target;
}

pub trait Query<Target> {
    fn query(self) -> Target;
    fn query_ref(&self) -> &Target;
    fn query_mut(&mut self) -> &mut Target;
}

impl<Head, Tail> Select<Head, ()> for Tree<Head, Tail> {
    #[inline(always)]
    fn select(self) -> Head {
        self.left
    }

    #[inline(always)]
    fn select_ref(&self) -> &Head {
        &self.left
    }

    #[inline(always)]
    fn select_mut(&mut self) -> &mut Head {
        &mut self.left
    }
}

impl<Head, Tail, Target, Prev> Select<Target, Succ<Prev>> for Tree<Head, Tail>
where Tail: Select<Target, Prev> {
    #[inline(always)]
    fn select(self) -> Target {
        self.right.select()
    }

    #[inline(always)]
    fn select_ref(&self) -> &Target {
        self.right.select_ref()
    }

    #[inline(always)]
    fn select_mut(&mut self) -> &mut Target {
        self.right.select_mut()
    }
}

pub trait IntoTree<X> {
    fn into_tree(self) -> Tree<X, ()>;
}


impl<X> IntoTree<X> for X {
    fn into_tree(self) -> Tree<X, ()> {
        Tree { left: self, right: (), }
    }
}

pub trait Extend<X, Y, Z> {
    type Output;
    fn extend(self, value: Z) -> Self::Output;
}

impl<X, Y, Z> Extend<X, Y, Z> for Tree<X, Y> {
    type Output = Tree<Z, Tree<X, Y>>;
    fn extend(self, value: Z) -> Self::Output {
        Tree { left: value, right: self }
    }
}

impl<X: Clone, Y: Clone> Clone for Tree<X, Y> {
    fn clone(&self) -> Self {
        Tree { left: self.left.clone(), right: self.right.clone() }
    }
}

impl<X: Copy, Y: Copy> Copy for Tree<X, Y> {}

impl<X: PartialEq, Y: PartialEq> PartialEq for Tree<X, Y> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl<X: Eq, Y: Eq> Eq for Tree<X, Y> {}

impl<X: Debug, Y: Debug> Debug for Tree<X, Y> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "virtual_exec_core::multistate::Tree {{left: {:?}, right: {:?}}}", self.left, self.right)
    }
}