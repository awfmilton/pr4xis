# Teaching pr4xis New Capabilities: Programming Languages and LLM CLI Command Grammars

## Overview

This report describes a concrete, repeatable process for adding new capabilities to pr4xis, specifically:

- Encoding the structural rules of a programming language (for example, PHP, Rust, or HTML) so that pr4xis can reason about programs in that language.
- Encoding the command grammars and invariants of large language model (LLM) CLIs (for example, Gemini or Claude command-line tools) so that pr4xis can reason about whether proposed invocations are valid.

The process is grounded in pr4xis’s actual architecture: extensions are written as **ontologies** defined in Rust using macros exposed by the `pr4xis` crate (such as `define_ontology!`), not by “feeding docs into a context window” in the style of an LLM.[^1]

## Mental Model: What a Capability Is in pr4xis

pr4xis is not an LLM. It is a reasoning engine built on category theory and explicit axioms.

- Domains are modeled as **categories** in the mathematical sense, with objects, morphisms, identity, and composition.
- Ontologies are implemented in Rust and generated via macros like `define_ontology!`, which sits alongside category macros in the `pr4xis` crate.[^1]
- Reasoning proceeds by deriving new claims from accepted axioms; there is no next-token prediction, context window, or temperature.

A “capability” in this framework is:

- A category whose objects are domain entities and whose morphisms are domain relationships.
- A set of reasoning systems over those morphisms (taxonomy, mereology, causation, opposition, etc.).
- A set of structural axioms (implied by the ontology definition) and domain axioms (explicit `Precondition`/`Axiom` implementations) that the engine uses to determine which states and actions are allowed.

Nothing is “learned” until these structures compile and all category/functor law tests pass.

## High-Level 9-Step Process (Used for *Any* New Capability)

The same high-level process applies whether encoding a programming language or an LLM CLI.

1. **State the capability as a one-sentence theorem.** Describe, in one sentence, how the domain’s structure forms a category (objects, morphisms, axioms). If you cannot do this, the domain is not yet decomposed into first principles.
2. **Choose an authoritative, finite source.** For a language, use the official language reference or standard. For a CLI, use its `--help` output and official auth/usage docs. Avoid blogs and summaries.
3. **Extract entities, relations, axioms, and qualities.** From the source, list concepts (entities), `is-a` relations (taxonomy), `part-of` relations (mereology), causal or opposed pairs, explicit invariants (candidate axioms), quantities, and context-dependent names.
4. **Scaffold the ontology directory.** Under `crates/domains/src/<branch>/<topic>/`, create `ontology.rs`, `tests.rs`, `mod.rs`, `citings.md`, and a `papers/` directory containing the source material.
5. **Write the `define_ontology!` block.** Use the ontology macro (and associated entity/relation types) to define entities, taxonomy, mereology, and any other reasoning systems needed.[^1]
6. **Add domain axioms.** Implement `Precondition`/`Axiom` types that encode the source’s invariants, with total `check` methods and property-based tests.
7. **Verify category laws for the new ontology.** Run targeted `cargo test` commands for the new domain’s tests and the global category-law validator.
8. **Compose with existing ontologies via functors.** Where structure overlaps existing domains, implement functors and verify functor laws.
9. **(Optional) Pair functors into adjunctions for gap detection.** Use adjunctions and associated tests to surface missing distinctions.

The final gate for any change is `cargo test --workspace`. If anything that used to pass now fails, the new capability has introduced a contradiction that must be resolved before shipping.

## Part 1: Adding a Programming Language Capability

This section specializes the general process for programming languages (PHP, Rust, HTML, etc.). The goal is to encode the language’s **syntax and structural invariants** so pr4xis can reason about programs as objects in a category, not to generate code.

### Step 1: Formulate the One-Sentence Theorem

Write a single sentence of the form:

> “The <Language> <version> reference can be encoded as a category whose objects are structs> and whose morphisms are <relationships>, with axioms capturing <invariants>.”

