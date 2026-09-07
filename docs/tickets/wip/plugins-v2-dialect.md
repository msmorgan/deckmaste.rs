---
needs: []
---
**The v2 RON dialect: what a card may write.** Rulings (user, 2026-09-07)
after reading the first canon cards, which spell
`Simple(symbol: Specific(color: Of(color: Green)))` where v1 wrote `Green`
and the Lean bench writes `pip .green`. Five parts, in this order; one
claim, since each depends on the one before.

1. **Injections embed.** Every mirror constructor with exactly one field
   whose type is another syntax type (`ManaSymbol::Simple`,
   `SimpleManaSymbol::Specific`, `ColorOrColorless::Of`, `ColorTerm::Lit`,
   and the rest; enumerate mechanically from the mirror) carries
   `#[macro_ron(embed)]`, and single-field newtypes `#[serde(transparent)]`,
   so `Red` at a mana position reads through the chain. The mechanism is
   v1's (`crates/deckmaste_semantics/src/mana.rs` explains why embed rather
   than `serde(untagged)`); the rule is the user's from v1 and is not
   loosened. Where two injections in one enum would both accept the same
   text, the reader refuses with both names rather than guessing; report each
   such pair.
2. **Positional application, names optional.** Lean writes
   `.simple (.specific (.of .green))` and `exile x (agent := .you)`. The
   reader accepts positional arguments mapped by the constructor's declared
   field order, and named arguments in any position after them, for every
   struct variant; the mirror keeps its struct fields (binder names are the
   contract with Lean, and the drift test reads them). The field-order
   inventory the drift test already builds is the reader's table.
3. **Literal leaves.** Lean marks `Amount.lit` with `semantic_literal`; the
   reader reads a bare numeral there. Mark the mana pip and generic-symbol
   leaves the same way on both sides so `[2, Green, Blue]` is a mana cost.
4. **The helper macro layer.** Port every non-keyword `semantic_macro` in
   `lean/Semantics/Macros.lean` (about 330 of 401) to declarations under
   `plugins_v2/builtin/macros/<family>/`, one file per macro, so a card can
   write what the Lean bench writes (`target creature`, `graveyard`,
   `thisPermanent`, `sequentially`, …). Keyword families keep their
   declaration metas; helpers get a plain meta with `name`, `kinds`,
   `params`, `body`, no spelling or grammar. Lean's named defaults become
   explicit parameters (no default slots). This is ADR §6's direction:
   `Macros.lean` is later generated from these.
5. **Macro-only cards.** Lean's `spelled` refuses a card whose term holds a
   raw constructor outside `semantic_literal` leaves and macro parameters
   (`Authoring.Form.onlyMacros`). The v2 reader enforces the same on card
   text before expansion, with the same refusal shape (list the raw names).
   Then convert every `plugins_v2/canon` card and every family body by
   load-and-reserialise through the new dialect, so nothing is rewritten by
   hand, and `lean-check` proves every card.

0. **Unknown fields are refused.** The reader silently drops a field a
   constructor does not declare: Fading and Impending wrote `amount:` for a
   `quantity` field and read as a count-less removal; retired arguments
   (`conferral:`) passed unnoticed. Every reader path refuses an unknown
   field with the constructor and field named; a test writes one and
   asserts the refusal. Lands first, before the conversion below.

Also: rename `plugins_v2/builtin/macros/stubs/<family>` to
`plugins_v2/builtin/macros/<family>` and re-point `read_builtin_v2`,
`facts.rs`, `gate.rs`, and the tests; the files are no longer stubs.

Record the dialect in `docs/decisions/semantics-v2.md` §11 (what a card may
write) and §12 (what a macro may write). Standard constraints apply; the
gate closure includes construction_core, english_v2, semantics_v2, xtask.

## Landing record

Parts 0–3, the rename, and the §11/§12 ADR text landed. Parts 4 (the helper
macro layer) and 5 (macro-only cards, the load-and-reserialise conversion) did
not — see `## Handoff`.

Measured on `runpzyxvmovx` (`docs: the v2 RON dialect…`), quiet host,
`nproc` 16.

### PROVE

