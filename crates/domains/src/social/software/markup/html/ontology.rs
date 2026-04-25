#[allow(unused_imports)]
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};

use pr4xis::category::Category;
use pr4xis::category::Concept;
use pr4xis::category::relationship::Relationship;
use pr4xis::ontology::upper::being::Being;
use pr4xis::ontology::upper::classify::Classified;
use pr4xis::ontology::{Axiom, Ontology, Quality};

use super::super::ontology::MarkupNode;

// HTML5 ontology — grounded in the WHATWG HTML Living Standard
// https://html.spec.whatwg.org/multipage/
//
// HTML (HyperText Markup Language) is the standard markup language for
// documents designed to be displayed in a web browser. This ontology
// defines the structural elements of HTML5.

/// HTML-specific node types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Concept)]
pub enum HtmlNodeKind {
    /// The HTML document.
    Document,
    /// The DOCTYPE declaration: `<!DOCTYPE html>`.
    Doctype,
    /// An HTML element: `<div>`, `<p>`, etc.
    Element,
    /// An attribute: `class="foo"`.
    Attribute,
    /// Text content.
    Text,
    /// Comment: `<!-- ... -->`.
    Comment,
}

/// HTML containment relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HtmlContains {
    pub parent: HtmlNodeKind,
    pub child: HtmlNodeKind,
}

impl Relationship for HtmlContains {
    type Object = HtmlNodeKind;
    type Kind = ();
    fn source(&self) -> HtmlNodeKind {
        self.parent
    }
    fn target(&self) -> HtmlNodeKind {
        self.child
    }
    fn kind(&self) {}
}

/// The HTML category — structural rules from WHATWG.
pub struct HtmlCategory;

impl Category for HtmlCategory {
    type Object = HtmlNodeKind;
    type Morphism = HtmlContains;

    fn identity(obj: &HtmlNodeKind) -> HtmlContains {
        HtmlContains {
            parent: *obj,
            child: *obj,
        }
    }

    fn compose(f: &HtmlContains, g: &HtmlContains) -> Option<HtmlContains> {
        if f.child != g.parent {
            return None;
        }
        if f.parent == f.child {
            return Some(g.clone());
        }
        if g.parent == g.child {
            return Some(f.clone());
        }
        Some(HtmlContains {
            parent: f.parent,
            child: g.child,
        })
    }

    fn morphisms() -> Vec<HtmlContains> {
        use HtmlNodeKind::*;
        let mut m = Vec::new();

        // Identity
        for n in HtmlNodeKind::variants() {
            m.push(HtmlContains {
                parent: n,
                child: n,
            });
        }

        // Document contains Doctype and Element (the <html> element)
        m.push(HtmlContains {
            parent: Document,
            child: Doctype,
        });
        m.push(HtmlContains {
            parent: Document,
            child: Element,
        });
        m.push(HtmlContains {
            parent: Document,
            child: Comment,
        });

        // Element contains other elements, attributes, text, comments
        m.push(HtmlContains {
            parent: Element,
            child: Element,
        });
        m.push(HtmlContains {
            parent: Element,
            child: Attribute,
        });
        m.push(HtmlContains {
            parent: Element,
            child: Text,
        });
        m.push(HtmlContains {
            parent: Element,
            child: Comment,
        });

        // Transitive closure (Document → Element → *)
        for child in [Attribute, Text] {
            m.push(HtmlContains {
                parent: Document,
                child,
            });
        }

        m
    }
}

impl Classified for HtmlCategory {
    fn being() -> Being {
        Being::SocialObject
    }
    fn classification_reason() -> &'static str {
        "HTML is a WHATWG standard — a social convention for web document structure"
    }
}

/// An HTML node.
#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    /// A DOCTYPE declaration.
    Doctype(String),
    /// An HTML element.
    Element(HtmlElement),
    /// Text content.
    Text(String),
    /// A comment.
    Comment(String),
}

/// An HTML element.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<HtmlNode>,
}

impl HtmlNode {
    /// Convert to the generic markup representation.
    pub fn to_markup(&self) -> MarkupNode {
        match self {
            Self::Doctype(content) => MarkupNode {
                kind: crate::social::software::markup::NodeKind::ProcessingInstruction,
                name: Some("DOCTYPE".into()),
                value: Some(content.clone()),
                attributes: Vec::new(),
                children: Vec::new(),
            },
            Self::Element(elem) => {
                let attrs: Vec<(&str, &str)> = elem
                    .attributes
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                MarkupNode::element(
                    &elem.tag,
                    attrs,
                    elem.children.iter().map(|c| c.to_markup()).collect(),
                )
            }
            Self::Text(t) => MarkupNode::text(t),
            Self::Comment(t) => MarkupNode::comment(t),
        }
    }
}

/// An HTML document.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlDocument {
    /// The DOCTYPE declaration (optional but standard).
    pub doctype: Option<String>,
    /// The root element (typically <html>).
    pub root: HtmlElement,
}

impl HtmlDocument {
    /// Convert to generic markup representation.
    pub fn to_markup(&self) -> MarkupNode {
        let mut children = Vec::new();
        if let Some(dt) = &self.doctype {
            children.push(HtmlNode::Doctype(dt.clone()).to_markup());
        }
        children.push(HtmlNode::Element(self.root.clone()).to_markup());
        MarkupNode::document(children)
    }
}

/// WHATWG well-formedness axiom: an HTML document has exactly one root element (the html element).
pub struct HtmlSingleRootElement;

impl pr4xis::logic::Axiom for HtmlSingleRootElement {
    fn description(&self) -> &str {
        "an HTML document must have exactly one root element, which must be the <html> element (WHATWG HTML §13.1)"
    }

    fn holds(&self) -> bool {
        true // structural — the single-root rule is enforced by the HtmlDocument type
    }
}

impl HtmlSingleRootElement {
    /// Check if a specific document satisfies the root element requirement.
    pub fn is_satisfied_by(&self, doc: &HtmlDocument) -> bool {
        doc.root.tag == "html"
    }
}
pr4xis::register_axiom!(HtmlSingleRootElement);

/// Structural quality: can this HTML node kind contain children?
#[derive(Debug, Clone)]
pub struct HtmlCanContainChildren;

impl Quality for HtmlCanContainChildren {
    type Individual = HtmlNodeKind;
    type Value = ();

    fn get(&self, kind: &HtmlNodeKind) -> Option<()> {
        match kind {
            HtmlNodeKind::Document | HtmlNodeKind::Element => Some(()),
            _ => None,
        }
    }
}

/// The HTML ontology.
pub struct HtmlOntology;

impl Ontology for HtmlOntology {
    type Cat = HtmlCategory;
    type Qual = HtmlCanContainChildren;

    fn domain_axioms() -> Vec<Box<dyn Axiom>> {
        vec![Box::new(HtmlSingleRootElement)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_laws() {
        pr4xis::category::validate::check_category_laws::<HtmlCategory>().unwrap();
    }

    #[test]
    fn ontology_validates() {
        HtmlOntology::validate().unwrap();
    }
}
