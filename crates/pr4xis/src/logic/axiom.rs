use crate::ontology::meta::{Citation, Label, ModulePath, OntologyName, RelationshipMeta};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};

/// Helper: write the `meta()` method for a hand-written `impl Axiom`
/// with a literature citation in one line. Ensures every axiom announces
/// itself without boilerplate.
///
/// Issue #153: axioms share the unified [`RelationshipMeta`] shape with
/// ontologies, functors, natural transformations, and adjunctions — every
/// structural entity carries one Lemon+PROV-O record, no parallel types.
///
/// # Example
///
/// ```ignore
/// impl Axiom for MyAxiom {
///     fn description(&self) -> &str { "..." }
///     fn holds(&self) -> bool { ... }
///     pr4xis::axiom_meta!(MyAxiom, "Smith (1999) Journal of X");
/// }
/// ```
#[macro_export]
macro_rules! axiom_meta {
    // Three-argument form with kind — name, kind, description, citation.
    ($name:ident [$kind:ident], $description:literal, $citation:literal) => {
        fn meta(&self) -> $crate::ontology::meta::RelationshipMeta {
            static META: $crate::ontology::meta::RelationshipMeta =
                $crate::ontology::meta::RelationshipMeta::new_static(
                    concat!(stringify!($name), "[", stringify!($kind), "]"),
                    $description,
                    $crate::parse_citation!($citation),
                    module_path!(),
                );
            META.clone()
        }
    };
    // Two-argument form with kind — name, kind, citation.
    ($name:ident [$kind:ident], $citation:literal) => {
        fn meta(&self) -> $crate::ontology::meta::RelationshipMeta {
            static META: $crate::ontology::meta::RelationshipMeta =
                $crate::ontology::meta::RelationshipMeta::new_static(
                    concat!(stringify!($name), "[", stringify!($kind), "]"),
                    concat!(stringify!($name), "[", stringify!($kind), "]"),
                    $crate::parse_citation!($citation),
                    module_path!(),
                );
            META.clone()
        }
    };
    // Three-argument form — name, description (English label), citation.
    ($name:ident, $description:literal, $citation:literal) => {
        fn meta(&self) -> $crate::ontology::meta::RelationshipMeta {
            static META: $crate::ontology::meta::RelationshipMeta =
                $crate::ontology::meta::RelationshipMeta::new_static(
                    stringify!($name),
                    $description,
                    $crate::parse_citation!($citation),
                    module_path!(),
                );
            META.clone()
        }
    };
    // Two-argument form — name + citation, description defaults to name.
    ($name:ident, $citation:literal) => {
        fn meta(&self) -> $crate::ontology::meta::RelationshipMeta {
            static META: $crate::ontology::meta::RelationshipMeta =
                $crate::ontology::meta::RelationshipMeta::new_static(
                    stringify!($name),
                    stringify!($name),
                    $crate::parse_citation!($citation),
                    module_path!(),
                );
            META.clone()
        }
    };
}

/// An axiom — a statement that must hold unconditionally.
///
/// Axioms are foundational truths about a domain. `holds()` verifies
/// the system is consistent with the axiom — the system cannot lie.
///
/// Used by both category-level structural checks (e.g. "no dead states")
/// and domain-level invariants (e.g. "energy is conserved").
///
/// Every axiom announces itself via `meta()` — its name, citation, and
/// module path, carried in the unified [`RelationshipMeta`] that every
/// structural entity in pr4xis shares (issue #153). `description()`
/// remains as an English fallback until the lexicon resolves
/// `meta().name` into per-language labels.
pub trait Axiom {
    /// English fallback — will be replaced by Lemon lexicon lookup of `meta().name`.
    fn description(&self) -> &str;

    /// Verify this axiom holds.
    fn holds(&self) -> bool;

    /// Structured metadata — name, citation, module path.
    ///
    /// The default is an **honest placeholder** using `core::any::type_name`
    /// and an empty citation — "this axiom hasn't declared its literature
    /// citation yet"; consumers can detect and flag via `citation.is_empty()`.
    ///
    /// Axioms declared via `ontology!`'s `axioms:` clause or with the
    /// [`axiom_meta!`](crate::axiom_meta!) helper inline override the
    /// default with the actual literature reference.
    fn meta(&self) -> RelationshipMeta {
        let tn = core::any::type_name::<Self>().to_string();
        RelationshipMeta {
            name: OntologyName::new(tn.clone()),
            description: Label::new(self.description().to_string()),
            citation: Citation::EMPTY,
            module_path: ModulePath::new(module_path!().to_string()),
        }
    }
}
