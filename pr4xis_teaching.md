I have everything I need. Now I'll produce the revised, pr4xis-specific document. The framing completely changes: pr4xis doesn't "learn a language" by reading docs into context — it acquires a capability when someone **encodes the capability's first principles as axioms in an ontology**, proves the structural laws hold, and composes it via functors with existing ontologies.

---

# Teaching pr4xis a Capability: A Repeatable Process Grounded in How pr4xis Actually Works

> **Purpose of this document:** Give you a copy-paste playbook for adding a new capability to pr4xis — a programming language, a command-line tool, or any other domain — by encoding it the way pr4xis is designed to be extended: as an **ontology built from first principles and axioms**, not as documentation injected into a context window. Nothing in your repository has been modified.

---

## Part 0 — Why this playbook is different

pr4xis is **not** an LLM. Re-read this sentence until it's uncomfortable, because it inverts almost every instinct developed with Claude, GPT, or Gemini.

From `README.md` and `docs/understand/concepts.md`:

- pr4xis derives the next claim **from accepted axioms**, the way a mathematician proves a theorem. There is no "next-token prediction", no "context window", no "temperature".
- Every domain is a **category** in the formal mathematical sense: objects (Entities), morphisms (Relationships), composition, and identity — with the category laws verified by `cargo test`.
- Capabilities are added by writing an **ontology** (usually via the `define_ontology!` macro), composing it with existing ontologies via **functors**, and optionally pairing those functors into **adjunctions** that surface missing distinctions.
- Nothing becomes part of pr4xis until it **passes the laws check**. `cargo test --workspace` is the gatekeeper, not a human reviewer's impression of whether the prose is convincing.

Consequences for this document:

| LLM teaching | pr4xis teaching |
|---|---|
| Paste docs into the chat | Encode named concepts as `Entity` enum variants |
| Write a `CLAUDE.md` memory file | Write a `define_ontology!` block |
| Hope the model generalizes | Prove the functor laws hold |
| "Version-pin" the docs | Cite the source paper in the ontology |
| Grep a HELP-OUTPUT.txt to avoid hallucination | Hallucination is structurally impossible — unencoded claims simply don't derive |
| Ship when it "seems right" | Ship when `cargo test --workspace` is green |

Everything that follows is the pr4xis-native equivalent of "teaching the AI something new".

---

## Part 1 — The mental model

### What "a capability" actually is in pr4xis

A capability is a **category plus reasoning systems plus axioms**, committed under `crates/domains/src/`. Per `docs/understand/concepts.md`:

- **Objects** are `Entity` enum variants — the named things your source paper talks about.
- **Morphisms** are `Relationship` values — directed maps between entities.
- **Reasoning systems** interpret subsets of morphisms: taxonomy (`is-a`), mereology (`part-of`), causation, opposition, context.
- **Structural axioms** come for free with `define_ontology!` — no cycles in a taxonomy, weak supplementation in a mereology, and so on.
- **Domain axioms** are extra `Precondition` or `Axiom` impls you add because your source paper says something must always be true (e.g., *"voltage = current × resistance"*, *"a king moves at most one square"*).

That is the whole vocabulary. Any new capability must fit into it. If it does not, the capability is not yet decomposed into first principles.

### The two kinds of "teaching" you might attempt

1. **Capability extension (the real thing).** You encode a new domain as an ontology and let the engine use it. This is durable, verifiable, and composable with all 106 existing ontologies.
2. **Session-scoped assistance (the scaffolding).** You invoke one of the skills under `.claude/skills/` (e.g., `ontology-from-paper`) to help *you* author the ontology faster. The skill itself is not pr4xis learning — it's *you* using Claude Code to produce a draft that pr4xis then validates.

This document is primarily about (1). Section 5 explains how to use (2) to accelerate it.

---

## Part 2 — The repeatable 9-step process

This process is the operational form of `docs/use/build-ontology-from-paper.md`, with the extra cross-cutting steps from `docs/use/write-axioms.md` and `docs/use/compose-via-functor.md` folded in. Use it every time.

### Step 1 — State the capability as a one-sentence theorem
Not "teach pr4xis PHP". Instead:
> *"PHP 8.3's syntactic and execution model can be encoded as a category whose objects are language constructs (files, classes, functions, statements, expressions, types), whose morphisms are the containment and typing relations stated in the PHP 8.3 reference, and whose axioms are the invariants the reference declares (strict types, readonly class immutability, PSR-12 file structure)."*