For example:

> “PHP 8.3’s language reference can be encoded as a category whose objects are language constructs (file, namespace, class, method, parameter, expression, type), whose mereology is the source-declared containment structure, whose taxonomy is the type and construct hierarchy, and whose domain axioms are the invariants and constraints the manual specifies.”

If the sentence cannot be written cleanly, choose a more precise source or narrow the scope (for example, “the type system only” or “file structure and declarations only”).

### Step 2: Select and Archive Authoritative Sources

Choose sources that are authoritative, citable, and finite:

- Official language reference (for example, php.net/manual for PHP, the Rust Reference for Rust, the HTML Living Standard for HTML).
- Related standards (for example, PSR-12 for PHP file structure and style).

Archive them for the ontology:

- Save relevant PDFs or HTML exports into a `papers/` subdirectory under the planned ontology path.
- Prefer stable, versioned references (specific language versions, tagged spec revisions).

### Step 3: Extract Entities, Relations, Axioms, and Qualities

Read the source and build four lists:

- **Entities**: each named construct or semantic concept that deserves its own `Entity` variant (for example, `File`, `Namespace`, `Class`, `ReadonlyClass`, `Trait`, `Interface`, `Method`, `Property`, `Parameter`, `Expression`, type forms).
- **Taxonomy rows (is-a)**: `(Child, Parent)` pairs like `(ReadonlyClass, Class)` or `(IntType, ScalarType)`.
- **Mereology rows (part-of)**: `(Part, Whole)` pairs like `(Method, Class)`, `(Class, Namespace)`, `(Namespace, File)`.
- **Other relations and axioms**: causal triggers, opposition pairs, and explicit invariants (for example, “readonly classes require readonly properties”, “strict_types declaration must appear first”).

Only encode relations and axioms that appear in the chosen sources; “obvious” or folk-knowledge rules are excluded unless they are explicitly documented.

### Step 4: Scaffold the Ontology Directory for the Language

Choose a branch under `crates/domains/src/` that matches the type of domain. Programming languages and software artefacts typically belong in a “software” branch (for example, `social/software/langs/anguage>/` or similar, depending on the existing tree in the repository).

Create a directory structure of the form:

```text
crates/domains/src/<branch>/<topic>/anguage_version>/
    ontology.rs       # define_ontology! block and entity/relation definitions
    tests.rs          # structural and property-based tests for axioms
    mod.rs            # module wiring
    citings.md        # bibliographic entries and source mapping
    papers/           # PDFs, text captures of reference material
```

Populate `papers/` with the language reference excerpts and related standards that will be cited in documentation and axiom implementations.

### Step 5: Implement the `define_ontology!` Block for the Language

In `ontology.rs`, define the core ontology using the macros from the `pr4xis` crate.[^1]

Typical structure:

```rust
use pr4xis::define_ontology;

define_ontology! {
    /// Short abstract explaining what this ontology encodes and citing the primary sources.
    pub Php83 {
        entity: PhpEntity,
        category: PhpCategory,
        relation: PhpRelation,

        taxonomy: PhpTaxonomy [
            (ReadonlyClass, Class),
            (IntType, ScalarType),
            (FloatType, ScalarType),
            (StringType, ScalarType),
            (BoolType, ScalarType),
            // additional is-a relations taken directly from the language reference
        ],

        mereology: PhpMereology [
            (Parameter, Method),
            (Method, Class),
            (Property, Class),
            (Class, Namespace),
            (Namespace, File),
            // additional part-of edges taken directly from the reference
        ],

        // Add other reasoning systems (causation, opposition, context) only if justified by the source
    }
}
```

Around this macro:

- Define the `PhpEntity` enum with one variant per language construct.
- Define a `PhpRelation` type that captures morphisms as needed.

The macro expands into a category, reasoning systems, and structural axioms (for example, taxonomy acyclicity) plus test scaffolding.

### Step 6: Add Domain Axioms for the Language

