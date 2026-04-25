#[allow(unused_imports)]
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};

use pr4xis::category::Category;
use pr4xis::category::Concept;
use pr4xis::category::relationship::Relationship;
use pr4xis::ontology::upper::being::Being;
use pr4xis::ontology::upper::classify::Classified;
use pr4xis::ontology::{Axiom, Ontology, Quality};

use super::super::ontology::MarkupNode;

// HTML5 ontology — grounded in the WHATWG HTML Living Standard.
// https://html.spec.whatwg.org/multipage/
//
// HTML (HyperText Markup Language) is the standard markup language for documents
// designed to be displayed in a web browser. This ontology defines the structural
// essence of HTML5 as a categorical structure.

/// HTML-specific node kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Concept)]
pub enum HtmlNodeKind {
    /// The HTML document itself.
    Document,
    /// The DOCTYPE declaration: `<!DOCTYPE html>`.
    DocType,
    /// An HTML element: `<div>`, `<p>`, `<span>`, etc.
    Element,
    /// An HTML attribute: `class="container"`, `id="main"`.
    Attribute,
    /// Text content within an element.
    Text,
    /// An HTML comment: `<!-- comment -->`.
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

/// The HTML category — structural rules as category laws.
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

        // Document contains Doctype, Element (root), and Comments
        m.push(HtmlContains {
            parent: Document,
            child: DocType,
        });
        m.push(HtmlContains {
            parent: Document,
            child: Element,
        });
        m.push(HtmlContains {
            parent: Document,
            child: Comment,
        });

        // Element contains other Elements, Attributes, Text, and Comments
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
        "HTML5 is a WHATWG/W3C standard social convention for web documents"
    }
}

/// An HTML element.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    pub name: String,
    pub attributes: Vec<HtmlAttribute>,
    pub children: Vec<HtmlNode>,
}

/// An HTML attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: String,
}

/// An HTML node — universal representation of HTML content.
#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element(HtmlElement),
    Text(String),
    Comment(String),
}

impl HtmlNode {
    /// Convert to the generic markup representation.
    pub fn to_markup(&self) -> MarkupNode {
        match self {
            Self::Element(elem) => {
                let attrs: Vec<(&str, &str)> = elem
                    .attributes
                    .iter()
                    .map(|a| (a.name.as_str(), a.value.as_str()))
                    .collect();
                MarkupNode::element(
                    &elem.name,
                    attrs,
                    elem.children.iter().map(|c| c.to_markup()).collect(),
                )
            }
            Self::Text(t) => MarkupNode::text(t),
            Self::Comment(t) => MarkupNode::comment(t),
        }
    }

    /// Get text content recursively.
    pub fn text_content(&self) -> String {
        match self {
            Self::Text(t) => t.clone(),
            Self::Element(elem) => elem
                .children
                .iter()
                .map(|c| c.text_content())
                .collect::<Vec<_>>()
                .join(""),
            _ => String::new(),
        }
    }
}

/// An HTML document.
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlDocument {
    pub doctype: Option<String>,
    pub root: HtmlElement,
}

impl HtmlDocument {
    /// Convert to generic markup representation.
    pub fn to_markup(&self) -> MarkupNode {
        // DocType is handled as a sibling or metadata in generic markup if needed,
        // but base MarkupNode doesn't have a specific DocType kind yet.
        MarkupNode::document(vec![HtmlNode::Element(self.root.clone()).to_markup()])
    }
}

/// HTML well-formedness: an HTML document must have exactly one root element.
pub struct SingleRootElement;

impl pr4xis::logic::Axiom for SingleRootElement {
    fn description(&self) -> &str {
        "an HTML document must have exactly one root element (the html element)"
    }

    fn holds(&self) -> bool {
        true // structural — enforced by HtmlDocument having exactly one root field
    }
}
pr4xis::register_axiom!(SingleRootElement);

/// HTML well-formedness: elements must be properly nested.
pub struct ProperNesting;

impl pr4xis::logic::Axiom for ProperNesting {
    fn description(&self) -> &str {
        "HTML elements must be properly nested — no overlapping tags"
    }

    fn holds(&self) -> bool {
        true // structural — enforced by the tree representation
    }
}
pr4xis::register_axiom!(ProperNesting);

/// Quality: is this HTML node kind a content node?
#[derive(Debug, Clone)]
pub struct IsContentNode;

impl Quality for IsContentNode {
    type Individual = HtmlNodeKind;
    type Value = ();

    fn get(&self, kind: &HtmlNodeKind) -> Option<()> {
        match kind {
            HtmlNodeKind::Element | HtmlNodeKind::Text | HtmlNodeKind::Comment => Some(()),
            _ => None,
        }
    }
}

/// The HTML ontology.
pub struct HtmlOntology;

impl Ontology for HtmlOntology {
    type Cat = HtmlCategory;
    type Qual = IsContentNode;

    fn domain_axioms() -> Vec<Box<dyn Axiom>> {
        vec![Box::new(SingleRootElement), Box::new(ProperNesting)]
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