If you cannot write this sentence, you do not yet have a source concrete enough to encode. Pick a better source.

### Step 2 — Choose a citable source
From `docs/use/build-ontology-from-paper.md`: *"Authoritative, citable, and finite."*
- A **language reference** (php.net/manual, HTML Living Standard, the Rust Reference)
- A **standards document** (ECMA-262, W3C HTML, PSR-12)
- A **CLI's own `--help` output** captured verbatim and committed alongside the ontology

The repo's explicit rule: *"Avoid: blog posts, Wikipedia pages, AI summaries, your own notes."*

### Step 3 — Extract concepts, relations, axioms, qualities
Read the source and list, side-by-side:
- **Named concepts** → candidate `Entity` variants (one name = one concept)
- **Is-a statements** → taxonomy rows
- **Part-of statements** → mereology rows
- **Causes / triggers** → causation rows
- **Opposites / contrasts** → opposition rows
- **Stated invariants** → domain axioms (each with source citation)
- **Measured quantities** → `Quality` types with units
- **Context-dependent names** → `ContextDef` entries

Follow the repo's hard rule: *"If the source says it, the relation goes in. If the source doesn't say it, the relation does not go in — even if it feels obvious."*

### Step 4 — Scaffold the ontology directory
Under `crates/domains/src/<branch>/<topic>/`:

```
ontology.rs       -- define_ontology! block, Entity enum, Relation type
tests.rs          -- structural law tests + proptest for each domain axiom
mod.rs            -- wiring
citings.md        -- full bibliographic entries, one per source
papers/           -- source PDFs if available
README.md         -- generated by the per-ontology-readme skill
```

Branches in use (confirmed in `crates/domains/src/`): `applied/`, `cognitive/`, `formal/`, `natural/`, `social/`.

### Step 5 — Write the `define_ontology!` block
Skeleton from `docs/use/build-ontology-from-paper.md`:

```rust
use pr4xis::define_ontology;

define_ontology! {
    /// One-paragraph abstract citing the source.
    /// Source: <Author, Title, Year, Edition>.
    pub MyOntology {
        entity: MyEntity,
        category: MyCategory,
        relation: MyRelation,

        taxonomy: MyTaxonomy [
            (ChildConcept, ParentConcept),
            // one row per is-a relation in the source
        ],

        mereology: MyMereology [
            (Part, Whole),
            // one row per part-of relation in the source
        ],

        // omit any reasoning system the source doesn't motivate
    }
}
```

Then `#[derive(Entity)]` the `MyEntity` enum, define `MyRelation`, and the macro emits the category, the reasoning systems, the structural axioms, and the test scaffolding.

### Step 6 — Add domain axioms
From `docs/use/write-axioms.md`, each domain axiom is a `Precondition` (state-dependent) or `Axiom` (unconditional) impl, with:

1. A `name()` and `description()`.
2. A total `check()` — every (situation, action) pair yields `Satisfied` or `Violated`, never "I don't know".
3. A doc comment citing the source paragraph.
4. A proptest verifying it across random inputs.

The repo's four hard rules for axioms:
- Don't encode "common sense" that isn't in a source.
- Don't use floating-point equality (use epsilon or rationals).
- Don't let one axiom catch multiple unrelated errors — split them.
- Don't make axioms expensive — preconditions run on every action.

### Step 7 — Verify the structural laws
Run, exactly as specified in the docs:

```bash
cargo test -p pr4xis-domains <topic>::tests
cargo test -p pr4xis category::validate::check_category_laws
```

If these fail, your encoding has a structural bug. **This is the actual "teaching has succeeded" moment** — not a chat going well, but the laws passing.

### Step 8 — Compose with existing ontologies via functors
From `docs/use/compose-via-functor.md` and `.claude/skills/functor-author/SKILL.md`, if your new ontology shares concepts with an existing one, write a functor and verify it:

```bash
cargo test -p pr4xis-domains <functor_name>
cargo test -p pr4xis category::validate::check_functor_laws
```

A functor whose laws pass **is a theorem** that your new domain's structure faithfully embeds into the target domain. If the laws fail, the tool tells you which morphism breaks composition or identity. Do not "fix" it until you understand why it broke — the failure is information.