Domain axioms capture genuine invariants from the language specification and style guides, rather than common sense or taste.

For each axiom:

1. Define a struct representing the axiom.
2. Implement a trait (for example, `Precondition<Situation, Action>`) with:
   - A unique name and human-readable description.
   - A total `check(&self, situation, action)` implementation that always returns either “Satisfied” or “Violated”, never “unknown”.
   - A violation reason string that points back to the relevant part of the source.
3. Add property-based tests using a library such as `proptest` (which is listed as a dev dependency in the crate).[^1]
4. Cite the exact section, paragraph, or line range from the reference in doc comments.

Example pattern for an axiom like “every property of a readonly class must be readonly”: define a `ReadonlyClassImmutability` struct, implement `check` to inspect readonly classes and their properties, and write a property-based test that constructs random readonly-class shapes and ensures the axiom never reports false positives or negatives.

### Step 7: Verify Structural Laws for the Language Ontology

Use `cargo test` to confirm that both the language-specific tests and the global category laws pass.

Typical commands:

```bash
# Run tests for the new language domain only
cargo test -p pr4xis-domains anguage_module_path>::tests

# Check category laws (identity and composition) across all categories, including the new one
cargo test -p pr4xis category::validate::check_category_laws
```

Fix any failing tests before proceeding. Failures at this stage indicate a mismatch between the encoded ontology and the category-theoretic guarantees pr4xis expects.

### Step 8: Compose with Existing Ontologies via Functors

Where the language shares structure with more abstract domains, implement functors to embed it into those domains. For example:

- A functor from `Php83` to a generic “object-oriented programming” ontology.
- A functor from the language’s type system into a generic type-theory ontology.

The functor implementation must preserve identities and composition; associated tests should call a global functor-law validator.

Typical commands:

```bash
cargo test -p pr4xis-domains <functor_name>
cargo test -p pr4xis category::validate::check_functor_laws
```

A failing functor-law test usually indicates either a mis-encoded mapping (for example, mapping two distinct constructs to one that collapses distinctions improperly) or a missing concept in the target ontology.

### Step 9: (Optional) Use Adjunctions for Gap Detection

If both directions of a structural relationship between domains are encoded, pair functors into an adjunction and run gap-analysis tests.

- Construct a functor `F: Language → AbstractDomain` and a functor `G: AbstractDomain → Language`.
- Implement tests that measure how many entities “collapse” under `G(F(x))` compared with `x`.

Entities that do not survive round-trips often point to missing distinctions, such as a special class kind, modifier, or semantic role not captured in the abstract domain.

### Final Gate for a Language Capability

Before considering the language “taught” to pr4xis, run:

```bash
cargo test --workspace
```

Any regression indicates a contradiction or integration problem. Investigate whether the new ontology, its axioms, or its functors are over-constraining or misaligned with existing domains.

## Part 2: Adding LLM CLI Command Grammar Capabilities

This section specializes the general process for CLIs that wrap LLMs (for example, “Gemini 3.1 Pro” or “Claude Opus” command-line tools). The goal is to encode their **command grammars and invocation invariants**, not to implement subprocess integration.

### Step 1: Formulate the One-Sentence Theorem for Each CLI

Write a sentence of the form:

> “The <CLI name> <version> command-line interface has a command grammar that can be encoded as a category whose objects are <invocation components> and whose morphisms are tainment / typing relations>, with axioms capturing <auth and flag invariants>.”

For example:

> “The Gemini 3.1 Pro CLI has a command grammar that can be encoded as a category whose objects are command forms and flag values and whose mereology is the ‘flag is-part-of invocation’ relationship, with axioms for required flags, mutually exclusive flags, and authentication preconditions.”

Repeat the same exercise for each CLI (for example, Claude’s CLI) as a separate ontology.

### Step 2: Gather and Archive CLI Sources

For each CLI, collect and archive authoritative sources:

