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

## Landing record (second landing, 2026-09-07)

Parts 0, the rename and the §11/§12 ADR text stand from the first landing.
Parts 1–3 are reverted and redone to the rulings above; the Rust corpus gate
lands. Parts 4 and 5 do NOT land — see the STOP and `## Handoff (second
landing)` below.

Measured on `zkootrrlnrox` (`docs: the v2 RON dialect after the second
landing…`), lock `covered` unchanged (no coverage lock is in play for this
lane), `nproc` 24, load average 6.28.

### PROVE

**No silent loss.** Nothing stopped being covered. `cargo xtask lean-check`
proves `plugins_v2/canon` 118/118 and `plugins_v2/testing` 2/2 before and
after every step; `cargo xtask facts check` reports both generated files up
to date and byte-identical (`FactsGen.idr` and `Facts.lean` both unchanged
this landing). No card, declaration, or rules row was edited: every step adds
readings the dialect did not have and removes readings the reverted mechanism
had.

The reverted commits took 11 reader tests with them. Each is accounted for:

| First landing's test subject | Here |
| --- | --- |
| `Green` at `ManaSymbol` reads the chain | re-spelled `a_bare_colour_reads_as_a_mana_symbol` |
| a native `Snow` is not routed | re-spelled `a_native_constructor_is_not_routed` |
| an unroutable identifier is refused | re-spelled `an_identifier_no_link_of_the_chain_claims_is_refused` |
| an AMBIGUOUS injection is refused naming both | RETIRED with its mechanism: one embed per enum makes the ambiguity unconstructible, and `macro_ron_derive` now refuses a second `embed` variant outright |
| over-applying a constructor is refused | RETIRED with its mechanism (the source rewrite that could over-apply is gone); the shape it guarded is now `a_mixed_application_is_refused` |
| `Simple(Specific(Of(Green)))` reads positionally | re-spelled `a_constructor_may_be_applied_positionally` + `the_named_and_positional_forms_agree`, and the one-field case is pinned as a refusal (`a_one_field_constructor_keeps_its_binder`) |
| `[2, Green, Blue]` is a mana cost | re-spelled `a_mana_cost_is_numerals_and_colours` |
| `3` is an `Amount` | re-spelled `a_bare_numeral_reads_as_an_amount` |
| `named_arguments_follow_the_positional_ones` (`#[ignore]`d) | re-spelled as the refusal `a_mixed_application_is_refused`; no `#[ignore]` survives |

**Structural laws.** The Lean build ends in success with no warnings (80
jobs). `cargo test -p deckmaste_semantics_v2 --test lean_drift` green
throughout: the mirror keeps its struct fields and its binder names, and the
markers are metadata the drift scan does not read. The emitter stays total and
every emitted card still proves — the injection's elided constructor reaches
Lean through the newtype-struct name, so `.simple (.specific (.of .white))` is
still what `Generated.Canon` carries even though the card file will write
`White`.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition, or card identity. The injection and numeral rules read
declared markers on the mirror; positional application reads the position's
own declared field count and serde's own field list; the unknown-field
refusal reads that same list.

### DISCLOSE

**The injection chain, each marked site with the card that wanted it** (for
veto):

| Marked | v1 counterpart | Card that wanted it |
| --- | --- | --- |
| `ManaSymbol::Simple { symbol }` | `ManaSymbol::Simple(SimpleManaSymbol)` (`mana.rs`) | every card with a mana cost — `Abbey Gargoyles` writes `Simple(symbol: Generic(amount: 2))` four times over |
| `SimpleManaSymbol::Specific { color }` | `SimpleManaSymbol::Specific(ColorOrColorless)` (`mana.rs`) | as above (`Specific(color: Of(color: White))`) |
| `ColorOrColorless::Of { color }` | `ColorOrColorless::Color(Color)` (`color.rs`) | as above, and every `ProducedRun` |
| `ColorTerm::Lit { color }` | none | NO card. Marked because the ruling names it; `ColorTerm` is reached only from `ManaTypeTerm::OfColor` and `Devotion`, which no canon card writes yet. Flagged for veto. |

v1's remaining three embeds have no mirror counterpart and are NOT mapped:
`ManaSpec::Specific(ColorOrColorless)` and `ManaProduction::Bare(ManaSpec)`
(v2's `ProducedMana` is a flat different shape — `Runs { runs }` — and carries
its riders as a field of `AddMana`, so neither wrapper exists), and
`StatValue::Count(Count)` (v2 has no `StatValue`; a stat value is an
`Amount`). Six v1 sites, three mapped, three with nothing to map onto.

