use alloc::format;
use alloc::string::String;
#[cfg(feature = "codegen")]
use pr4xis::codegen::Emit;

use crate::social::software::markup::ontology::{MarkupNode, NodeKind};

/// A marker struct for the HTML emission target.
pub struct Html;

#[cfg(feature = "codegen")]
impl Emit<Html> for MarkupNode {
    fn emit(&self) -> String {
        match self.kind {
            NodeKind::Document => {
                let mut out = String::new();
                for child in &self.children {
                    out.push_str(&Emit::<Html>::emit(child));
                }
                out
            }
            NodeKind::Element => {
                let name = self.name.as_deref().unwrap_or("unknown");
                let mut out = format!("<{}", name);

                for (k, v) in &self.attributes {
                    out.push_str(&format!(" {}=\"{}\"", k, v));
                }

                out.push('>');

                for child in &self.children {
                    out.push_str(&Emit::<Html>::emit(child));
                }

                out.push_str(&format!("</{}>", name));
                out
            }
            NodeKind::Text => self.value.clone().unwrap_or_default(),
            NodeKind::Comment => {
                format!("<!--{}-->", self.value.as_deref().unwrap_or_default())
            }
            NodeKind::ProcessingInstruction => {
                format!("<?{}?>", self.value.as_deref().unwrap_or_default())
            }
            NodeKind::Attribute => {
                // Attributes are typically handled in the Element variant,
                // but if someone emits an attribute node directly, we can
                // return its string representation.
                let name = self.name.as_deref().unwrap_or("unknown");
                let value = self.value.as_deref().unwrap_or_default();
                format!("{}=\"{}\"", name, value)
            }
        }
    }
}
