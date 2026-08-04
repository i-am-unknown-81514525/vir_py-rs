pub mod core;
mod types;

pub trait Dewrap<T> {
    fn dewrap(self) -> T;
}

impl<T> Dewrap<T> for T {
    fn dewrap(self) -> T {
        self
    }
}