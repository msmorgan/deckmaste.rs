---
needs: []
---
**Decide where a state-checked static ability lives, then finish hoisting
`Sba` out of `Ability::Static`.** Residue of `idris-sba-not-a-static-ability`
(2026-09-04): its conferral half landed (Aura's [CR#704.5m] rule is now
`Property::StateBased`), but `StaticEffect::Sba` stayed, on two grounds:
Ascend is a genuine static ability ([CR#702.131b]) whose only core spelling is
`Static(Sba(…))`, and v1 lowering is total over a source grammar that keeps
`Sba` until cutover.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched.

Ruling needed: a static ability whose effect holds while a game state holds
(Ascend; the testing Sagas) is a conditional static, not a state-based action.
Proposed: rename `StaticEffect::Sba { when, then }` to a conditional-static
spelling (the workbench already has `Conditionally c se` for exactly this),
so `Ability::Static` never says "Sba"; rules-defined SBAs stay on
`Property::StateBased` / `SbaRule`. Lowering's v1-facing input maps v1's `Sba`
onto the renamed variant until cutover. Re-spell `plugins/builtin/macros/keyword/Ascend.ron`'s
core-facing twin and `plugins/testing/cards/Test Saga*.ron`; keep
`deckmaste_plugin::keywords::ascend_macro_expands_to_static_sba` re-spelled
against the new name, never deleted.

Size: S–M. Done when: no `Sba` reachable from `Ability::Static` in
`deckmaste_core`; Ascend and the test Sagas lower and pass; workspace tests
green; workbench build at its module count. Standard constraints apply.

Ruling 2026-09-04: option (a) — rename `StaticEffect::Sba { when, then }`
to the conditional-static spelling mirroring the workbench's
`Conditionally c se`; lowering maps v1's `Sba` onto it until cutover;
rules-defined SBAs stay on `Property::StateBased`. Ready to run.

## As landed

**`deckmaste_core::StaticSpec::Sba { when, then }` → `StaticSpec::ConditionallyDo
{ when, then }`.** Field names, payload types and every consumer's behaviour are
unchanged; this is a pure rename plus the doc corrections it forces. `Ability::
Static` no longer says "Sba" anywhere: `grep -rn "Sba" crates/deckmaste_core/src/`
now matches only `SbaRule`/`SbaBody`/`validate_sba` — the rules-defined,
ability-free tier the previous round routed to.

**The name.** `Conditionally` was already taken by the wrapper that gates
another *static* ([CR#611.3a], the workbench's `Conditionally se c marking`),
so the instruction-payload sibling takes the adjacent spelling
`ConditionallyDo`: "any time `when` holds, do `then`". `StateChecked` was
rejected — one character of daylight from `Property::StateBased` is the exact
confusion this ticket exists to remove.

Sites (9 files):

- `crates/deckmaste_core/src/continuous.rs` — the variant and its doc. The doc
  now opens as a conditional static ([CR#604.1]) and keeps the [CR#702.131b]
  Ascend type case and the "NOT the home for a rules-defined SBA" routing note
  ([CR#704.1] → `Property::StateBased` / `SbaRule`) verbatim.
- `crates/deckmaste_core/src/region.rs` — the validator arm.
- `crates/deckmaste_engine/src/sba.rs` — the pass-1 collector arm, two test
  constructions (`aura_graveyard_sba`, the two Ascend statics) and the doc
  comments naming the variant.
- `crates/deckmaste_engine/src/legal.rs`, `condition.rs` — comments naming the
  variant's shape.
- `crates/deckmaste_lowering/src/continuous.rs` — the v1-facing map. `deckmaste_
  semantics::StaticEffect::Sba` (frozen, deletion-bound) now lowers to
  `deckmaste_core::StaticSpec::ConditionallyDo`, per the ruling's "until
  cutover" clause. Its unit test is re-spelled, not deleted:
  `lowers_static_effect_sba` → `lowers_static_effect_sba_onto_conditionally_do`.
- `crates/deckmaste_lowering/src/property.rs` — the conferral re-home arm
  ([CR#704.5m] `Property::StateBased`) reads the renamed variant.
- `crates/deckmaste_plugin/tests/keywords.rs` — `ascend_macro_expands_to_static_
  sba` → `ascend_macro_expands_to_static_conditionally_do`, re-spelled against
  the new name with the same card, the same drift guard against `ASCEND_GATE`
  and the same asserted outcome. The canonical-gate string it reads through
  `macros.read_str` stays v1 text (`Static(Sba(...))`) because that reader is
  the semantic one.
- `plugins/builtin/macros/keyword/Ascend.ron` — comment only; its data is v1
  source text (see the STOP below).

**Unchanged, deliberately.** `deckmaste_semantics::StaticEffect::Sba` and the
v1 RON that spells it (`plugins/builtin/macros/keyword/Ascend.ron`,
`plugins/testing/cards/Test Saga.ron`, `Test Saga Range.ron`,
`plugins/builtin/macros/types/enchantment/{Aura,Saga}.ron`); `SbaRule` and
`plugins/builtin/rules/sba/*`; `idris/`; `deckmaste_plugin::idris_emit`'s
v1 `StaticEffect::Sba` arm.

## Landing record

**Change id:** `wupxpxum` (`core: rename StaticSpec::Sba to ConditionallyDo,
the conditional-static home`) — every number below measured on that tree, with
`plugins/wizards` byte-identical before and after regeneration.

**Gates** (all foreground, from the feature workspace):

- `cargo check --workspace --all-targets` → `Finished dev profile
  [unoptimized + debuginfo] target(s) in 19.20s`
- `cargo fmt --check` → exit 0, no output (nightly-only-option warnings
  filtered)
- `cargo clippy --workspace --all-targets` → `Finished dev profile
  [unoptimized + debuginfo] target(s) in 21.62s`; **11 warnings, all
  pre-existing and all in files this diff does not touch**
  (`crates/deckmaste_lowering/src/card.rs`: 10 ×
  `unneeded_wildcard_pattern`; 1 × `items_after_test_module` for
  `static_observes_event` in `crates/deckmaste_lowering/src/ability.rs`). The
  round introduces none.
- `cargo test --workspace` → 127 suites, **6113 passed, 0 failed, 6 ignored**.
  The three failures the previous landing recorded as pre-existing
  (`ascend_gate_const_matches_canonical_condition`, the two `deckmaste_noncanon`
  `sped_red` cases) are green on this tree.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` → `checked 14198 citations against cr.txt (eff.
  2026-08-07); 0 stale`
- `cargo xtask cite audit --diff < /tmp/round.diff` → `audited 9 citation
  site(s)`; each rule text read against its claim. No `cite bless` was needed:
  every rule this round cites ([CR#604.1,702.131b,704,704.5m,704.5n]) is
  already registered in `cr-citations.lock`.
- `cargo xtask generate plugins/wizards` → re-ran; the tree hash is identical
  before and after (`d9f0e03638242e2e8cf5e790f018f2ff`), so no regeneration was
  needed. Expected: the generator's inputs are v1 semantic text, which this
  round does not change.
- Ticket gates: `grep -rn "Sba" crates/deckmaste_core/src/` → only `SbaRule`,
  `SbaBody`, `validate_sba`; Ascend passes
  (`sba::tests::ascend_permanent_grants_citys_blessing_at_ten`,
  `citys_blessing_is_multi_holder`, `ascend_macro_expands_to_static_
  conditionally_do`, `resolve::tests::ascend_*`); the test Sagas lower and
  render (`renders_saga_chapter_range_marker`,
  `renders_saga_chapter_roman_markers`,
  `a_multi_chapter_saga_face_round_trips_line_by_line`). The Idris workbench was
  not touched (sibling round owns it), so no workbench build was run.

**Assurance counts:** restored 0; re-spelled 2
(`deckmaste_lowering::continuous::tests::lowers_static_effect_sba` →
`…_sba_onto_conditionally_do`; `deckmaste_plugin::keywords::ascend_macro_
expands_to_static_sba` → `…_to_static_conditionally_do` — same subjects, same
asserted outcomes, new spelling); ignored-with-blocker 0; added 0; removed 0.

**Deviations and additions:**

1. No construction added or deleted; the round is a rename plus the doc lines
   the rename falsifies. Comment churn is confined to lines that named the
   variant.
2. `plugins/builtin/macros/keyword/Ascend.ron`'s **comment** gained "(core:
   `ConditionallyDo`)"; its data is unchanged (see STOP).

**STOP — the RON re-spelling the round brief asks for cannot be data.**

The brief says to re-spell `plugins/builtin/macros/keyword/Ascend.ron`'s
core-facing twin and `plugins/testing/cards/Test Saga*.ron`. Those three files
are **v1 semantic source text**, not core-facing data: `Plugin::card_from_str`
and `MacroSet::read_str` deserialize them into `deckmaste_semantics::Card` /
`…::Ability` and only then call `lower()`. Their `Static(Sba(…))` is
`deckmaste_semantics::StaticEffect::Sba`, which this ticket's own scope ruling
holds deletion-bound and untouched, and which the ruling's "lowering maps v1's
`Sba` onto the renamed variant until cutover" clause requires to keep its name.
Re-spelling the data would make all three files unreadable by the v1 reader.

Resolved in favour of the ruling: the data spellings stand, the core-facing
twin of each — what they lower to, and what the engine's own typed
constructions build — is what got re-spelled, plus Ascend.ron's comment naming
the core variant. Reported rather than invented.

**Residue for the coordinator to route** (not fixed here, out of the ruling's
scope): the test Sagas' [CR#714.4] sacrifice is a genuine rules-defined
state-based action ("This state-based action doesn't use the stack") authored
as a CARD-LEVEL static, so after this rename it lowers to
`Static(ConditionallyDo(…))` — an SBA still wearing a static ability. Its
taxonomically-correct home is a `Property::StateBased` conferral on the Saga
subtype (the shape the previous round gave Aura) or an `SbaRule`; the v2 Saga
stub declares neither today. Ascend is NOT in this bucket: [CR#702.131b] makes
it a static ability outright, and `ConditionallyDo` is now its honest home.