**Further sites where a canon card visibly suffers — none marked, with the
census.** After the mana chain and the numeral leaves, the one-field
constructor spellings canon writes, by frequency: `SingleFaced(face:)` 117,
`Mana(cost:)` 34, `HasType(type:)` 26, `Target(quantity:)` 23, `And(…)` 23,
`Type(…)` 18, `Up` 15, `Static(spec:)` 14, `InZone` 13, `Sequentially` 11,
then a tail of ≤9. None is marked, for two reasons, both worth a ruling:

- `SingleFaced`'s payload is a STRUCT (`CardFace`) and `Cost::Mana`'s is a
  `Vec`; the mechanism needs `SupportsMacros` on the payload, which is an
  enum-only derive. These cannot be injections as the mechanism stands.
- The rest are semantic claims, not pure injections: marking
  `Predicate::HasType` makes a bare `Creature` read as a predicate, and
  `Predicate` has dozens of one-field constructors competing for one embed
  slot. Hand-choosing one is a design decision, not a mechanical consequence,
  so it is left for the user.

**The 148-pair ambiguity inventory ceases to exist**, as the ruling says: the
mechanism that enumerated 173 injections is gone, and one embed per enum is
now enforced by the derive rather than reported.

**New `SupportsMacros` types, hence new kinds and param types (5).** `embed`
is a `SupportsMacros` mechanism, so the chain's types had to move off the
plain `#[derive(Deserialize, Expand, Serialize)]`: `Color`,
`ColorOrColorless`, `SimpleManaSymbol`, `ManaSymbol` (`words.rs`) and
`ColorTerm` (`phrase.rs`). `ron::kinds()` and `ron::param_types()` register
each, which the two totality tests require. The consequence is that a macro
may now be declared at each of those positions — which is what
`Macros.lean`'s `thatColor : ColorTerm` will want in part 4.

**Deviations and additions.**

- `macro_ron_derive` `embed` now takes a struct variant of exactly one field.
  Unlike a NEWTYPE embed it keeps its tag (`is_named`), so
  `Simple(symbol: …)` — what every canon card writes today — still reads; the
  bare form is added, not substituted. A struct embed with two fields, or with
  a `#[macro_ron(default = …)]`, is a compile error.
- `macro_ron_derive` refuses a SECOND `embed` variant on one enum. The rule
  was stated but unenforced (`Input::embed` silently took the first).
- **A struct embed and a struct `literal` write bare through a newtype struct
  named `Type.Variant`, and `ron::raw_options` gains `UNWRAP_NEWTYPES`.** This
  is the one non-obvious piece and it is worth the reviewer's eye. `Serialize`
  has two consumers with opposite needs: the RON dialect wants the payload
  bare, and `lean_emit` (whose output must name every constructor or the card
  does not elaborate) wants the application. serde erases the type on the way
  down, so the elided constructor has to travel as data — the newtype-struct
  NAME, which ron's `unwrap_newtypes` drops and `lean_emit` reads. `.` is a
  legal RON raw-identifier character, so the name still validates with the
  extension off. The mirror declares no newtype structs of its own, so the
  extension reaches nothing else, and v1 is untouched (its newtype and tuple
  embeds still serialize their payload directly).
- `macro_ron` `literal` now takes a struct variant of one field and carries
  that field's binder into the splice (`Kind::literal_binder`), so the reader
  splices `Lit(value: 3)` — the spelling the variant reads — rather than
  `Lit(3)`, which is the one-field positional form ron cannot read (below).
- **The three kind-driven pre-scans in `deserialize_enum` now share one
  capture.** Each used to capture the value and, when it did not apply,
  re-read it with `Intercept::Skip` — which cancels every scan after it. That
  was invisible until a kind carried two of them: `SimpleManaSymbol` both
  embeds `ColorOrColorless` and is a numeral leaf, and its literal scan
  silently ate its embed (a bare `Green` stopped reading at
  `SimpleManaSymbol`). Capturing once and trying literal → embed → bare
  invocation in order fixes it generally. Behaviour change beyond the ticket:
  a kind that embeds another type AND hosts a bare-invocable macro now expands
  that macro, where before the embed scan cancelled it. Nothing in v1 or v2
  exercised the combination; the whole gate closure is green.