**No silent loss.** Nothing stopped being covered. `cargo xtask lean-check`
proves `plugins_v2/canon` 118/118 and `plugins_v2/testing` 2/2 before and
after every part; `cargo xtask facts check` reports both generated files up to
date. No card, declaration, or rules row changed shape: parts 0–3 add readings
the dialect did not have and refuse readings it should never have had, and the
rename moves files without editing one.

**Structural laws.** `cargo xtask facts check` is byte-identical except the
one line the rename owns: `FactsGen.idr`'s generated header names its source
directory, which moved from `plugins_v2/builtin/macros/stubs/keyword_abilities`
to `plugins_v2/builtin/macros/keyword_abilities`. Regenerated with
`cargo xtask facts generate`; `Facts.lean` is unchanged. The Lean build ends
in success with no warnings (80 jobs). `cargo test -p deckmaste_semantics_v2
--test lean_drift` green throughout: the mirror keeps its struct fields, so
the binder names stay the contract with Lean, and the new attributes are
metadata the drift scan does not read.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition, or card. The injection and numeral-leaf rules read declared
markers on the mirror; the unknown-field refusal reads serde's own declared
field list; positional application reads the declared binder order.

### DISCLOSE

**What the dialect now accepts** (each with a positive and a negative test in
`crates/deckmaste_semantics_v2/tests/reader.rs`):

- An unknown named argument is refused with the constructor and the field
  named, at a struct-variant position and at a plain struct position. A
  declared field still reads.
- `Green` at a `ManaSymbol` position reads as
  `Simple(symbol: Specific(color: Of(color: Green)))`. A native constructor
  (`Snow`) is not routed. An ambiguous spelling (`Lit(value: 1)` at
  `Quantity`, which injects an `Amount` twice) is refused naming `UpToOf` and
  `ExactlyOf`.
- `Simple(Specific(Of(Green)))` reads as the named form. Over-applying a
  constructor is refused naming it.
- `[2, Green, Blue]` is a mana cost and `3` is an `Amount`.

**Ambiguity inventory.** 148 injection pairs across 25 types share at least
one accepted identifier: `Predicate` 46, `Instruction` 29, `GameEvent` 15,
`ProducedMana` 10, `ChoiceDomain` 8, `KeywordParam` 6, `Condition` 5,
`Reach` 5, `DeonticPatient` 3, `Duration` 3, `PreventCut` 3, `Exposed` 2, and
one each in `Amount`, `CaptureInput`, `Causing`, `Concurrent`, `CountBound`,
`DeckTrait`, `FlipScope`, `HeaderPossessor`, `IgnoredOutcomes`, `Quantity`,
`Repetition`, `SpendPurpose`, `TokenQuality`. These are the ticket's "report
each such pair": they are not defects, they are positions where the injection
shorthand is unavailable and the constructor must be written. The reader
refuses such a spelling when one is actually written, naming both
constructors; none is written today (canon proves 118/118). The concentration
in `Predicate` and `Instruction` is the mechanical enumeration doing what the
ticket asked — every one-field constructor of a syntax type is an injection,
and those two types have many one-`NounPhrase` constructors.

**STOPs.**

1. **Trailing named arguments after positional ones do not read, and cannot.**
   The ticket asks for `Exile(x, agent: You)` mirroring Lean's
   `exile x (agent := .you)`. RON has no mixed application form: `C(a, b: c)`
   is neither a tuple nor a struct, and ron's own value scanner refuses it
   while capturing the value — before any reader hook runs. The only way to
   accept it is a source-level normalization pass over the document text
   before ron sees it, whose constructor→binder lookup is necessarily
   position-free and therefore ambiguous for 14 of the mirror's 259 struct
   constructor names (`And`, `Attachment`, `Chosen`, `Combat`, `Described`,
   `Keyword`, `Mana`, `Not`, `Or`, `Parameter`, `Run`, `Spell`, `UpTo`,
   `Zones` are each declared by more than one type with different binder
   lists), and which would also mis-target a same-named macro invocation. Not
   resolved. Recorded as an `#[ignore]`d test naming the blocker
   (`named_arguments_follow_the_positional_ones`). Wholly positional and
   wholly named applications both read.
