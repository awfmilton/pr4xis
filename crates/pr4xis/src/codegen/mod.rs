#[cfg(feature = "codegen")]
mod builder;
mod emit;
#[cfg(feature = "codegen")]
mod generate;
#[cfg(feature = "codegen")]
pub mod wordnet;

#[cfg(feature = "codegen")]
pub use builder::{EntityDef, GenerateConfig, OntologyBuilder};
pub use emit::Emit;
#[cfg(feature = "codegen")]
pub use generate::generate_rust;

// Re-export CodegenData from the always-available module.
pub use crate::codegen_data::CodegenData;
