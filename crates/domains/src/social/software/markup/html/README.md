# HTML5 -- WHATWG HTML Ontology

Models HTML5 as an extension of the parent markup ontology: the universal node kinds are mapped to HTML5 constructs (Document, DocType, Element, Attribute, Text, Comment) and the WHATWG structural rules are enforced as category and axioms.

Key references:
- [WHATWG HTML Living Standard](https://html.spec.whatwg.org/multipage/)

## Entities

| Category | Entities |
|---|---|
| HTML node kinds (6) | Document, DocType, Element, Attribute, Text, Comment |

## Category

`HtmlCategory` has `HtmlNodeKind` as objects and `HtmlContains` as morphisms. The edge set encodes the basic HTML structural rules: `Document → {DocType, Element, Comment}`; `Element → {Element, Attribute, Text, Comment}`; plus the transitive closure `Document → {Attribute, Text}`.

## Qualities

| Quality | Type | Description |
|---|---|---|
| IsContentNode | () | Element, Text, and Comment are content nodes; Attribute, Document, and DocType are not |

## Axioms (2)

| Axiom | Description | Source |
|---|---|---|
| SingleRootElement | An HTML document must have exactly one root element | WHATWG HTML §13.2.1 |
| ProperNesting | HTML elements must be properly nested — no overlapping tags | WHATWG HTML §13.2 |

## Functors

HTML extends the parent `markup` ontology; `HtmlNode::to_markup` and `HtmlDocument::to_markup` realise the forgetful direction by projecting HTML trees onto generic `MarkupNode` trees.

## Files

- `ontology.rs` -- `HtmlNodeKind`, `HtmlContains`, `HtmlCategory`/`HtmlOntology`, `HtmlDocument`/`HtmlElement`/`HtmlAttribute`/`HtmlNode` rich types, `IsContentNode` quality, `SingleRootElement`/`ProperNesting` axioms, tests
- `tests.rs` -- structural and conversion tests
- `mod.rs` -- module declarations
- `citings.md` -- references to the WHATWG HTML Living Standard
