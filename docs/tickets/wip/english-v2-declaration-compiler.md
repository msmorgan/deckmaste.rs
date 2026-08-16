---
needs: [english-v2-engine-hardening]
---
**Stage 4: the declaration compiler MVP — generate the frozen golden
item-for-item, prove it diff-identical, then flip `deckmaste_english_v2` onto
the macro and delete the hand-written copies.** Authority
`docs/decisions/english-v2-rewrite.md` (§Crates, §The declaration language,
§Generated code contract, §Implementation sequence item 4 — read all four
first; item 4 records the settled mechanics this ticket implements). Every
item is pinned. If something cannot be completed as written — in particular
if any golden item cannot be expressed in the ADR's declaration vocabulary —
STOP and report; that is a design gap, not an implementer choice.

0. **Pre-freeze golden cleanup (behavior-neutral, lands first).**
   `Agreement` exists as two type-distinct enums with duplicated logic:
   render.rs has a private `Agreement` + `inflect` table +
   `agreement_for_pronoun`, and build.rs has a `pub(crate)` `Agreement` with
   its own `agreement_for_pronoun` and a second `inflect` copy in scan.rs.
   Unify: one `pub(crate) Agreement` in a new runtime module
   `src/features.rs` holding the single `inflect` table and single
   `agreement_for_pronoun`; render.rs, build.rs, and scan.rs consume it;
   both duplicates deleted. Zero test-content changes; all suites green
   unchanged.

1. **Crates: core + façade.** Two new crates.
   `crates/deckmaste_construction_core` — ordinary lib: declaration parsing,
   validation, and codegen over `proc_macro2::TokenStream`; fully
   unit-testable; also consumed by xtask for `expand`. Deps: `proc-macro2`,
   `quote`, `syn`, `prettyplease`. `crates/deckmaste_construction` —
   `proc-macro = true`, thin wrapper only: its sole exported
   `constructions!` macro delegates straight to core. Neither depends on anything else — nothing
   deletion-slated anywhere in the graph. `deckmaste_english_v2` gains
   exactly one new dependency: `deckmaste_construction`. xtask gains
   `deckmaste_construction_core`.

2. **Declaration surface.** One grammar-wide function-like
   `constructions!` invocation lives inline in `deckmaste_english_v2` source
   and contains every declaration: constructions, vocab, lexemes, terminal
   bindings, and roots. The macro NEVER reads a file: no `include!`, no path
   arguments, no build script, no OUT_DIR. Syntax comes from the ADR
   §The-declaration-language vocabulary and nowhere else; the MVP implements
   exactly the subset the emission table below requires — single-`form`
   constructions with one mandatory named `element` product and literal /
   role / `lex(..)` / `verb(lexeme)` /
   `noun(role)` atoms; `vocab` declarations generating enum + variant→word
   render fn; name-only `lexeme` declarations generating the enum alone;
   binding-only `codec` declarations with an explicit `lex`/`noun` atom class
   and intrinsic-identity `identity` declarations for existing runtime
   terminals; checked construction mappings to hand-written constructors;
   `require <role> is <Variant>` role refinements; the pinned
   agreement/number feature equations; and roots. Terminal bindings, checked
   construction mappings, and roots are the three counted metadata escape
   hatches. General `require` predicates, generative `codec`/`identity`
   bodies, and every other deferred ADR construct (`opt`, `seq`,
   `when`/`otherwise` multi-form, morphology, scanners, `derive` beyond the
   pinned feature equations) are hard "unimplemented in MVP" compile errors
   naming the construct — never a silent partial implementation.

