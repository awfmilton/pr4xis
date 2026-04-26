#[allow(unused_imports)]
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};

use pr4xis::category::validate::check_category_laws;

use super::ontology::*;

// =============================================================================
// Markup Category tests
// =============================================================================

#[test]
fn markup_category_laws() {
    check_category_laws::<MarkupCategory>().unwrap();
}

#[test]
fn document_contains_element() {
    let morphisms = MarkupCategory::morphisms();
    assert!(morphisms.contains(&Contains {
        parent: NodeKind::Document,
        child: NodeKind::Element,
    }));
}

#[test]
fn element_contains_text() {
    let morphisms = MarkupCategory::morphisms();
    assert!(morphisms.contains(&Contains {
        parent: NodeKind::Element,
        child: NodeKind::Text,
    }));
}

#[test]
fn element_contains_attribute() {
    let morphisms = MarkupCategory::morphisms();
    assert!(morphisms.contains(&Contains {
        parent: NodeKind::Element,
        child: NodeKind::Attribute,
    }));
}

// =============================================================================
// MarkupNode tests
// =============================================================================

#[test]
fn build_simple_document() {
    let doc = MarkupNode::document(vec![MarkupNode::element(
        "root",
        vec![],
        vec![MarkupNode::element(
            "child",
            vec![("id", "1")],
            vec![MarkupNode::text("hello")],
        )],
    )]);

    assert_eq!(doc.kind, NodeKind::Document);
    assert_eq!(doc.node_count(), 4); // doc + root + child + text
    assert_eq!(doc.depth(), 3);
}

#[test]
fn find_elements() {
    let doc = MarkupNode::document(vec![MarkupNode::element(
        "root",
        vec![],
        vec![
            MarkupNode::element("item", vec![], vec![MarkupNode::text("a")]),
            MarkupNode::element("item", vec![], vec![MarkupNode::text("b")]),
            MarkupNode::element(
                "other",
                vec![],
                vec![MarkupNode::element(
                    "item",
                    vec![],
                    vec![MarkupNode::text("c")],
                )],
            ),
        ],
    )]);

    let items = doc.find_all("item");
    assert_eq!(items.len(), 3);
}

#[test]
fn text_content() {
    let doc = MarkupNode::element(
        "p",
        vec![],
        vec![MarkupNode::text("hello "), MarkupNode::text("world")],
    );
    assert_eq!(doc.text_content(), "hello world");
}

#[test]
fn attribute_lookup() {
    let elem = MarkupNode::element("div", vec![("class", "main"), ("id", "top")], vec![]);
    assert_eq!(elem.attribute("class"), Some("main"));
    assert_eq!(elem.attribute("id"), Some("top"));
    assert_eq!(elem.attribute("missing"), None);
}

#[test]
fn well_formed_document() {
    let doc = MarkupNode::document(vec![MarkupNode::element("root", vec![], vec![])]);
    assert!(is_well_formed(&doc));
}

#[test]
fn not_well_formed_no_root() {
    let doc = MarkupNode::document(vec![MarkupNode::comment("just a comment")]);
    assert!(!is_well_formed(&doc));
}

#[test]
fn not_well_formed_multiple_roots() {
    let doc = MarkupNode::document(vec![
        MarkupNode::element("a", vec![], vec![]),
        MarkupNode::element("b", vec![], vec![]),
    ]);
    assert!(!is_well_formed(&doc));
}

#[test]
fn comment_is_preserved() {
    let node = MarkupNode::comment("this is a comment");
    assert_eq!(node.kind, NodeKind::Comment);
    assert_eq!(node.value.as_deref(), Some("this is a comment"));
}

#[cfg(feature = "codegen")]
mod emission {
    use super::*;
    use crate::social::software::markup::emit::Html;
    use pr4xis::codegen::Emit;

    #[test]
    fn emit_simple_html() {
        let node = MarkupNode::element(
            "div",
            vec![("class", "foo")],
            vec![
                MarkupNode::text("hello"),
                MarkupNode::element("span", vec![], vec![MarkupNode::text("world")]),
            ],
        );
        let html = Emit::<Html>::emit(&node);
        assert_eq!(html, "<div class=\"foo\">hello<span>world</span></div>");
    }

    #[test]
    fn emit_void_element() {
        let node = MarkupNode::element("img", vec![("src", "foo.png")], vec![]);
        let html = Emit::<Html>::emit(&node);
        assert_eq!(html, "<img src=\"foo.png\" />");
    }

