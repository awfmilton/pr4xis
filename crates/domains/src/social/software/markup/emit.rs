use alloc::format;
use alloc::string::String;
use pr4xis::codegen::Emit;

use super::ontology::{MarkupNode, NodeKind};

/// HTML emission target marker.
pub struct Html;

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
                let name = self.name.as_deref().unwrap_or("div");
                let mut out = format!("<{}", name);

                for (k, v) in &self.attributes {
                    out.push_str(&format!(" {}=\"{}\"", k, escape_html(v)));
                }

                if is_void_element(name) {
                    out.push_str(" />");
                } else {
                    out.push('>');
                    for child in &self.children {
                        out.push_str(&Emit::<Html>::emit(child));
                    }
                    out.push_str(&format!("</{}>", name));
                }
                out
            }
            NodeKind::Text => escape_html(self.value.as_deref().unwrap_or("")),
            NodeKind::Comment => {
                format!("<!-- {} -->", self.value.as_deref().unwrap_or(""))
            }
            NodeKind::ProcessingInstruction => {
                let name = self.name.as_deref().unwrap_or("");
                if name.to_lowercase() == "doctype" {
                    format!("<!DOCTYPE {}>", self.value.as_deref().unwrap_or("html"))
                } else {
                    format!("<?{} {}?>", name, self.value.as_deref().unwrap_or(""))
                }
            }
            NodeKind::Attribute => String::new(), // Attributes are handled in Element
        }
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn is_void_element(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