### Step 9 — (Optional) Pair functors into an adjunction for gap detection
From `docs/understand/concepts.md` and `.claude/skills/adjunction-author/SKILL.md`, two opposing functors `F: A → B` and `G: B → A` may form an adjunction. Run the gap analysis:

```bash
cargo test -p pr4xis-domains test_full_chain_collapse_measurement -- --nocapture
```

Each entity that collapses in a round-trip `G(F(A)) ≠ A` is a **missing distinction the math detected automatically** — exactly how the bioelectricity Kv discovery in `crates/domains/src/natural/biomedical/` surfaced the homeostatic-vs-therapeutic split. If your capability has role-ambiguous concepts, the adjunction will find them.

### Final gate — the workspace test
```bash
cargo test --workspace
```
Per `docs/use/build-ontology-from-paper.md`: *"If anything that was passing is now failing, you have introduced a contradiction with an existing ontology."* Do not roll back the new ontology reflexively — the conflict may reveal a bug in an older one.

---

## Part 3 — Worked Example A: Teaching pr4xis HTML + PHP

The wrong mental picture: "show pr4xis the PHP manual so it can write PHP". pr4xis does not write PHP; there is no code generator downstream. The right mental picture: **encode PHP's structural rules as an ontology so pr4xis can reason about PHP programs as verified objects** — answer "is this PSR-12 compliant?", "does this file satisfy strict-types?", "which functor maps PHP class hierarchies to the generic class-hierarchy ontology in `cognitive/`?" — and compose those answers with other domains.

### Step 1 — One-sentence theorems (two, because HTML and PHP are separate categories)
> *"The HTML5 Living Standard's element tree can be encoded as a category whose objects are element kinds and whose taxonomy is the 'element is a kind of content-model category' hierarchy declared by the spec."*
>
> *"PHP 8.3's language reference can be encoded as a category whose objects are language constructs (file, namespace, class, method, parameter, expression, type), whose mereology is the containment structure, whose taxonomy is the type hierarchy, and whose domain axioms are the invariants the manual declares."*

### Step 2 — Sources
- html.spec.whatwg.org — HTML Living Standard (the standards doc, not an MDN summary)
- php.net/manual/en — PHP 8.3 manual
- php-fig.org/psr/psr-12 — PSR-12 (the style axioms)

Drop the verbatim excerpts into `crates/domains/src/social/software/markup/html/papers/` and `crates/domains/src/social/software/langs/php/papers/` (the `social/software/markup/xml` path already exists per `crates/cli/src/main.rs` line 13 — follow that convention).

### Step 3 — Extract (abbreviated sample)

HTML entities: `Document`, `Element`, `Head`, `Body`, `MetadataContent`, `FlowContent`, `PhrasingContent`, `Heading`, `Paragraph`, `Form`, `Input`, `Button`, ...

HTML taxonomy: `(Heading, FlowContent)`, `(Paragraph, FlowContent)`, `(Input, PhrasingContent)`, `(PhrasingContent, FlowContent)`, ...

HTML mereology: `(Head, Document)`, `(Body, Document)`, `(MetadataContent, Head)`, ...

PHP entities: `File`, `Namespace`, `Class`, `ReadonlyClass`, `Trait`, `Interface`, `Method`, `Property`, `Parameter`, `Expression`, `IntType`, `FloatType`, `StringType`, `BoolType`, ...

PHP taxonomy: `(ReadonlyClass, Class)`, `(IntType, ScalarType)`, ...

PHP mereology: `(Method, Class)`, `(Parameter, Method)`, `(Class, Namespace)`, `(Namespace, File)`, ...

PHP domain axioms (from the manual, each with a citation):
- **`StrictTypesDeclaration`** — a file that declares `strict_types=1` must have that declaration as its first statement after `<?php`. *Source: php.net/manual/en/control-structures.declare.php.*
- **`ReadonlyClassImmutability`** — every property of a `ReadonlyClass` must be declared `readonly`. *Source: php.net/manual/en/language.oop5.basic.php#language.oop5.basic.class.readonly.*
- **`Psr12FileHeader`** — PHP files contain a `declare(strict_types=1);` statement, blank line, then namespace, blank line, then use imports. *Source: PSR-12 §3.*

### Step 4-5 — Scaffold and `define_ontology!`

```
crates/domains/src/social/software/markup/html/
crates/domains/src/social/software/langs/php/
```