    #[test]
    fn emit_escaping() {
        let node = MarkupNode::element(
            "p",
            vec![("title", "a & b")],
            vec![MarkupNode::text("<script>alert('hi')</script>")],
        );
        let html = Emit::<Html>::emit(&node);
        assert_eq!(
            html,
            "<p title=\"a &amp; b\">&lt;script&gt;alert(&#39;hi&#39;)&lt;/script&gt;</p>"
        );
    }

    #[test]
    fn emit_document_with_doctype() {
        let doc = MarkupNode::document(vec![
            MarkupNode {
                kind: NodeKind::ProcessingInstruction,
                name: Some("doctype".into()),
                value: Some("html".into()),
                attributes: vec![],
                children: vec![],
            },
            MarkupNode::element(
                "html",
                vec![],
                vec![MarkupNode::element(
                    "body",
                    vec![],
                    vec![MarkupNode::text("hi")],
                )],
            ),
        ]);
        let html = Emit::<Html>::emit(&doc);
        assert_eq!(html, "<!DOCTYPE html><html><body>hi</body></html>");
    }

    #[test]
    fn emit_comment() {
        let node = MarkupNode::comment("secret");
        let html = Emit::<Html>::emit(&node);
        assert_eq!(html, "<!-- secret -->");
    }
}

use pr4xis::category::Category;

// =============================================================================
// Property-based tests
// =============================================================================

mod prop {
    use super::*;
    use proptest::prelude::*;

    fn arb_node_kind() -> impl Strategy<Value = NodeKind> {
        prop_oneof![
            Just(NodeKind::Document),
            Just(NodeKind::Element),
            Just(NodeKind::Attribute),
            Just(NodeKind::Text),
            Just(NodeKind::Comment),
            Just(NodeKind::ProcessingInstruction),
        ]
    }

    proptest! {
        /// Every node kind has an identity morphism.
        #[test]
        fn prop_identity_exists(kind in arb_node_kind()) {
            let id = MarkupCategory::identity(&kind);
            prop_assert_eq!(id.parent, kind);
            prop_assert_eq!(id.child, kind);
        }

        /// Document transitively contains all node kinds.
        #[test]
        fn prop_document_contains_all(kind in arb_node_kind()) {
            let morphisms = MarkupCategory::morphisms();
            let expected = Contains { parent: NodeKind::Document, child: kind };
            prop_assert!(morphisms.contains(&expected));
        }

        /// Element can contain other Elements (recursive nesting).
        #[test]
        fn prop_element_nests(_dummy in 0..1i32) {
            let morphisms = MarkupCategory::morphisms();
            let self_contain = Contains { parent: NodeKind::Element, child: NodeKind::Element };
            prop_assert!(morphisms.contains(&self_contain));
        }

        /// Leaf nodes (Text, Attribute, Comment) don't contain Elements.
        #[test]
        fn prop_leaves_dont_contain_elements(kind in arb_node_kind()) {
            if matches!(kind, NodeKind::Text | NodeKind::Attribute | NodeKind::Comment) {
                let morphisms = MarkupCategory::morphisms();
                let contains_element = morphisms.iter().any(|m|
                    m.parent == kind && m.child == NodeKind::Element);
                prop_assert!(!contains_element,
                    "{:?} should not contain Elements", kind);
            }
        }

        /// Identity composition is idempotent.
        #[test]
        fn prop_identity_idempotent(kind in arb_node_kind()) {
            let id = MarkupCategory::identity(&kind);
            let composed = MarkupCategory::compose(&id, &id);
            prop_assert_eq!(composed, Some(id));
        }

        /// Node count of a tree is always >= 1.
        #[test]
        fn prop_node_count_positive(depth in 0..3usize) {
            let node = if depth == 0 {
                MarkupNode::text("leaf")
            } else {
                let children: Vec<MarkupNode> = (0..depth)
                    .map(|i| MarkupNode::text(&format!("child{}", i)))
                    .collect();
                MarkupNode::element("parent", vec![], children)
            };
            prop_assert!(node.node_count() >= 1);
        }

        /// Text content of a text node equals its value.
        #[test]
        fn prop_text_roundtrip(s in "[a-z]{1,20}") {
            let node = MarkupNode::text(&s);
            prop_assert_eq!(node.text_content(), s);
        }
    }
}
