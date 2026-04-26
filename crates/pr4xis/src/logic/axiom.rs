use crate::ontology::meta::{Identifier, RelationshipMeta};
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
///     fn holds(&self) -> bool { ... }
///     pr4xis::axiom_meta!("MyAxiom");
/// }
/// ```
#[macro_export]
macro_rules! axiom_meta {
    ($id:literal) => {
        fn meta(&self) -> $crate::ontology::meta::RelationshipMeta {
            $crate::ontology::meta::RelationshipMeta::from_identifier(
                $crate::ontology::meta::Identifier::new_static($id),
            )
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
        RelationshipMeta::from_identifier(Identifier::new(tn))
    }
}
