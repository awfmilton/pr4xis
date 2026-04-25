use super::ontology::*;

#[test]
fn test_html_to_markup_conversion() {
    let html = HtmlDocument {
        doctype: Some("html".into()),
        root: HtmlElement {
            tag: "html".into(),
            attributes: vec![("lang".into(), "en".into())],
            children: vec![
                HtmlNode::Element(HtmlElement {
                    tag: "head".into(),
                    attributes: vec![],
                    children: vec![],
                }),
                HtmlNode::Element(HtmlElement {
                    tag: "body".into(),
                    attributes: vec![],
                    children: vec![
                        HtmlNode::Element(HtmlElement {
                            tag: "h1".into(),
                            attributes: vec![],
                            children: vec![HtmlNode::Text("Hello HTML".into())],
                        }),
                        HtmlNode::Comment("A comment".into()),
                    ],
                }),
            ],
        },
    };

    let markup = html.to_markup();
    assert_eq!(
        markup.kind,
        crate::social::software::markup::NodeKind::Document
    );
    // Doctype + Root
    assert_eq!(markup.children.len(), 2);

    let doctype = &markup.children[0];
    assert_eq!(
        doctype.kind,
        crate::social::software::markup::NodeKind::ProcessingInstruction
    );
    assert_eq!(doctype.name.as_deref(), Some("DOCTYPE"));
    assert_eq!(doctype.value.as_deref(), Some("html"));

    let root = &markup.children[1];
    assert_eq!(root.name.as_deref(), Some("html"));
    assert_eq!(root.attribute("lang"), Some("en"));
    assert_eq!(root.children.len(), 2);
}

#[test]
fn test_html_axiom_validation() {
    let axiom = HtmlSingleRootElement;

    // Valid: rooted at <html>
    let valid_html = HtmlDocument {
        doctype: None,
        root: HtmlElement {
            tag: "html".into(),
            attributes: vec![],
            children: vec![],
        },
    };
    assert!(axiom.is_satisfied_by(&valid_html));

    // Invalid: rooted at <div>
    let invalid_html = HtmlDocument {
        doctype: None,
        root: HtmlElement {
            tag: "div".into(),
            attributes: vec![],
            children: vec![],
        },
    };
    assert!(!axiom.is_satisfied_by(&invalid_html));
}
