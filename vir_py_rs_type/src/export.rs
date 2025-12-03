/// A trait for types that can be exported to a standard Rust type `T`.
pub trait Export<T> {
    /// Performs the conversion.
    fn export(&self) -> T;
}