- The output of `li> --help` captured verbatim into a text file.
- Official authentication and configuration docs (PDF or text export), especially sections describing environment variables and required flags.

Store them under a path such as:

```text
crates/domains/src/applied/llm_clis/li_name_version>/papers/
    li>-help.txt
    li>-auth.pdf
    li>-usage.pdf
```

### Step 3: Extract CLI Entities, Relations, and Axioms

From the help and docs, list:

- **Entities**: `Invocation`, `Subcommand`, `Flag`, `FlagValue`, `ModelFlag`, `TemperatureFlag`, `StreamFlag`, `JsonFlag`, `SystemFlag`, `EnvVar`, `AuthToken`, etc.
- **Mereology rows**: `(Flag, Invocation)`, `(FlagValue, Flag)`, `(Subcommand, Invocation)`, `(EnvVar, Environment)`.
- **Taxonomy rows**: `(ModelFlag, Flag)`, `(TemperatureFlag, Flag)`, `(StreamFlag, Flag)`, `(JsonFlag, Flag)`.
- **Opposition pairs**: mutually exclusive constructs like `(StreamFlag, JsonFlag)` when specified as such.
- **Domain axioms**, each tied to explicit text in `--help` or docs, such as:
  - “A valid invocation of this CLI requires environment variable `<NAME>` to be set.”
  - “The `--model` flag must take one of the listed model IDs.”
  - “Flags `--stream` and `--json` may not appear together.”
  - “The `--temperature` value must be within a documented numeric range.”

Again, encode only what the official help/docs assert; do not infer behavior from external examples unless those examples are also treated as primary sources.

### Step 4: Scaffold the Ontology Directories for CLIs

Under an appropriate branch for “applied” or integration domains (such as `crates/domains/src/applied/llm_clis/`), create a folder per CLI and version:

```text
crates/domains/src/applied/llm_clis/gemini_3_1_pro/
    ontology.rs
    tests.rs
    mod.rs
    citings.md
    papers/
        gemini-help.txt
        gemini-auth.pdf

crates/domains/src/applied/llm_clis/claude_opus_4_7/
    ontology.rs
    tests.rs
    mod.rs
    citings.md
    papers/
        claude-help.txt
        claude-auth.pdf
```

Wire each module into its parent `mod.rs` so that it is included in the workspace build and tests.

### Step 5: Implement `define_ontology!` Blocks for Each CLI

In each `ontology.rs`, define the CLI’s ontology using the macros exposed by the `pr4xis` crate.[^1]

Example pattern for the Gemini CLI:

```rust
use pr4xis::define_ontology;

define_ontology! {
    /// Gemini 3.1 Pro CLI command grammar.
    /// Source: `gemini --help` output and authentication docs (see papers/).
    pub GeminiCli {
        entity: GeminiCliEntity,
        category: GeminiCliCategory,
        relation: GeminiCliRelation,

        taxonomy: GeminiCliTaxonomy [
            (ModelFlag, Flag),
            (TemperatureFlag, Flag),
            (StreamFlag, Flag),
            (SystemFlag, Flag),
            (JsonFlag, Flag),
            // additional flag categories as required
        ],

        mereology: GeminiCliMereology [
            (Flag, Invocation),
            (FlagValue, Flag),
            (Subcommand, Invocation),
        ],

        opposition: GeminiCliOpposition [
            // Example: if the help specifies these as mutually exclusive
            (StreamFlag, JsonFlag),
        ],
    }
}
```

Define `GeminiCliEntity` to include variants for `Invocation`, `Subcommand`, each flag category, and any other CLI-level constructs. Define `GeminiCliRelation` as the relation type.

### Step 6: Add CLI Domain Axioms

Implement preconditions and axioms that correspond directly to documented CLI invariants. Examples:

- An authentication precondition that checks for the CLI’s required environment variable and rejects invocations when it is absent.
- A precondition for `--model` that ensures the value is one of the documented model identifiers.
- A precondition enforcing mutual exclusion for flagged options (for example, `--stream` vs. `--json`).
- A precondition ensuring numeric flags lie in documented ranges (for example, temperature bounds).

