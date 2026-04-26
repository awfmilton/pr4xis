#[macro_export]
macro_rules! register_lexicon {
    ($id:ident, $id_lit:literal, $name:literal, $description:literal, $citation:literal) => {
        #[cfg(not(target_arch = "wasm32"))]
        $crate::paste::paste! {
            #[$crate::linkme::distributed_slice($crate::ontology::registry::LEXICON)]
            #[linkme(crate = $crate::linkme)]
            static [<_LEXICON_ENTRY_ $id:snake:upper>]: fn() -> $crate::ontology::meta::LexicalRecord =
                || $crate::ontology::meta::LexicalRecord::new_static(
                    $id_lit,
                    $name,
                    $description,
                    $citation,
                    module_path!(),
                );
        }
    };
}