2. **Two bugs in the positional-argument scan, found by the canon load, not by
   the unit tests.** The argument list was taken to open at the first `(` in
   the captured value — which a leading `//` comment can own
   (`Eumidian Terrabotanist`) — and an argument was not recognized as named
   when a comment preceded it or its binder was a raw identifier
   (`r#while:`, `Arrogant Wurm` through `Madness`). Both fixed in the
   numeral-leaf commit; both would have shipped had `lean-check` not been run
   before the part-2 commit was believed. Method note: the Rust gate does not
   cover the plugin corpus, so `cargo xtask lean-check` belongs in the loop
   for every dialect change, not only at the end.

**Deviations and additions.**

- The ticket says the injection marker is `#[macro_ron(embed)]`. It is
  `#[macro_ron(inject)]`, and the numeral marker is `#[macro_ron(numeral)]`
  rather than `literal`. `embed` and `literal` are live `SupportsMacros`
  markers with different semantics (one embed per enum, newtype/tuple variants
  only, a `visit_newtype_struct` hop, `Kind`-level registration) that v1 still
  depends on; reusing the names would have entangled the two mechanisms on the
  ~19 v2 types that derive `SupportsMacros`. The rule the ticket states is
  unchanged, and the module docs name v1's `mana.rs` as its origin.
- New: `macro_ron::shape` (`Shape`, `Constructor`, `Injection`, `ShapeSet`,
  `Syntax`) and `#[derive(Syntax)]`. The registry is built by the derive's
  `register`, which walks every type its fields name, so registering the roots
  registers the mirror's whole reachable graph. `ron.rs::shapes()` names 18
  roots; 217 shapes register.
- New: `Intercept::Injections`, a fourth interception level. Without it,
  whichever kind-driven pre-scan in `deserialize_enum` ran first silently
  cancelled the others by re-reading with `Skip` — a pre-existing shape the
  new branch would have inherited.
- `#[derive(Syntax)]` added to 213 mirror types (all but generic `Delta<T>`,
  which carries a hand-written impl); `#[macro_ron(inject)]` added to 173
  constructors, enumerated mechanically as the ticket directs.
- `WrapVariant` now carries the variant's own name so the unknown-field
  refusal can say `RerollStored` rather than the generated helper struct's
  `__InstructionRerollStored`. A `#[serde(rename)]` on that helper was tried
  first and reverted: ron writes a struct's serde name when `struct_names` is
  on, which `deckmaste_migrations` reads back, and a rename to the bare
  variant name can also collide with a registered kind.
- `read_builtin_v2` now skips `macros/meta/`, which the old `macros/stubs/`
  boundary excluded for free.
- Not done: parts 4 and 5, and the `#[serde(transparent)]` half of part 1 —
  the mirror declares no single-field newtype structs, so there was nothing to
  mark.

**Glossary.** No term the landing needed is missing from
`docs/contexts/game-model/CONTEXT.md`. "Injection" and "numeral leaf" are
reader vocabulary, defined in `macro_ron::shape`'s module docs and recorded in
`semantics-v2.md` §11.1.

**Assurance counts.** Restored 0, re-spelled 0, ignored-with-blocker 1
(`named_arguments_follow_the_positional_ones`, blocker named in the attribute
— STOP 1), added 11, removed 0.

### REPORT

- `cargo xtask lean-check`: `plugins_v2/canon` 118/118, `plugins_v2/testing`
  2/2, 37.5 s cold / 0.8 s warm.
- `lean/scripts/build`: success, 80 jobs, no warnings.
- `cargo xtask facts check`: both generated files up to date;
  `FactsGen.idr` regenerated for the rename's path (197 rows), `Facts.lean`
  unchanged.
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15700 citations, 0 stale. `jj diff --git | cargo xtask cite audit --diff`:
  0 sites (no citation changed).
- `cargo xtask gate --changed` closure, run green:
  `cargo test -p macro_ron_derive -p macro_ron -p deckmaste_construction_core
  -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics
  -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine
  -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon
  -p deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui
  -p deckmaste -p xtask`.
