use crate::social::software::markup::ontology::{MarkupNode, NodeKind};
use alloc::string::String;
use pr4xis::codegen::Emit;

/// Marker type for HTML emission.
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
                let mut out = String::new();
                out.push('<');
                out.push_str(name);

                for (key, value) in &self.attributes {
                    out.push(' ');
                    out.push_str(key);
                    out.push_str("=\"");
                    out.push_str(&escape_html(value));
                    out.push('\"');
                }

                if is_void_element(name) {
                    out.push_str(" />");
                } else {
                    out.push('>');
                    for child in &self.children {
                        out.push_str(&Emit::<Html>::emit(child));
                    }
                    out.push_str("</");
                    out.push_str(name);
                    out.push('>');
                }
                out
            }
            NodeKind::Text => escape_html(self.value.as_deref().unwrap_or_default()),
            NodeKind::Comment => {
                let mut out = String::from("<!-- ");
                out.push_str(self.value.as_deref().unwrap_or_default());
                out.push_str(" -->");
                out
            }
            NodeKind::Attribute => String::new(), // Attributes are handled in Element
            NodeKind::ProcessingInstruction => String::new(), // Not common in HTML
        }
    }
}

fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::social::software::markup::ontology::MarkupNode;

    #[test]
    fn test_html_emission() {
        let node = MarkupNode::element(
            "div",
            vec![("class", "container"), ("id", "main")],
            vec![
                MarkupNode::element("h1", vec![], vec![MarkupNode::text("Hello World")]),
                MarkupNode::comment("This is a comment"),
                MarkupNode::element("br", vec![], vec![]),
                MarkupNode::text("Some & text"),
            ],
        );

        let doc = MarkupNode::document(vec![node]);
        let output = Emit::<Html>::emit(&doc);

        assert_eq!(
            output,
            "<div class=\"container\" id=\"main\"><h1>Hello World</h1><!-- This is a comment --><br />Some &amp; text</div>"
        );
    }
}