- The untagged-embed fall-through now also passes a NUMERAL down the chain:
  reaching that scan means the host has no numeral leaf of its own, so `2` at
  a `ManaSymbol` position becomes `Simple(Generic(2))`, exactly as `Green`
  becomes `Simple(Specific(Of(Green)))`. Without this `[2, Green, Blue]` is
  not a mana cost.
- **Positional application is a `MacroSet` switch**
  (`reading_positional_arguments`), off by default, on for v2 — §12's "all
  off by default and all consumer-declared". v1 reads exactly as before.
- Positional application reaches a PLAINLY serde-derived enum too, not only a
  `SupportsMacros` one. ron's own `VariantAccess::struct_variant` goes
  straight to `handle_struct_after_name` and reads the named form only, so the
  reader takes the NEWTYPE channel instead (`newtype_variant_seed` +
  `deserialize_any`): with `unwrap_variant_newtypes` the variant's parens
  become the content's and ron's lookahead picks the form. Without this,
  positional application would have covered only the ~24 `SupportsMacros`
  types out of 213 mirror types.
- New: `crates/deckmaste_semantics_v2/tests/corpus.rs`, the ruling's Rust
  corpus gate.
- Not done: parts 4 and 5 (STOP 2).

**STOPs.**

1. **A one-argument positional application cannot be read, and the reason is
   ron's, not ours.** `Hybrid(Generic(amount: 1), Red)` reads; `CountOf(x)`
   does not, and `CountOf(group: x)` must be written. Inside a newtype
   variant ron's `deserialize_any` classifies `( x )` with
   `check_struct_type(NewtypeMode::InsideNewtype,
   TupleMode::DifferentiateNewtype)`; a single element without a trailing
   comma is `StructType::NewtypeTuple` (ron 0.12 `src/de/mod.rs:340`), which
   clears the newtype flag and re-enters `deserialize_any` at the `(`.
   `handle_any_struct` (`src/de/mod.rs:174`) then classifies `(Bare)` as
   `StructType::Unit` and calls `visitor.visit_unit()` (`src/de/mod.rs:201`) —
   the argument's NAME is discarded before any visitor of ours sees it, so no
   adapter can recover it. Not resolved; pinned as a refusal
   (`a_one_field_constructor_keeps_its_binder`), never a misreading. The
   consequence for part 4 is concrete: a ported helper macro's body must write
   the binder at every one-field constructor.

2. **Part 4 (the helper macro layer) contradicts
   `deckmaste_construction_core`'s model of a declaration, and I stopped
   rather than choose.** The ticket says helper macros get "a plain meta with
   `name`, `kinds`, `params`, `body`, no spelling or grammar" under
   `plugins_v2/builtin/macros/<family>/`. But the rename this ticket already
   landed re-pointed `deckmaste_construction_core::macro_def::read_builtin_v2`
   at exactly those family directories, and that function reads EVERY `.ron`
   below `macros/` except `macros/meta/` and requires each to normalize into a
   spelled `NormalizedDeclaration`: `expected_builtin_identity`
   (`macro_def.rs:2844`) matches the path's first component against a closed
   list of nine family names and errors `UnexpectedBuiltinLocation` otherwise,
   and `normalize` builds noun/verb plans from spelling and grammar metadata a
   helper does not have. A one-file probe (`macros/pronouns/It.ron`, a plain
   `(name: "It", kinds: [NounPhrase], params: [], body: Pro(Bare, One, Whole))`)
   loads cleanly through `deckmaste_semantics_v2`'s reader — the dialect parts
   1–3 make the body writable, and the corpus test stayed green — and fails
   `deckmaste_construction_core`'s `builtin_v2_*` tests at the first file.
   Two ways out, and both are the user's call, not mine:
   (a) house helpers where `read_builtin_v2` does not look — a sibling
   excluded the way `meta/` is (`macros/helpers/<family>/`) — which keeps
   construction_core's "a declaration is a spelled construction" model intact
   and costs one `filter` there; or
   (b) give construction_core an unspelled-declaration kind, which is a real
   change to that crate's model and to what english_v2 reads.
   The ticket's letter picks neither, and CLAUDE.md's landing contract says a
   ticket-vs-reality contradiction is a STOP even when I can resolve it.