PHP `ontology.rs` skeleton:

```rust
use pr4xis::define_ontology;

define_ontology! {
    /// PHP 8.3 language structure.
    /// Source: php.net/manual/en (PHP 8.3), PSR-12 §3.
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
            // ... every is-a the manual declares
        ],

        mereology: PhpMereology [
            (Parameter, Method),
            (Method, Class),
            (Property, Class),
            (Class, Namespace),
            (Namespace, File),
            // ... every part-of the manual declares
        ],
    }
}
```

### Step 6 — A domain axiom (the whole point)

```rust
/// Every property of a readonly class must itself be readonly.
///
/// **Source:** php.net/manual/en/language.oop5.basic.php,
/// section "Readonly classes" (PHP 8.2+).
///
/// **Statement:** For every `Class` C such that `C is-a ReadonlyClass`,
/// every `Property` P where `(P, C) ∈ mereology` must carry the
/// `readonly` modifier.
pub struct ReadonlyClassImmutability;

impl Precondition<PhpSituation, PhpAction> for ReadonlyClassImmutability {
    fn check(&self, s: &PhpSituation, a: &PhpAction) -> PreconditionResult {
        let next = s.simulate(a);
        for class in next.readonly_classes() {
            for prop in next.properties_of(class) {
                if !prop.is_readonly() {
                    return PreconditionResult::Violated {
                        rule: "ReadonlyClassImmutability",
                        reason: format!(
                            "Property {} of readonly class {} is not readonly",
                            prop.name(), class.name()
                        ),
                        situation: s.clone(),
                        attempted_action: a.clone(),
                    };
                }
            }
        }
        PreconditionResult::Satisfied {
            rule: "ReadonlyClassImmutability",
            reason: "All properties of every readonly class are readonly",
        }
    }
}
```

Proptest (pattern from `docs/use/write-axioms.md`):

```rust
proptest! {
    #[test]
    fn readonly_class_props_are_readonly_for_all_valid_classes(
        class in any_readonly_class_strategy()
    ) {
        let s = PhpSituation::containing(class.clone());
        let a = PhpAction::Noop;
        prop_assert!(matches!(
            ReadonlyClassImmutability.check(&s, &a),
            PreconditionResult::Satisfied { .. }
        ));
    }
}
```

### Step 7 — Verify

```bash
cargo test -p pr4xis-domains php::tests
cargo test -p pr4xis category::validate::check_category_laws
```

### Step 8 — Compose with existing ontologies

PHP's `Class`/`Method`/`Property` almost certainly share structure with a generic "object-oriented programming" ontology or with the type-system ontology if one exists under `formal/`. Use the `functor-author` skill (Section 5) to scaffold `Php83ToOop` and run:

```bash
cargo test -p pr4xis-domains Php83ToOop
cargo test -p pr4xis category::validate::check_functor_laws
```

Same procedure for HTML ↔ generic tree structure in `formal/`.

### Step 9 — Gap detection (illustrative)
If you pair `Php83ToOop` with its reverse, the round-trip may collapse `ReadonlyClass` back to `Class` — exactly the kind of finding adjunction gap analysis exists to surface. Resolve with a `ContextDef` or leave it as a documented missing distinction in the generic OOP ontology.

### Final gate
```bash
cargo test --workspace
```

### What "pr4xis knows PHP now" means
After this succeeds, pr4xis can answer questions about PHP programs **that derive from the axioms**: *"does this source tree satisfy PSR-12?"*, *"is this readonly class immutability violation a real violation or an artefact of my AST?"*, *"which PHP constructs have no morphism into the generic OOP ontology and therefore represent PHP-specific semantics?"* Those answers carry a trace. They cannot be hallucinated, because — per the LLM-vs-pr4xis table in `README.md` — *"hallucination is impossible — every claim traces to a proof"*.

---

## Part 4 — Worked Example B: Teaching pr4xis two CLIs (Gemini 3.1 Pro and Claude Opus 4.7)

Again, check the instinct: we are **not** teaching pr4xis to call these CLIs over a subprocess (though something downstream could). We are encoding the CLIs' **command grammar and invocation invariants** as ontologies, so pr4xis can reason about whether a proposed command line is valid, which flags compose, and what the execution preconditions are.

