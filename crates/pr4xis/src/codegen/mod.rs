#[cfg(feature = "std")]
mod builder;
pub mod emit;
#[cfg(feature = "std")]
mod generate;
#[cfg(feature = "std")]
pub mod wordnet;

#[cfg(feature = "std")]
pub use builder::{EntityDef, GenerateConfig, OntologyBuilder};
pub use emit::Emit;
#[cfg(feature = "std")]
pub use generate::generate_rust;

// Re-export CodegenData from the always-available module.
pub use crate::codegen_data::CodegenData;