3. **Part 5's load-and-reserialise would delete every card's comments.** The
   conversion is "load-and-reserialise … so nothing is rewritten by hand", but
   `plugins_v2/canon` cards carry a leading `//` comment with the card's real
   oracle text and its CR citation (`Abbey Gargoyles`: `// "Flying, protection
   from red" [CR#702.16a]`), and some carry interior comments — the first
   landing's STOP 2 names `Eumidian Terrabotanist`'s. ron's writer emits no
   comments, so a straight reserialise loses them. A converter that preserves
   the leading comment block and rewrites only the value is available; interior
   comments are not recoverable that way. Which loss is acceptable is a ruling,
   not an implementation detail. Part 5 is unstarted.

**Glossary.** No term this landing needed is missing from
`docs/contexts/game-model/CONTEXT.md`. "Injection" and "numeral leaf" are
reader vocabulary, defined in `semantics-v2.md` §11.1 and beside the markers on
the mirror.

**Assurance counts.** Restored 0, re-spelled 8 (the table above),
retired-with-mechanism 2 (also above, each named with the mechanism that made
its subject unconstructible), ignored-with-blocker 0 (the first landing's one
`#[ignore]` is gone, re-spelled as a refusal), added 16 (14 in
`tests/reader.rs`, 2 in the new `tests/corpus.rs`), removed 0 beyond the
reverted commits' own.

### REPORT

- `cargo xtask lean-check`: `plugins_v2/canon` 118/118, `plugins_v2/testing`
  2/2. 108 s cold (step 2, warm Lean cache), under a second warm.
- `lean/scripts/build`: success, 80 jobs, no warnings.
- `cargo xtask facts check`: both generated files up to date; neither
  regenerated this landing.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: `plugins_v2/builtin`
  1862 declarations across 12 kinds; `plugins_v2/canon` 118 cards;
  `plugins_v2/testing` 2 cards, 1 token, 1 sba / 1 conferral / 1 damage row.
- `cargo xtask cite check --list-noncompliant`: 0.
  `cargo xtask cite check`: 15706 citations, 0 stale.
  `jj diff --git --from <claim> --to @ | cargo xtask cite audit --diff`: 6
  sites, each read against its rule (two colour sites, one mana-type site,
  three mana-symbol sites). `cargo xtask cite bless` was run and its result
  DISCARDED: it registered nothing new and would only have pruned the same two
  now-uncited entries a previous landing (`facts-generator-sheds-v1`) already
  recorded as a prune-only diff.
- `cargo xtask gate --changed` closure, run green in 3 m 20 s:
  `cargo test -p macro_ron_derive -p macro_ron -p deckmaste_construction_core
  -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics
  -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine
  -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon
  -p deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui
  -p deckmaste -p xtask`.
- `cargo fmt --all --check` clean; clippy clean on `macro_ron`,
  `macro_ron_derive`, `deckmaste_semantics_v2`, `deckmaste_semantics`.
- Injections declared: 4, across 4 types. Numeral leaves: 2. New
  `SupportsMacros` types / kinds / param types: 5.

## Handoff (second landing)

Steps 1–5 and 8 of the second brief are landed and green; the workspace is
parked with `@` empty and NOT integrated.

**Next: part 4, the helper macro layer** — blocked on STOP 2 above, which
needs a ruling on where an unspelled helper declaration lives. With that
settled the port is mechanical but not small; here is the recon, so the next
implementer does not re-derive it.

- `lean/Semantics/Macros.lean` holds 409 `semantic_macro` declarations. Their
  return types: `Instruction` 95, `NounPhrase` 72, `Predicate` 49, `Ability`
  41, `StaticSpec` 30, `Amount` 23, `GameEvent` 21, `ZoneExpr` 20, `Condition`
  13, `Quantity` 6, `Subtype` 6, `ManaSymbol` 5, `Duration` 4, `CounterKind` 3,
  `ColorTerm` 2, `Cost` 2, `CharacteristicBundle` 2, `JoinedHeader` 2, and one
  each of `Bindings`, `RollRow`, `HeaderPossessor`, `LevelBand`,
  `PrototypeFrame`, plus `List Predicate`.
- The file's own section structure gives the family directory names, in file
  order: Pronouns, Quantities, Determiners, Zones, Predicates, Nouns, Mana,
  Durations, Amounts, Counters, Instructions, Events, Abilities, Cards.