3. **The emission table.** The compiler must generate, and the proof harness
   must cover, exactly these items — nothing more:
   - From the 19 construction declarations — ast.rs: category enums
     `Ability`, `Sentence`, `Clause`, `NounPhrase`, `VerbPhrase`, `Amount`;
     structs `Spell`, `Imperative`, `Declarative`, `WithWhere`,
     `EventClause`, `WhereClause`, `PronounNp`, `Common`, `DemonstrativeNp`,
     `TargetNp`, `CountNp`, `Destroy`, `Connive`, `DealDamage`, `GainLife`,
     `NumberAmount`, `VariableAmount`, plus the field-private SHAPES of
     `Triggered` and `SelfReferenceNp` (their `::new` impls stay
     hand-written beside the invocation); each with its exact golden derive
     set. render.rs: `impl Render for Ability`/`Sentence`,
     `render_sentence_body`, `render_clause`, `render_verb_phrase`,
     `render_noun_phrase`, `render_amount`, `agreement_for_noun_phrase`,
     `number_for_noun_phrase`. visit.rs: the `Visitor` trait and every
     `walk_*` fn. parser/rules.rs: `Category`, `Construction`, `RuleId`
     (with `construction()`/`index()`/`COUNT`), the `RULES` table.
     parser/build.rs: the `build` dispatch, one arm per rule.
   - From `vocab` declarations: `TriggerWord`, `Article`, `Demonstrative`,
     `Pronoun`, `Variable` enums + `render_trigger_word`, `render_article`,
     `render_demonstrative`, `render_pronoun`, `render_variable`.
   - From name-only `lexeme` declarations: `NounLexeme`, `VerbLexeme` enums
     (`VerbLexeme` keeps its extra `Ord`/`PartialOrd` derives).
   - Runtime residue, NEVER generated: context.rs and catalogs.rs entire;
     `Noun`, `CatalogIdentity` (+ impl), `SignedNumber`, `Sign`,
     `SelfReferenceSpelling` (codec/context-coupled value types);
     `Triggered::new`, `SelfReferenceNp::new`; the `Render` trait itself,
     `Writer`, `Number`, `CatalogCasing`, `CatalogKindCasing` (+ its
     `CatalogKind` impl), `pluralize`, `render_noun`,
     `render_signed_number`; `src/features.rs` (item 0); `Lexical`,
     `NounNumber`, `BuildValue`; everything in engine.rs, scan.rs,
     materialize.rs, selection.rs, error.rs, parser/mod.rs.

4. **The proof harness (transitional).** An integration test in
   `deckmaste_construction_core` embeds the full declaration text, runs core
   codegen, and compares against the golden: both sides parse with `syn` and
   normalize through `prettyplease::unparse`, then compare byte-exact per
   emission-table item; a mismatch fails with a unified diff of the
   normalized text. Golden items are read from the english_v2 sources via a
   `CARGO_MANIFEST_DIR`-relative path. The declaration text exists in two
   places only within this ticket's own commit sequence (harness now, crate
   invocations at the flip) and the harness is deleted in the flip commit —
   its passing run is recorded by the commit sequence, not kept as a
   permanent gate.

5. **The flip.** In one commit after the proof holds: the declarations move
   into `deckmaste_english_v2` as a new `src/constructions.rs` module (the
   macro invocation plus the hand-written `impl Triggered` /
   `impl SelfReferenceNp` blocks), and every emission-table item is deleted
   from its golden file. Post-flip layout, pinned: ast.rs keeps its runtime
   residue plus `pub use` re-exports of every generated AST item so `ast::*`
   consumer paths are unchanged; visit.rs becomes `pub use` re-exports of
   `Visitor` and the `walk_*` fns; render.rs keeps the `Render` trait and
   runtime helpers; parser/rules.rs is deleted with `Lexical` and
   `NounNumber` relocating to a new `parser/lexical.rs`; parser/build.rs is
   deleted with `BuildValue` relocating to `parser/materialize.rs`;
   parser-internal imports repoint to the generated items. `pub(crate)`
   visibility adjustments forced by module motion are permitted; the crate's
   public API surface is otherwise unchanged.

6. **`expand` affordance.** `cargo xtask english_v2 expand` uses core to
   parse the declarations out of the english_v2 source and prints the
   prettyplease-rendered generated items grouped by declaration; nonzero
   exit on parse or validation failure. This is the post-flip reviewability
   path for generated code.

The demonstrative construction is reproduced AS-IS: one rule, with number
derived from the stored `Demonstrative` word. Re-expressing it as two
declared forms is BANNED here — it changes `RULES` and breaks
diff-identical; it is stage 5's first multi-form declaration.

OUT OF SCOPE, stage 5+: scanner (word→variant) and morphology/`inflect`
generation; general `require` predicates (checked-constructor bodies stay
hand-written); generative `codec`/`identity` bodies; `opt`/`seq`/
`when`/`otherwise`/multi-form; doc-comment support in declarations; selection
growth; any engine/scan/materialize/selection behavior change (import paths
only); catalog changes; migrations.

Acceptance: item 0 lands first and behavior-neutral. The proof harness
passes covering every emission-table item before the flip. After the flip:
the golden's generated items and the harness are deleted; declarations live
only in english_v2; ALL behavioral suites green with ZERO test-content
changes across the entire ticket (a needed test edit means STOP and
report); both round-trip laws byte-exact on the five slice sentences;
`cargo clippy --workspace --all-targets -- -D warnings` clean;
`cargo xtask english_v2 expand` works as pinned; dependency edges exactly as
item 1 (english_v2 += deckmaste_construction only). Standard constraints
apply.