### Step 1 — One-sentence theorems
> *"The Gemini 3.1 Pro CLI has a command grammar that can be encoded as a category whose objects are command forms and flag values and whose mereology is the 'flag is-part-of command' containment, with axioms stating required flags, mutually-exclusive flags, and auth preconditions."*
>
> *"The Claude Opus 4.7 CLI is the analogous encoding for its own command grammar."*

### Step 2 — Sources (non-negotiable)
- `gemini --help` captured verbatim. Commit as `papers/gemini-help.txt`.
- `claude --help` captured verbatim. Commit as `papers/claude-help.txt`.
- The official auth/install docs for each CLI, committed as PDFs in the same `papers/` directories.

The repo's rule again: no blogs, no Wikipedia, no AI summaries. The `--help` output is the primary source — it is finite, authoritative, and versioned with the binary.

### Step 3 — Extract (sketch, using placeholder flags — replace with what the real `--help` prints)

Entities: `Invocation`, `Subcommand`, `Flag`, `FlagValue`, `ModelFlag`, `TemperatureFlag`, `StreamFlag`, `SystemFlag`, `JsonFlag`, `EnvVar`, `AuthToken`.

Mereology: `(Flag, Invocation)`, `(FlagValue, Flag)`, `(Subcommand, Invocation)`.

Taxonomy: `(ModelFlag, Flag)`, `(TemperatureFlag, Flag)`, `(StreamFlag, Flag)`, ...

Domain axioms (each citing the exact line number of `gemini-help.txt` or `claude-help.txt`):

- **`AuthEnvVarPresent`** — any `Invocation` requires the appropriate environment variable (`GEMINI_API_KEY` or `ANTHROPIC_API_KEY`) to be set. *Source: gemini-help.txt L<n>; anthropic docs §auth.*
- **`ModelFlagTakesKnownValue`** — the value of `--model` must be one of the model IDs enumerated by `<cli> models list`. *Source: <cli>-help.txt L<n>.*
- **`StreamAndJsonMutuallyExclusive`** *(if the source says so)* — an `Invocation` may not carry both `--stream` and `--json`. *Source: <cli>-help.txt L<n>.*
- **`TemperatureInRange`** — if present, `--temperature` takes a value in `[0.0, 2.0]`. *Source: <cli>-help.txt L<n>.*

Every one of those axioms has a line-number citation. If a rule is not in `--help`, **it does not become an axiom.** No exceptions. This is the pr4xis equivalent of the "don't invent flags" rule in the old LLM playbook — but instead of hoping the model remembers, the engine structurally refuses to derive a command that isn't supported by an axiom.

### Step 4-5 — Scaffold

```
crates/domains/src/applied/llm_clis/gemini_3_1_pro/
    ontology.rs
    tests.rs
    mod.rs
    citings.md
    papers/
        gemini-help.txt
crates/domains/src/applied/llm_clis/claude_opus_4_7/
    ontology.rs
    tests.rs
    mod.rs
    citings.md
    papers/
        claude-help.txt
```

`applied/` is the right branch per `crates/domains/src/` — that's where pragmatic integration ontologies live (confirmed by `applied/data_provisioning/` in `crates/cli/src/main.rs` line 7).

`define_ontology!` skeleton for Gemini (Claude is parallel):

```rust
define_ontology! {
    /// Gemini 3.1 Pro CLI command grammar.
    /// Source: `gemini --help` output captured 2026-04-24, committed verbatim
    /// at papers/gemini-help.txt.
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
            // ... every flag category from the --help
        ],

        mereology: GeminiCliMereology [
            (Flag, Invocation),
            (FlagValue, Flag),
            (Subcommand, Invocation),
        ],

        opposition: GeminiCliOpposition [
            // if --stream and --json are mutually exclusive per the help text:
            (StreamFlag, JsonFlag),
        ],
    }
}
```

### Step 6 — Axioms (two CLIs, one shared invariant)

```rust
/// An invocation requires the CLI's auth env var to be set.
///
/// **Source:** Gemini CLI auth docs (committed at papers/gemini-auth.pdf);
/// `gemini --help` L12-14 references `GEMINI_API_KEY`.
pub struct GeminiAuthPresent;

impl Precondition<CliSituation, CliAction> for GeminiAuthPresent {
    fn check(&self, s: &CliSituation, a: &CliAction) -> PreconditionResult {
        if let CliAction::Invoke(inv) = a {
            if inv.cli() == Cli::Gemini && !s.env_has("GEMINI_API_KEY") {
                return PreconditionResult::Violated {
                    rule: "GeminiAuthPresent",
                    reason: "GEMINI_API_KEY is not set in the environment".into(),
                    situation: s.clone(),
                    attempted_action: a.clone(),
                };
            }
        }
        PreconditionResult::Satisfied {
            rule: "GeminiAuthPresent",
            reason: "GEMINI_API_KEY is set",
        }
    }
}
```