- 50 of the 409 are NOT term-shaped and are the STOP candidates: 21 call a
  `Primitives.*` helper (a Lean function, not a constructor — `Primitives.
  NounPhrase.eitherOf`, `Primitives.Instruction.offer`, `Primitives.StaticSpec.
  partScope`, …), 24 compute (`.map`, `match`, `let`, `<|`, `++`,
  `subject.plur`), and 5 are defined by pattern matching on an argument
  (`itOrThem`, `sameWindow`, `agentPlur`, `chooseSpree`, `partyRoles`).
- A converter must be TYPE-DIRECTED, not name-directed: 25 constructor names
  with a one-field form are declared by more than one enum (`And`, `Or`,
  `Not`, `Lit`, `Of`, `Target`, `The`, `Keyword`, `Parameter`, `Mana`,
  `Static`, `Written`, … — the full list is reproducible by scanning the
  mirror), so `.not p` resolves only against the expected type. Walking down
  from the macro's declared return type through each constructor's declared
  field types resolves every one; a global name table does not.
- STOP 1 above is a converter constraint: a one-field constructor must be
  written with its binder (`CountOf(group: …)`), a two-or-more-field one may
  be positional (`Pro(Bare, One, Whole)`).
- The parameter types Lean's macro signatures name (`Deed`, `NounWord`,
  `Plurality`, `VerbedMarking`, `Nat`, …) are mostly NOT registered param
  types: `ron::param_types()` carries the `SupportsMacros` kinds plus a
  handful of aliases. Each type a ported signature names needs registering
  there, or the declaration is unregistrable. `Nat` has no v2 type; `Any` is
  `macro_ron`'s own.
- Per the ruling, parameter lists are `Params::Named` and Lean's named
  defaults become defaults on the parameter. §7 and §12 of the ADR now say so.

**Then: part 5** — the macro-only refusal (which cannot be turned on until
part 4 exists: canon writes raw constructors everywhere today) and the
load-and-reserialise conversion, blocked on STOP 3's ruling about comments.
When it runs: the writer already elides injections and numeral leaves, so a
reserialised `Abbey Gargoyles` cost becomes `[2, White, White, White]`.
`cargo xtask facts check` must stay byte-identical and `lean-check` 118/118 —
run it in the loop, not only at the end.


## Rulings after the second landing (user, 2026-09-07)

- **One-argument positional application reads.** ron classifies `(x)` in a
  newtype variant as a newtype tuple and drops the identifier, so the
  reader uses macro_ron's existing value capture: if the first token after
  `(` is not `ident:`, the captured value is re-read as a sequence. No
  rewriting; classification only. `CountOf(x)` reads.
- **construction_core reads only its nine spelled families by name** and
  ignores every other directory under `plugins_v2/builtin/macros/`. Helper
  macro declarations live beside them under `macros/<family>/` with a plain
  meta (`name`, `kinds`, `params`, `body`), no spelling or grammar.
- **Comments are preserved by splice.** The one-off card converter keeps
  each file's leading comment block verbatim and reserialises only the
  value; interior comments are counted, listed in the landing record, and
  re-placed by hand. The converter is deleted afterwards.
- **Non-term Lean macros stay Lean-only.** The 50 `semantic_macro`s that
  are functions, compute from their arguments, or pattern-match are not
  substitution bundles and do not port. List them in ADR §12;
  `lean-macros-from-ron` generates around them. The 24 computing ones are
  routed by name to `semantics-v2-macro-capture-and-plurality`.
- **The remaining v1 embed sites are deferred** until part 5's conversion
  shows which positions suffer; the landing record lists the candidates
  with the card that wants each, for veto, and a follow-up ticket carries
  the marking.

## Landing record (third landing, 2026-09-07)

The second rulings' first two items land, and part 4 (the helper macro layer)
lands. Part 5 does NOT: its first half is blocked by STOP 1 below, and its
second half has no purpose without it. See `## Handoff (third landing)`.

Measured on `qwpuupnkxtus` (`docs: the helper macro layer…`), `nproc` 24, load
average 1.24.

### PROVE

**No silent loss.** Nothing stopped being covered. `cargo xtask lean-check`
proves `plugins_v2/canon` 118/118 and `plugins_v2/testing` 2/2 before and
after every step. `cargo xtask facts check` reports both generated files up
to date and byte-identical — neither was regenerated this landing. No card,
declaration, or rules row was edited: every step either adds a reading the
dialect did not have, narrows which directories a reader looks in, or adds
new declaration files beside the existing ones.