- `cargo fmt --all` clean; clippy clean on `macro_ron`, `macro_ron_derive`,
  `deckmaste_semantics_v2`.
- Shapes registered: 217. Injections declared: 173 across 74 types.
  Ambiguous injection pairs: 148 across 25 types.

## Handoff

Parts 0–3, the rename and the ADR text are landed and green; the workspace is
parked with `@` empty and NOT integrated.

**Next: part 4, the helper macro layer.** Port every non-keyword
`semantic_macro` in `lean/Semantics/Macros.lean` (409 declarations, of which
the keyword families already have declaration metas) to one declaration file
each under `plugins_v2/builtin/macros/<family>/`, plain meta with `name`,
`kinds`, `params`, `body` and no spelling or grammar. The dialect parts 1–3
landed are what makes those bodies writable: a body may write
`Pro(Bare, One, Whole)` where Lean writes `.pro .bare .one .whole`. Give the
ported macros POSITIONAL parameter lists — `Params::Named` is an unordered
map, so a named signature cannot be applied positionally, and STOP 1 means a
mixed application is unavailable either way. Lean's named defaults become
explicit positional parameters, per the ticket's no-default-slots rule.

The port wants a converter (Lean `semantic_macro` line → RON declaration),
not 330 hand-written files. Bodies that use Lean terms the mirror has no
constructor for, or that would need a default slot, are the ticket's STOPs.

**Then: part 5.** The macro-only refusal over card text before expansion
(Lean's `Authoring.Form.onlyMacros`, refusal listing the raw names), then the
load-and-reserialise conversion of `plugins_v2/canon`'s 118 cards and the
family bodies through the new dialect with a one-off converter that is then
deleted. Watch the ambiguity inventory above: the writer must NOT elide an
injection at one of those 148 pairs. `cargo xtask facts check` must stay
byte-identical, and `lean-check` must stay 118/118 — run it in the loop, not
only at the end (STOP 2).

## Rulings after the first landing (user, 2026-09-07)

Parts 0, the rename and the ADR text stand. Parts 1–3 as landed are
reverted and redone:

- **Injections use the existing `#[macro_ron(embed)]`**, extended to accept
  a struct variant with exactly one field (construct `Simple { symbol }`,
  write bare, one embed per enum as before). The `macro_ron::shape`
  registry, `#[derive(Syntax)]`, `Intercept::Injections` and the 173
  `inject` markers are removed. The chain is hand-chosen, never enumerated:
  `ManaSymbol::Simple`, `SimpleManaSymbol::Specific`, `ColorOrColorless::Of`,
  `ColorTerm::Lit`, then the rest of what v1 embeds (14 sites in
  `color.rs`, `mana.rs`, `stat_value.rs`) mapped onto the mirror, and any
  site where a canon card visibly suffers. Each marked embed is listed in
  the landing record with the card that wanted it, for veto. One embed per
  enum keeps the reader unambiguous; the 148-pair inventory ceases to exist
  with the mechanism.
- **Numeral leaves use the existing `literal` marker** (`Amount::Lit`, the
  generic mana symbol); `numeral` is removed.
- **Positional application is read at the deserializer**, not by text
  rewrite: the struct-variant helper deserializes through `deserialize_any`
  with a visitor accepting both `visit_map` (named) and `visit_seq`
  (positional, binder order). The rewrite pass is deleted. Mixed
  `C(a, b: c)` is refused, and the ignored test is re-spelled as that
  refusal.
- **Defaults on macro parameters are allowed.** The no-default-slots rule
  was an Idris carryover; Idris retires with v1. A helper macro declares
  Lean's named defaults as defaults, and generated Lean carries them
  natively. Retire the rule in `semantics-v2.md` and the family briefs.
- **The Rust gate reads the corpus:** a `deckmaste_semantics_v2` test loads
  every `plugins_v2/canon` and `plugins_v2/testing` card and every family
  body through the reader, no Lean involved.
- `lean-macros-from-ron` is promoted to planned and chained after this
  ticket; `semantics-v2-designation-storage-columns` stays engine-side
  (needs `lowering-v2`).
