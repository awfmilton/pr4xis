use alloc::string::String;
use core::marker::PhantomData;

/// Formal abstraction for translating validated ontological structures into target-language syntax.
///
/// Targets are implemented as marker structs to allow a single type to implement
/// emission for multiple backends (e.g., HTML, SQL, Rust).
pub trait Emit<Target> {
    /// Emit the structure as a string for the specified target.
    fn emit(&self) -> String;
}

/// A marker for types that can be emitted.
pub struct Emitter<T, Target> {
    _data: PhantomData<T>,
    _target: PhantomData<Target>,
}
