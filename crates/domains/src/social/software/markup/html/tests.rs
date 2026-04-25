use super::super::ontology::NodeKind;
use super::*;

#[test]
fn test_html_to_markup() {
    let html = HtmlDocument {
        doctype: Some("html".into()),
        root: HtmlElement {
            name: "html".into(),
            attributes: vec![HtmlAttribute {
                name: "lang".into(),
                value: "en".into(),
            }],
            children: vec![
                HtmlNode::Element(HtmlElement {
                    name: "head".into(),
                    attributes: Vec::new(),
                    children: Vec::new(),
                }),
                HtmlNode::Element(HtmlElement {
                    name: "body".into(),
                    attributes: Vec::new(),
                    children: vec![
                        HtmlNode::Element(HtmlElement {
                            name: "h1".into(),
                            attributes: Vec::new(),
                            children: vec![HtmlNode::Text("Hello HTML5".into())],
                        }),
                        HtmlNode::Comment("A comment".into()),
                    ],
                }),
            ],
        },
    };

    let markup = html.to_markup();
    assert_eq!(markup.kind, NodeKind::Document);
    assert_eq!(markup.children.len(), 1);

    let root = &markup.children[0];
    assert_eq!(root.kind, NodeKind::Element);
    assert_eq!(root.name.as_deref(), Some("html"));
    assert_eq!(root.attribute("lang"), Some("en"));
    assert_eq!(root.children.len(), 2);

    let body = &root.children[1];
    assert_eq!(body.name.as_deref(), Some("body"));
    assert_eq!(body.children.len(), 2);

    let h1 = &body.children[0];
    assert_eq!(h1.name.as_deref(), Some("h1"));
    assert_eq!(h1.text_content(), "Hello HTML5");

    let comment = &body.children[1];
    assert_eq!(comment.kind, NodeKind::Comment);
    assert_eq!(comment.value.as_deref(), Some("A comment"));
}

#[test]
fn test_html_text_content() {
    let node = HtmlNode::Element(HtmlElement {
        name: "p".into(),
        attributes: Vec::new(),
        children: vec![
            HtmlNode::Text("Part 1 ".into()),
            HtmlNode::Element(HtmlElement {
                name: "span".into(),
                attributes: Vec::new(),
                children: vec![HtmlNode::Text("Part 2".into())],
            }),
        ],
    });

    assert_eq!(node.text_content(), "Part 1 Part 2");
}