Two tests changed subject and are re-spelled, not deleted:

| Subject | Here |
| --- | --- |
| `a_one_field_constructor_keeps_its_binder` — a one-argument positional application is refused | re-spelled as the positive `a_one_field_constructor_may_be_applied_positionally`, plus `a_one_field_argument_may_be_a_named_application` for the case the classification could get wrong (a one-field constructor whose ARGUMENT is binder-led) |
| `unknown_builtin_nursery_locations_fail_closed` — a nursery directory outside the nine families is refused | re-spelled as `a_nursery_directory_outside_the_nine_families_is_not_read`, which asserts the same reader over the same tree with the outcome the ruling changed; the location check INSIDE a family is unchanged and still pinned by `builtin_reader_rejects_malformed_and_nonfinal_locations`, which keeps its other three cases |
| `both_readers_accept_every_builtin_declaration` (xtask) — the two readers name the same declarations | re-spelled to compare over the SPELLED families only, whose set it reads off construction_core's own reading rather than restating |

**Structural laws.** The Lean build ends in success with no warnings (80
jobs). `cargo test -p deckmaste_semantics_v2 --test lean_drift` green
throughout: no mirror type, variant, field or binder changed — the landing
adds registrations (`kinds()`, `param_types()`) and declaration files, and the
drift scan reads neither. The emitter stays total and every emitted card still
proves.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition, or card identity. The one-argument classification reads the
argument list's own leading token; the nine-family read is a directory list,
which is the declaration model's own vocabulary, not a word's; the converter
that wrote the helper declarations resolved every constructor against the type
the position declares.

### DISCLOSE

**What the dialect now accepts.** `CountOf(x)` — a one-field constructor
applied positionally. ron cannot classify it: inside a newtype variant a
single argument without a trailing comma is `StructType::NewtypeTuple`, which
clears the newtype flag and reads the argument as a bare value, discarding the
identifier before any visitor of ours is reached. So the reader classifies the
argument list itself, from `macro_ron`'s value capture — the deserializer sits
INSIDE the variant's parens, so what comes back is `group: x` or `x` — and
hands the list back to ron in the parentheses it was written in, as the named
struct when it is binder-led and as a one-element tuple when it is not. An
empty list (`C()`) stays the named form, so a wholly defaulted constructor
reads as the zero-entry map it always did.

**Part 4, the helper macro layer.** 279 of `lean/Semantics/Macros.lean`'s 409
`semantic_macro`s are now one plain declaration each under
`plugins_v2/builtin/macros/<family>/`, the families being the Lean file's own
sections. `semantics-v2.md` §12.1 records the layer and lists every macro that
stayed in Lean, in seven buckets with the reason for each; the counts are 29
whose name is a constructor of their own kind (STOP 1), 15 the spelled keyword
families already declare, 28 that call a hand-written `Primitives` helper, 23
that compute, 28 that call one of those, 4 that expand to no single position,
and 3 defined by pattern matching. The ruling's "50 Lean-only" is 130: the
second landing's recon put 21 macros in the `Primitives` bucket on the premise
that `Primitives.*` is a Lean FUNCTION. It is not — `declare_semantic_primitives`
generates a `Primitives.T.ctor` alias for every constructor of every
`semantic_expression` type, so those convert as the constructor they alias,
and only the 28 that call the hand-written helpers in
`lean/Semantics/Macros/Primitives.lean` are blocked by that layer. The
remaining growth over 50 is the three buckets the recon did not have: the
name-is-a-constructor STOP, the keyword-declared identities, and the cascade.

Everything ported is registered: 49 new param types in
`deckmaste_semantics_v2::ron::param_types` (every mirror type a ported
signature names, list-typed ones under the plural of their element type —
v1's `Abilities` convention — and Lean's `abbrev`s under the alias's own
name), and four new hand-built kinds (`CharacteristicBundle`,
`HeaderPossessor`, `LevelBand`, `PrototypeFrame`), which are positions a
ported body expands to that are not `semantic_expression` types.

