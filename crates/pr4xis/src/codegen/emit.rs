use alloc::string::String;

/// A trait for emitting code for a specific target.
pub trait Emit<Target> {
    /// Emit the code as a string.
    fn emit(&self) -> String;
}