Each axiom should have:

- A unique rule name.
- A description and violation reason string.
- A total `check` function that always reports satisfied or violated.
- Inline doc comments citing line numbers in `--help` or specific sections in the docs.
- Property-based tests in `tests.rs` that construct CLI invocations and check that the axiom behaves correctly.

### Step 7: Verify Structural Laws for CLI Ontologies

Run CLI-specific tests and global category-law checks:

```bash
# Per-domain tests for each CLI
cargo test -p pr4xis-domains applied::llm_clis::gemini_3_1_pro::tests
cargo test -p pr4xis-domains applied::llm_clis::claude_opus_4_7::tests

# Global category laws
cargo test -p pr4xis category::validate::check_category_laws
```

Fix structural or axiom-related failures as they arise.

### Step 8: Compose CLI Ontologies with a Generic CLI Ontology

Define a generic CLI ontology that captures shared structure across all CLIs: commands, subcommands, flags, env-var-based auth, mutually exclusive flag sets, etc. Then:

- Implement a functor from each concrete CLI ontology into the generic CLI ontology.
- Ensure the functors preserve identities and composition, and add tests that exercise the functor-law validator.

Typical commands:

```bash
cargo test -p pr4xis-domains GeminiCliToGenericCli
cargo test -p pr4xis-domains ClaudeOpusCliToGenericCli
cargo test -p pr4xis category::validate::check_functor_laws
```

Passing tests show that both CLIs instantiate the same abstract command grammar structure, allowing pr4xis to reason uniformly about “an LLM CLI invocation” at the generic level.

### Step 9: Adjunction-Based Gap Detection Across CLIs

To detect asymmetric capabilities or configuration options between CLIs:

- Define a “forward” functor `F: GeminiCli → GenericCli` and a “backward” functor `G: GenericCli → GeminiCli`.
- Do the same for the second CLI.
- Write tests that compute how many entities collapse when round-tripped (for example, `G(F(x))` vs. `x`).

Entities that collapse (for example, special-purpose flags or modes that have no counterpart in the generic ontology or in another CLI’s image) identify structural differences, such as a tool-use flag or streaming behavior available in one CLI but not another.

### Final Gate for CLI Capabilities

As with programming languages, run the full workspace test suite:

```bash
cargo test --workspace
```

Only when this passes is the CLI ontology considered integrated. At that point, pr4xis can answer questions such as whether a proposed command line is valid with respect to the documented grammar and invariants.

## Part 3: Using the `.claude/skills/` Tools as Scaffolding

The repository includes a `.claude/skills/` directory with pre-defined skills that assist with ontology creation, functor authoring, adjunctions, and documentation. These are development-time tools and do not replace pr4xis’s own reasoning engine.

The high-level pattern is:

1. Use skills to generate **drafts** of ontologies, functors, or README files.
2. Run the generated code through `cargo test` and pr4xis’s law-checking tests.
3. Manually review and fix `todo!` markers, test failures, and any mismatches.

Skills are intentionally non-authoritative; pr4xis’s category and functor law checks remain the final arbiter of correctness.

## Part 4: What “Training” Means for pr4xis

In pr4xis, “training” a capability such as a language or CLI means:

- Designing and implementing a new ontology that encodes the domain as a category, with explicit entities, relations, and reasoning systems.
- Adding domain axioms that capture invariants from authoritative sources.
- Proving that the resulting category and its functors satisfy structural laws via automated tests.

There is no stochastic generalization or hidden internal state; all behavior is derived from explicit axioms and verified structure. Once the tests pass, pr4xis can reason about programs written in the encoded languages and invocations of the encoded CLIs in a way that is traceable down to the original sources and axioms.

---

## References

1. [i-am-logger - Overview](https://github.com/i-am-logger) - i-am-logger has 155 repositories available. Follow their code on GitHub.