**The converter.** Type-directed, as the second handoff explains: a
constructor resolves against the type the position expects, walking down from
the macro's declared return type through each constructor's declared field
types, because 25 one-field constructor names are declared by more than one
enum. It also converts Lean's named arguments (`.enact v i (agent := a)`),
Lean's low-precedence application (`f <| x`), structure literals, tuples, and
a constructor field carrying `:= none` that the application leaves off. It
converts to a FIXED POINT: a macro that does not port takes its callers with
it, which is the 28-macro cascade bucket. It lived in the session scratchpad
and is gone; nothing of it entered version control.

**Deviations and additions.**

- `macro_ron::expand` gains `RawText`, a capture that takes ron's raw-value
  text without ron's own `RawValue`, whose `Deserialize` re-parses the capture
  and refuses anything that is not a standalone value — a fused argument list
  is not one.
- `MacroAware::deserialize_newtype_struct` narrows its raw-value interception
  from "not `Skip`" to `Full`: a raw capture of newtype-variant content
  (`SkipStructs`) is a fused argument list, not a whole value with holes of
  its own. A raw capture that gets past that branch now goes to the inner
  deserializer BARE, since its visitor wants `visit_borrowed_str` and `Wrap`
  does not forward it.
- `read_builtin_v2` takes its nine families by name (`BUILTIN_FAMILIES`)
  rather than reading the whole nursery and refusing what it cannot place.
  `META_DIR` is gone with the exclusion it served. File order within the
  reading is unchanged: the nine are listed alphabetically, which is the order
  the old recursive scan produced with `meta/` filtered out.
- New: `every_nullary_helper_expands` in `tests/corpus.rs`. Reading a
  declaration only checks its body is well-formed RON — the body is opaque
  text until something expands it — so this writes each nullary helper where a
  card would and reads it as the type its kind names. 96 expand; 289 nullary
  declarations at kinds the test does not dispatch (the declaration families'
  own loader tags) are counted and skipped.
- Not done: part 5 (STOP 1), and the 130 unported macros (§12.1).

**STOPs.**

1. **29 helper macros are named for a constructor of their own kind, and RON
   has no mark that separates the two. Part 5 cannot start until that is
   ruled.** Lean writes `.draw` for the constructor and `draw` for the macro;
   the dialect has only `Draw`, and native dispatch takes it. Such a
   declaration would LOAD — the reader's identity exemption covers it, since
   its body's head is its own name — and never be invocable, which is the
   shape `semantics-v2-macro-bodies-keyword-actions`'s STOP 1 already
   established as a defect. They are listed in §12.1; they include `draw`,
   `move`, `choose`, `triggered`, `activated`, `keyword`, `shuffle`, `doIf`
   and `delay`, the phrasings most of the corpus is built from. Part 5's first
   half is the macro-only card refusal, and Lean's own
   `Authoring.Form.onlyMacros` refuses EVERY constructor of a
   `semantic_expression` type outside the literal leaves
   (`lean/Semantics/Authoring.lean:317`), so a card that may write only macros
   needs a macro for every constructor it writes. With these 29 unreachable
   there is no such set, the refusal cannot be turned on, and part 5's second
   half — reserialising canon "with helper macros in place of raw
   constructors" — has nothing to put in their place. Not resolved: naming is
   the user's, and inventing a suffix convention here would be a dimension
   pinned by the implementer.
2. **The second landing's recon miscounted the `Primitives` bucket, and the
   ruling's "50 Lean-only macros" is 130.** Recorded above under part 4 and
   listed in §12.1 rather than left as a discrepancy between the ADR and the
   ruling. The ruling's CRITERION — functions, computing, pattern-matching —
   is unchanged and is what the buckets apply.

**Glossary.** No term this landing needed is missing from
`docs/contexts/game-model/CONTEXT.md`. "Helper macro", "spelled family" and
"argument list" are reader and plugin-format vocabulary, defined in
`semantics-v2.md` §§11, 12.1 and beside the code that reads them.

**Assurance counts.** Restored 0, re-spelled 3 (the table above), ignored-with
-blocker 0, added 3 (`a_one_field_constructor_may_be_applied_positionally`,
`a_one_field_argument_may_be_a_named_application`,
`every_nullary_helper_expands`), removed 0.

### REPORT

- `cargo xtask lean-check`: `plugins_v2/canon` 118/118, `plugins_v2/testing`
  2/2, 0.7 s warm.
