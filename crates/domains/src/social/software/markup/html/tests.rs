use super::ontology::*;

#[test]
fn test_html_to_markup_conversion() {
    let html = HtmlDocument {
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
    assert_eq!(markup.kind, crate::social::software::markup::NodeKind::Document);
    assert_eq!(markup.children.len(), 1);

    let root = &markup.children[0];
    assert_eq!(root.name.as_deref(), Some("html"));
    assert_eq!(root.attribute("lang"), Some("en"));
    assert_eq!(root.children.len(), 2);
}
