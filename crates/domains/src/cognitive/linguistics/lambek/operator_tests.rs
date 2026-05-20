//! Tests for the math-operator wiring added in the Phase 3 linguistics work
//! (`svo::infix_operator`, `svo::interrogative_determiner`, the `Operator`
//! lexical entry, its pregroup mapping, and operator tokenization).
//!
//! These live in a dedicated module so the existing `tests.rs` is left
//! untouched. They use an inline LMF lexicon (no Git-LFS WordNet dependency)
//! so they run in any environment.

#![allow(unused_imports)]

use alloc::{string::ToString, vec, vec::Vec};

use super::pregroup::{BasicType, PregroupElement, PregroupType};
use super::reduce::{TypedToken, reduce_sequence};
use super::tokenize;
use super::types::{LambekType, reduce, svo};
use crate::cognitive::linguistics::english::English;
use crate::cognitive::linguistics::language::{Language, lexical_entry_to_pregroup};
use crate::cognitive::linguistics::lexicon::pos::{LexicalEntry, Operator, PosTag};
use crate::social::software::markup::xml::lmf;

/// Math operators that Phase 3b added to the embedded English lexicon.
const MATH_OPERATORS: [&str; 5] = ["+", "-", "*", "/", "="];

/// Minimal English language for tokenizer tests. Function words (including the
/// operators) are built automatically by `English::from_wordnet`; the WordNet
/// here only needs a couple of content words.
fn sample_lang() -> English {
    let wn = lmf::reader::read_wordnet(SAMPLE_LMF).unwrap();
    English::from_wordnet(&wn)
}

const SAMPLE_LMF: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<LexicalResource>
  <Lexicon id="test" label="Test" language="en" email="" license="" version="1.0" url="">
    <LexicalEntry id="e-dog-n"><Lemma writtenForm="dog" partOfSpeech="n"/><Sense id="d1" synset="s-dog"/></LexicalEntry>
    <LexicalEntry id="e-cat-n"><Lemma writtenForm="cat" partOfSpeech="n"/><Sense id="c1" synset="s-cat"/></LexicalEntry>
    <Synset id="s-dog" partOfSpeech="n" members="e-dog-n"><Definition>a domesticated carnivore</Definition></Synset>
    <Synset id="s-cat" partOfSpeech="n" members="e-cat-n"><Definition>a small feline</Definition></Synset>
  </Lexicon>
</LexicalResource>"#;

// =============================================================================
// Type-structure tests — document the exact Lambek types the feature defines.
// =============================================================================

#[test]
fn infix_operator_has_n_n_over_n_type() {
    // (N\N)/N — takes an N on the right, then an N on the left, yields N.
    let expected = LambekType::right_div(
        LambekType::left_div(LambekType::n(), LambekType::n()),
        LambekType::n(),
    );
    assert_eq!(svo::infix_operator(), expected);
}

#[test]
fn interrogative_determiner_has_np_over_n_type() {
    // NP/N — "what" in "what dog".
    let expected = LambekType::right_div(LambekType::np(), LambekType::n());
    assert_eq!(svo::interrogative_determiner(), expected);
}

// =============================================================================
// Reduction tests — the types actually combine the way an SVO grammar needs.
// =============================================================================

#[test]
fn infix_operator_applies_to_right_argument() {
    // (N\N)/N + N → N\N  (forward application; "+ 2" awaits a left operand)
    let result = reduce(&svo::infix_operator(), &LambekType::n());
    assert_eq!(
        result,
        Some(LambekType::left_div(LambekType::n(), LambekType::n()))
    );
}

#[test]
fn infix_operator_full_expression_reduces_to_noun() {
    // N + (N\N)/N + N → N   ("2 + 2" at the type level)
    let tokens = vec![
        TypedToken {
            word: "2".into(),
            lambek_type: LambekType::n(),
        },
        TypedToken {
            word: "+".into(),
            lambek_type: svo::infix_operator(),
        },
        TypedToken {
            word: "2".into(),
            lambek_type: LambekType::n(),
        },
    ];
    // It collapses to a single N (a noun phrase / value), not a sentence S,
    // so `success` (which means "is a complete sentence") stays false.
    let result = reduce_sequence(&tokens);
    assert_eq!(result.remaining.len(), 1, "expression should fully reduce");
    assert_eq!(result.final_type, Some(LambekType::n()));
    assert!(!result.success, "an arithmetic value is an N, not a sentence");
}

#[test]
fn interrogative_determiner_combines_with_noun() {
    // NP/N + N → NP  ("what" + "dog" → NP)
    let result = reduce(&svo::interrogative_determiner(), &LambekType::n());
    assert_eq!(result, Some(LambekType::np()));
}

// =============================================================================
// Tokenizer tests — operators survive punctuation stripping and get typed.
// =============================================================================

#[test]
fn each_math_operator_tokenizes_as_infix_operator() {
    let lang = sample_lang();
    for op in MATH_OPERATORS {
        let text = alloc::format!("3 {op} 4");
        let tokens = tokenize::tokenize(&text, &lang);
        assert_eq!(tokens.len(), 3, "`{text}` should yield three tokens");
        assert_eq!(tokens[1].word, op, "middle token should be the operator");
        assert_eq!(
            tokens[1].lambek_type,
            svo::infix_operator(),
            "operator `{op}` should be typed as an infix operator",
        );
    }
}

#[test]
fn operator_survives_trailing_sentence_punctuation() {
    // The Phase 3 change stops the tokenizer from stripping +-*/= as
    // punctuation, while still trimming a trailing "?".
    let lang = sample_lang();
    let tokens = tokenize::tokenize("2 + 2?", &lang);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].word, "2");
    assert_eq!(tokens[1].word, "+");
    assert_eq!(tokens[2].word, "2");
    assert_eq!(tokens[1].lambek_type, svo::infix_operator());
}

// =============================================================================
// Lexicon / annotation tests — operators are first-class lexical entries.
// =============================================================================

#[test]
fn operators_are_in_the_embedded_english_lexicon() {
    let lang = sample_lang();
    for op in MATH_OPERATORS {
        let entry = lang
            .lexical_lookup(op)
            .unwrap_or_else(|| panic!("`{op}` should be in the lexicon"));
        match &entry {
            LexicalEntry::Operator(o) => assert_eq!(o.text, op),
            other => panic!("`{op}` should be an Operator entry, got {other:?}"),
        }
        assert_eq!(entry.pos_tag(), PosTag::Operator);
    }
}

#[test]
fn operator_maps_to_sentence_connective_pregroup() {
    // s · s^l · s^l — the connective pregroup the feature assigns operators.
    let entry = LexicalEntry::Operator(Operator { text: "+".into() });
    let expected = PregroupType::new(vec![
        PregroupElement::basic(BasicType::S),
        PregroupElement::left_adj(BasicType::S),
        PregroupElement::left_adj(BasicType::S),
    ]);
    assert_eq!(lexical_entry_to_pregroup(&entry), expected);
}
