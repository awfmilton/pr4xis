#[cfg(test)]
mod tests {
    use crate::social::software::markup::html::*;
    use crate::social::software::markup::ontology::NodeKind;
    use pr4xis::ontology::Ontology;

    #[test]
    fn category_laws() {
        pr4xis::category::validate::check_category_laws::<HtmlCategory>().unwrap();
    }

    #[test]
    fn ontology_validates() {
        HtmlOntology::validate().unwrap();
    }

    #[test]
    fn html_to_markup_conversion() {
        let doc = HtmlDocument {
            doctype: Some("html".into()),
            root: HtmlElement {
                name: "html".into(),
                attributes: vec![HtmlAttribute {
                    name: "lang".into(),
                    value: "en".into(),
                }],
                children: vec![HtmlNode::Element(HtmlElement {
                    name: "body".into(),
                    attributes: Vec::new(),
                    children: vec![HtmlNode::Text("Hello, world!".into())],
                })],
            },
        };

        let markup = doc.to_markup();
        assert_eq!(markup.kind, NodeKind::Document);
        assert_eq!(markup.children.len(), 2); // Doctype + html element
        assert_eq!(markup.children[1].name.as_deref(), Some("html"));
    }
}