- `lean/scripts/build`: success, 80 jobs, no warnings, 0.2 s warm.
- `cargo xtask facts check`: both generated files up to date; neither
  regenerated this landing.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: `plugins_v2/builtin`
  2141 declarations across 26 kinds (1862 before the port, +279); 96 nullary
  declarations expand; `plugins_v2/canon` 118 cards; `plugins_v2/testing` 2
  cards, 1 token, 1 sba / 1 conferral / 1 damage row.
- `cargo test -p deckmaste_construction_core`: 428 tests, unchanged in count
  (426 + the 2 that were failing before the re-spell).
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15725 citations, 0 stale (15706 before; the 19 new ones ride on the doc
  comments the port carried over from Lean).
  `jj diff --git --from <claim> --to @ | cargo xtask cite audit --diff`: 16
  sites, each read against its rule. `cargo xtask cite bless` was run and its
  result DISCARDED: it registered nothing new and would only have pruned the
  same two now-uncited entries `facts-generator-sheds-v1` already recorded.
- `cargo xtask gate --changed` closure, run green (107 `test result: ok`
  lines, 0 failures):
  `cargo test -p macro_ron_derive -p macro_ron -p deckmaste_construction_core
  -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics
  -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine
  -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon
  -p deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui
  -p deckmaste -p xtask`.
- `cargo fmt --all` clean; clippy clean on `macro_ron`,
  `deckmaste_construction_core`, `deckmaste_semantics_v2`, `xtask`.
- Helper declarations written: 279 across 13 families — `pronouns` 12,
  `quantities` 5, `determiners` 15, `zones` 19, `predicates` 51, `nouns` 34,
  `mana` 4, `durations` 4, `amounts` 8, `counters` 2, `instructions` 84,
  `events` 15, `abilities` 26. New param types: 49. New kinds: 4.

## Handoff (third landing)

Steps 1, 2, 3 and 5 of the third brief are landed and green; the workspace is
parked with `@` empty and NOT integrated.

**Next: step 4 (part 5), blocked on STOP 1 above.** It needs a ruling on how
the dialect writes a macro whose name is a constructor of its own kind — 29 of
them, listed in `semantics-v2.md` §12.1. Until then:

- The macro-only refusal has no satisfiable target. Lean's
  `Authoring.Form.onlyMacros` allows only macro calls, literal leaves,
  parameters and non-`semantic_expression` constructors; every canon card
  today is raw constructors throughout, and 29 of the phrasings that would
  replace them are unreachable.
- The card converter has nothing to substitute. Reserialising canon through
  the dialect alone (injections elided, numerals bare, applications
  positional) is available and would work — the writer already elides — but it
  is the cosmetic half, and running it first means converting the 118 files
  twice and re-placing their interior comments by hand twice.

When the ruling arrives, the shape is: (a) mint the 29 declarations under the
ruled names, which un-blocks most of the 28-macro cascade too; (b) turn the
refusal on behind the same kind of `MacroSet` switch as
`reading_positional_arguments`; (c) the one-off converter, keeping each file's
leading comment block verbatim and counting and listing the interior ones for
hand re-placement, over `plugins_v2/canon`, `plugins_v2/testing` and the
family bodies. `cargo xtask facts check` must stay byte-identical and
`lean-check` 118/118 — run it in the loop, not only at the end.

The other 101 unported macros are already routed: the 23 computing ones to
`semantics-v2-macro-capture-and-plurality` by name, and the deferred injection
sites to the new `semantics-v2-embed-candidates`.

## Rulings after the third landing (user, 2026-09-07)

- **Case is the mark.** Macros carry Lean's camelCase names verbatim
  (`draw`, `abilityWord`); constructors stay PascalCase. This is Lean's own
  `.draw` / `draw` distinction, and `lean-macros-from-ron` becomes an
  identity on names. Every macro is camelCase, keyword and family
  declarations included, renamed by converter — provided construction_core
  and english_v2 do not key spelling lookups on the declaration name's
  case (verify first and report; if they do, rename helpers only now and
  mint a follow-up for the families).
- **The `Primitives.*` aliases port** as the identity macros they are. The
  Lean-only set is the computing and pattern-matching macros: the computing
  ones routed by name to `semantics-v2-macro-capture-and-plurality`, the
  rest listed in ADR §12.1; the alias bucket in §12.1 is deleted once they
  port, and the record states the corrected count.
- Part 5 then proceeds as ruled after the second landing.