Analogous `ClaudeAuthPresent` for `ANTHROPIC_API_KEY`.

### Step 7 — Verify

```bash
cargo test -p pr4xis-domains gemini_3_1_pro::tests
cargo test -p pr4xis-domains claude_opus_4_7::tests
cargo test -p pr4xis category::validate::check_category_laws
```

### Step 8 — Compose with a generic CLI-invocation ontology
Both CLIs share structure (subcommands, flags, env-var auth, mutually-exclusive flag pairs). Author a single abstract ontology `applied/llm_clis/generic/` and build functors:

```
Functor: GeminiCli → GenericCli
Functor: ClaudeOpus47Cli → GenericCli
```

Run:

```bash
cargo test -p pr4xis-domains GeminiCliToGenericCli
cargo test -p pr4xis-domains ClaudeOpus47CliToGenericCli
cargo test -p pr4xis category::validate::check_functor_laws
```

The functor laws passing is the theorem that both CLIs instantiate the same command-grammar structure — without which a `pr4xis` caller could not treat "an LLM CLI invocation" as a first-class concept.

### Step 9 — Adjunction gap detection
If Claude Opus 4.7 has a construct Gemini 3.1 Pro doesn't (or vice versa), the round-trip through the two functors to the generic ontology and back will collapse it. That collapse is exactly the information you need: *"Claude has a `--tool-use` flag that has no Gemini analogue"*, for example. Surface with a `ContextDef` or leave documented as a known asymmetry.

### Final gate
```bash
cargo test --workspace
```

### What "pr4xis knows these CLIs now" means
After this succeeds, pr4xis can, for any proposed command line, derive a pass/fail against the axioms: *"is `GEMINI_API_KEY` set?"*, *"is `--temperature 3.5` in range?"*, *"is `--stream --json` allowed?"* Every answer traces to a cited line of `--help`. Unsupported flags cannot be "invented" because no axiom derives them.

---

## Part 5 — Leveraging the existing skills under `.claude/skills/`

The repo already ships purpose-built skills for every step above (see `.claude/skills/README.md`). They are not pr4xis itself — they are Claude Code skills the contributor uses to produce drafts that pr4xis then validates. Use them to accelerate the 9-step process.

| Skill | Where it fits in the 9 steps |
|---|---|
| [`ontology-from-paper`](.claude/skills/ontology-from-paper/SKILL.md) | Steps 3-9 end-to-end; produces a **draft for human review**, does not auto-commit |
| [`functor-author`](.claude/skills/functor-author/SKILL.md) | Step 8; scaffolds `Functor` impl + `check_functor_laws` test |
| [`adjunction-author`](.claude/skills/adjunction-author/SKILL.md) | Step 9; scaffolds adjunction with unit/counit and gap-analysis test |
| [`per-ontology-readme`](.claude/skills/per-ontology-readme/SKILL.md) | Step 4; generates `README.md` from `ontology.rs` |
| [`per-ontology-citings`](.claude/skills/per-ontology-citings/SKILL.md) | Step 4; extracts in-code citations into `citings.md` |
| [`per-ontology-mermaid-internal`](.claude/skills/per-ontology-mermaid-internal/SKILL.md) | Step 4; internal-structure diagram |
| [`per-ontology-mermaid-external`](.claude/skills/per-ontology-mermaid-external/SKILL.md) | Step 4; functor/adjunction diagram |
| [`per-ontology-rollout`](.claude/skills/per-ontology-rollout/SKILL.md) | Step 4; wraps all four `per-ontology-*` in one call |

Invocation conventions from `.claude/skills/README.md`:
- Self-contained — each skill has everything it needs in its `SKILL.md`.
- **Read-modify-verify-report** — every skill reads, edits, runs tests, reports. **None auto-commit.**
- Cite the test — every numerical claim is paired with the command that re-derives it.
- Modest framing — drafts, not finished work.

