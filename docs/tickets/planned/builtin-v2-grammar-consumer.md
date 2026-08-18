---
needs: [builtin-v2-spelling-stub-design]
---
Implement the complete runtime path ratified by the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md):
load v2 declarations from a plugin dependency closure, compile their closed
grammar contributions and positional spelling frames, let
`deckmaste_english_v2` consume that environment, match the resulting English
AST against frames, and hand the assembled semantic RON to ordinary semantic
and Idris validation.

The v2 declaration schema lives in neutral `macro_ron` data types, separate
from legacy `MacroDef`: declaration kind replaces v1 `kinds:`, parameters are
positional only, `spelling` accepts literal text plus `<Param(n)>`, `grammar`
is optional closed data, and an ungraduated nursery record may omit a semantic
body. Do not add v2 fields to the permissive v1 reader, translate v2 records
through `template:`/`frames:`, or make english_v2 depend on
`deckmaste_english`, `deckmaste_construction_compiler`,
`deckmaste_constructions_macro`, or the splice-and-reparse machinery slated
for deletion. A stub contributes grammar but cannot be selected as a semantic
frame until it has a typed positional signature and body.

Keep ownership acyclic and explicit. `deckmaste_data` owns an owned,
serde-readable `CardSource`/`CardFaceSource` snapshot (names are illustrative)
for structural printed fields plus exact name-bearing English rules text;
Wizards-data adapters and hand-authored plugins both produce that same value.
`macro_ron` owns only the v2 declaration data. `deckmaste_english_v2` owns the
runtime grammar compiler, immutable `ParserEnvironment`, AST, parser, and exact
renderer. `deckmaste_plugin` owns dependency resolution, source discovery,
frame matching, compact semantic-RON assembly, and the existing semantic
validation pipeline. Pass a `&ParserEnvironment` to parser entry points; there
is no process-global or thread-local plugin vocabulary.

Add `cards_english/` as the plugin-local prose source lane. Each `.ron` file is
a `CardSource` envelope, not semantic RON: its rules-text fields remain English
until this compiler consumes them. Existing `cards/` remains the direct
semantic-RON lane, and both may coexist in one plugin. A duplicate card/face
identity across either lane is a load error rather than a precedence rule.
External importers may stream `CardSource` values through the same library
entry point without first writing plugin files; compilation produces the same
normalized semantic card either way. Generated semantic RON may be an explicit
cache or review artifact, but the loader never requires an English author to
check it in as source.

Loading is explicitly two-phase and closure-wide:

1. Resolve the ordered dependency closure, including the plugin being loaded,
   and read every v2 declaration header before reading any English-authored
   card.
2. Merge semantic declarations, grammar contributions, and spelling frames
   with one shadowing decision: a dependent plugin may replace a dependency's
   same-kind name; duplicate same-kind declarations within one plugin are an
   error; filesystem order never chooses a winner.
3. Compile one parser environment from core grammar, canonical catalogs,
   parse-context identities, and the merged contributions.
4. Read `cards_english/` and any importer-provided `CardSource` stream, parse
   each face's English under that environment, assemble semantic RON, then run
   the same semantic loader, normalization, lowering, and Idris checks used for
   direct RON input.

`grammar` is data for a closed set of compiler-owned runtime recipes, not a
plugin grammar DSL. Reuse the stage-5 terminal contract: whole-surface English
verb morphology, noun morphology, fixed term/clause/keyword surfaces,
terminal choice, context identity, and the already reviewed codecs. Every
`Verb` recipe requires exactly one valence from `Intransitive`, `Transitive`,
`Numerative`, or `Custom`; omission is a schema/load error. The first three
compile generic VP rules with respectively no complement, one object noun
phrase, or one amount expression. `Custom` carries an explicit VP-tail shape
set over the same closed compiler-owned grammatical categories and literal
atoms; the runtime compiler lowers those finite alternatives through the
generic extension point. A missing or empty shape set, an unknown category, or
an invalid atom sequence is a load error.

A declaration chooses a recipe and supplies its surface table; it cannot
submit Rust, callbacks, arbitrary productions, precedence, semantic guards,
or an untyped custom tail. Extend the generated grammar once with generic
extension points for the three ordinary valences and other closed recipes.
Do not generate Rust per plugin or mint a static construction per official
catalog entry. The runtime compiler validates recipe fields, duplicate
surfaces, declaration-category compatibility, custom tail shapes, and
deterministic merge order, reporting the plugin and source path for every
failure.

The contributed terminals and generic recipe rules enter the existing Earley
chart and packed forest. Ordered choice, PEG, recursive descent, a plugin-only
parser, and a fallback parse path are banned. Parse/render support for each
runtime recipe remains derived from one surface table, with the same exact
terminal round-trip law as statically declared terminals.

Compile `spelling` once into a typed partial pattern over the generated
English AST. `<Param(n)>` is resolved directly against the positional type
vector; an unknown type or out-of-range index is a load error. Matching uses
the generated total traversal/substitution surface, never source substrings or
textual splice-and-reparse. It recursively recovers typed arguments and
assembles the compact invocation `M(a0, ..., an)`; there is no named-parameter
environment or declaration-order lookup. Equally valid frame matches remain
an explicit ambiguity and are never selected by registration or filesystem
order.

Keep syntax and semantics separate without discarding syntax. Valence makes
`scry 1` and `scry X` grammatical and rejects the cross-family shapes
`destroy 1` and `scry each creature target opponent controls`. It does not
encode the positional semantic signature: the parser may still accept a
same-valence noun phrase that the declaration's typed spelling frame cannot
consume. Typed frame matching rejects that candidate, and Idris validates any
assembled card afterward. Do not add Magic legality, semantic guards, or
parameter-type refinements to the English parser.

Acceptance is an end-to-end synthetic plugin suite, independent of the
official vocabulary ceiling:

- a dependency declares a novel subtype and counter kind, and a dependent
  plugin uses both in English;
- one plugin declares a novel verb and uses it in an English-authored card in
  that same plugin;
- the same `CardSource` compiles identically when read from `cards_english/`
  and when supplied by an importer, while a cross-lane duplicate card identity
  is diagnosed;
- a literal amount and `X` both fill an `Amount` hole, while an unbound `X`
  reaches semantic validation and fails there rather than in the parser;
- representative declarations exercise all four valences; the parser rejects
  a direct object after an intransitive verb, an amount after a transitive
  verb, and an object noun phrase after a numerative verb;
- a syntactically valid but hole-type-incompatible same-valence reading reaches
  typed frame matching and has no match;
- `Custom` admits exactly its declared typed tail alternatives, and omission
  of valence or a missing/empty/invalid custom shape set fails
  deterministically;
- a same-kind redeclaration shadows its dependency coherently in semantics,
  grammar, and spelling, while a same-plugin duplicate fails deterministically;
- an ungraduated stub adds vocabulary but cannot assemble a semantic macro;
- direct RON and English input for the same compact positional invocation emit
  the same Idris term and pass the same validation path; and
- a synthetic declaration absent from every official catalog still parses and
  round-trips through its plugin-built environment.

Parser assertions inspect generated AST and selected frame identity directly;
full bracket strings are not golden contracts. Standard constraints apply.
