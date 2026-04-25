use alloc::string::String;

/// A trait for emitting code in a target language.
///
/// This trait provides a unified abstraction for translating validated
/// ontological structures into target-language syntax.
pub trait Emit<Target> {
    /// Emits the structure as a string in the target language.
    fn emit(&self) -> String;
}