Typical use pattern:

1. You identify a source paper (Step 1-2).
2. You invoke `ontology-from-paper` with the paper as input.
3. The skill produces `ontology.rs`, `tests.rs`, `mod.rs`, candidate functors, candidate adjunctions, and a report.
4. **You read the report.** Every `todo!` is a place the skill refused to guess.
5. You fill in the `todo!`s, address any failing law checks, and run `cargo test --workspace`.
6. When green, you commit. Not before.

The key inversion vs. LLM workflows: **the skill is never the last word.** The laws check is.

---

## Part 6 — Verification prompts (pr4xis flavor)

In the LLM playbook, verification was "ask a closed-form question and see if it hallucinates". In pr4xis, verification is *always* a shell command with a deterministic answer. After any capability extension, run these and confirm the output:

```bash
# 1. Your new ontology's tests pass.
cargo test -p pr4xis-domains <topic>::tests

# 2. The category laws hold for every category in the workspace, including yours.
cargo test -p pr4xis category::validate::check_category_laws

# 3. Every functor in the workspace, including yours, satisfies the functor laws.
cargo test -p pr4xis category::validate::check_functor_laws

# 4. No pre-existing test regressed.
cargo test --workspace

# 5. (If you added an adjunction) the gap analysis produces a numerical report.
cargo test -p pr4xis-domains test_full_chain_collapse_measurement -- --nocapture

# 6. Count of functors — expected to grow by the number you added.
grep -rn "impl Functor" crates/domains/src/ crates/pr4xis/src/ | wc -l
```

From `docs/understand/concepts.md`: *"Every claim about the codebase is verifiable by `cargo test -p pr4xis category::validate::check_category_laws`, `cargo test -p pr4xis category::validate::check_functor_laws`, `grep -rn 'impl Functor' crates/domains/src/ crates/pr4xis/src/`, or `cargo test -p pr4xis-domains test_full_chain_collapse_measurement -- --nocapture`."*

If any of those six commands fails, the capability has not been added. There is no partial credit.

---

## Part 7 — Quick-reference recap card

```
TEACHING pr4xis A NEW CAPABILITY — 9 STEPS
────────────────────────────────────────────
1. State the capability as a one-sentence theorem about categorical structure.
2. Pick a citable source (textbook, standard, spec, CLI --help). No blogs.
3. Extract: entities, taxonomy rows, mereology rows, causation, opposition,
   domain axioms (each with a source citation), qualities with units,
   context-dependent names.
4. Scaffold crates/domains/src/<branch>/<topic>/ with ontology.rs, tests.rs,
   mod.rs, citings.md, papers/. Wire into parent mod.rs.
5. Write the define_ontology! block. Structural axioms come for free.
6. Add domain axioms as Precondition impls: name, description, total check,
   cited doc comment, proptest.
7. Verify structural laws:
     cargo test -p pr4xis-domains <topic>::tests
     cargo test -p pr4xis category::validate::check_category_laws
8. Compose with neighbors via functors. Verify:
     cargo test -p pr4xis category::validate::check_functor_laws
9. (Optional) Pair functors into adjunctions. Run gap analysis.

FINAL GATE: cargo test --workspace must be green.

HARD RULES
- Every entity, relation, and axiom must trace to a cited source line.
- If the source doesn't say it, it does not go in — even if "obvious".
- No floating-point equality in axiom checks — use epsilon or rationals.
- Preconditions must be total (Satisfied or Violated, never ambiguous).
- Never commit a functor whose laws fail. Fix the encoding or todo! honestly.
- Skills produce drafts. The laws check ships the code.

WHAT pr4xis "KNOWS" AFTER YOU FINISH
Not "PHP-shaped text that pattern-matches". Instead: a category whose
objects, morphisms, and axioms faithfully encode the source spec, composes
with every other ontology via proven functors, and surfaces its own
missing distinctions via adjunction gap analysis. Every answer carries
a trace from conclusion back to the axiom that licenses it.
```

---

*End of document. Nothing in your repository was modified. This playbook is written specifically against pr4xis's axiomatic architecture as documented in `README.md`, `docs/understand/concepts.md`, `docs/use/build-ontology-from-paper.md`, `docs/use/write-axioms.md`, `docs/use/compose-via-functor.md`, and the skills under `.claude/skills/` — not against generic LLM workflows.*