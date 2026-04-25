#[cfg(feature = "codegen")]
pub mod emit;
pub mod ontology;
pub mod xml;

pub use ontology::*;

#[cfg(test)]
mod tests;
