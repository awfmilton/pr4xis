#[allow(unused_imports)]
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};

use pr4xis::category::Category;
use pr4xis::category::Concept;
use pr4xis::category::relationship::Relationship;
use pr4xis::ontology::upper::being::Being;
use pr4xis::ontology::upper::classify::Classified;
use pr4xis::ontology::{Axiom, Ontology, Quality};

use super::super::ontology::{MarkupNode, NodeKind};

// HTML5 ontology — grounded in the WHATWG HTML Living Standard
// https://html.spec.whatwg.org/multipage/
//
// HTML5 (HyperText Markup Language) is the standard markup language
// for documents designed to be displayed in a web browser. This ontology
// defines HTML5 as a formally verified categorical structure.

/// HTML5-specific node kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Concept)]
pub enum HtmlNodeKind {
    /// The HTML document.
    Document,
    /// The DOCTYPE declaration: `<!DOCTYPE html>`.
    Doctype,
    /// An HTML element: `<div>`, `<p>`, `<span>`, etc.
    Element,
    /// An attribute: `class="foo"` on an element.
    Attribute,
    /// Text content within an element.
    Text,
    /// Comment: `<!-- ... -->`.
    Comment,
}

/// HTML5 containment relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HtmlRelation {
    pub parent: HtmlNodeKind,
    pub child: HtmlNodeKind,
}

impl Relationship for HtmlRelation {
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

/// The HTML5 category — structural rules as category laws.
pub struct HtmlCategory;

impl Category for HtmlCategory {
    type Object = HtmlNodeKind;
    type Morphism = HtmlRelation;

    fn identity(obj: &HtmlNodeKind) -> HtmlRelation {
        HtmlRelation {
            parent: *obj,
            child: *obj,
        }
    }

    fn compose(f: &HtmlRelation, g: &HtmlRelation) -> Option<HtmlRelation> {
        if f.child != g.parent {
            return None;
        }
        if f.parent == f.child {
            return Some(g.clone());
        }
        if g.parent == g.child {
            return Some(f.clone());
        }
        Some(HtmlRelation {
            parent: f.parent,
            child: g.child,
        })
    }

    fn morphisms() -> Vec<HtmlRelation> {
        use HtmlNodeKind::*;
        let mut m = Vec::new();

        // Identity
        for n in HtmlNodeKind::variants() {
            m.push(HtmlRelation {
                parent: n,
                child: n,
            });
        }

        // Document contains Doctype, Element (root), and Comment
        m.push(HtmlRelation {
            parent: Document,
            child: Doctype,
        });
        m.push(HtmlRelation {
            parent: Document,
            child: Element,
        });
        m.push(HtmlRelation {
            parent: Document,
            child: Comment,
        });

        // Element contains Element, Attribute, Text, and Comment
        m.push(HtmlRelation {
            parent: Element,
            child: Element,
        });
        m.push(HtmlRelation {
            parent: Element,
            child: Attribute,
        });
        m.push(HtmlRelation {
            parent: Element,
            child: Text,
        });
        m.push(HtmlRelation {
            parent: Element,
            child: Comment,
        });

        // Transitive closure (Document → Element → *)
        for child in [Attribute, Text] {
            m.push(HtmlRelation {
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
        "HTML5 is a WHATWG standard — an agreed-upon markup language for the web"
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

/// An HTML node.
#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element(HtmlElement),
    Text(String),
    Comment(String),
}

impl HtmlNode {
    /// Convert to generic markup representation.
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
        let mut children = Vec::new();
        if let Some(dt) = &self.doctype {
            children.push(MarkupNode {
                kind: NodeKind::ProcessingInstruction,
                name: Some("DOCTYPE".into()),
                value: Some(dt.clone()),
                attributes: Vec::new(),
                children: Vec::new(),
            });
        }
        children.push(HtmlNode::Element(self.root.clone()).to_markup());
        MarkupNode::document(children)
    }
}

/// WHATWG well-formedness axiom: an HTML document must have exactly one root element.
pub struct SingleRootElement;

impl pr4xis::logic::Axiom for SingleRootElement {
    fn description(&self) -> &str {
        "an HTML document must have exactly one root element (typically <html>)"
    }

    fn holds(&self) -> bool {
        true // structural — enforced by HtmlDocument having exactly one root field
    }
}
pr4xis::register_axiom!(SingleRootElement);

/// WHATWG well-formedness axiom: element tags must be properly nested.
pub struct ValidNesting;

impl pr4xis::logic::Axiom for ValidNesting {
    fn description(&self) -> &str {
        "HTML elements must be properly nested — no overlapping tags"
    }

    fn holds(&self) -> bool {
        true // structural — enforced by the tree representation
    }
}
pr4xis::register_axiom!(ValidNesting);

/// Quality: is this HTML node kind a content node (can appear inside elements)?
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

/// The HTML5 ontology.
pub struct HtmlOntology;

impl Ontology for HtmlOntology {
    type Cat = HtmlCategory;
    type Qual = IsContentNode;

    fn domain_axioms() -> Vec<Box<dyn Axiom>> {
        vec![Box::new(SingleRootElement), Box::new(ValidNesting)]
    }
}
