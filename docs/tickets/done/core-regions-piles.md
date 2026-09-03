---
needs: [core-regions-discourse]
---
**Stage 4 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
piles are registers.** Types, validator, and lowering only; the engine
resolution stays in `engine-piles`, which needs this. Standard constraints
apply.

## Scope

- `SeparatePiles { dests: [DefId], group, by }` defines one pile register
  per pile [CR#700.3a]; `ChoosePile { dest, from: [RefId], by, random }`
  defines the chosen pile [CR#700.3b]. `Selection::PilesOf`,
  `PileSource::Labels`, and `PileSource::Noted` are deleted; a pile is read
  as `Reg`. `Value::Pile` is an object group, not an object.
- Lowering maps the authored pile labels of `plugins/canon/cards/Do or
  Die.ron` onto dests, so core carries no label. Removing the labels from
  the authored surface itself, and the Idris `DivideAndChoose` emit gap,
  stay in `core-do-or-die-divide-and-choose`.
- Validator: a `ChoosePile` reads only pile registers; `Each` over a pile
  register iterates its members.

## Gates

Do or Die lowers with no label in the core term, and the engine still
reaches its `todo!()` pile seam rather than a lowering error. Whims of the
Fates is the N-pile design target once authored.

## Landing record

- Construction count: core region `Kind` 4 → 5 (`Pile` added); core
  `Selection` 12 → 11 (`PilesOf` deleted); core `PileSource` 2 → 0 (type
  deleted); core `OneShotEffect` remains unchanged at 22 variants; engine
  activation `Value` 5 → 6 (`Pile` added). The authored semantic inventory
  is unchanged.
- Coverage and lock state: the canon inventory remains 80 → 80 cards and
  complete corpus lowering stays green, including Do or Die; Wizards
  regeneration graduated 0 cards and produced no tracked drift (23,365
  remain in progress). No coverage lock changed, and `cr-citations.lock`
  is unchanged.
- Assurance: restored 0; re-spelled 5
  (`lowers_one_shot_effect_separate_piles`, `lowers_separate_piles`,
  `lowers_pile_source_labels`, `lowers_pile_source_noted`, and
  `lowers_selection_piles_of`); ignored with blockers 0; added 5
  (`pile_registers_validate_as_iterable_groups`,
  `choose_pile_rejects_non_pile_registers`,
  `a_pile_value_is_an_object_group_not_an_object`,
  `do_or_die_reaches_the_engine_piles_seam`, and
  `do_or_die_lowers_pile_labels_to_registers`); removed 0.
- Positive artifacts: `cargo fmt --all -- --check`;
  `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D
  warnings`; focused core, lowering, engine, and plugin pile tests; canon and
  Wizards corpus lowering; `cargo xtask generate plugins/wizards`; citation
  scan 0 non-compliant strings and 17,644 checked with 0 stale; all 12
  changed citation sites audited against their rule text.
- Deviations and additions: no scope deviations. The scoped runtime
  `Value::Pile` carrier preserves last-known information while exposing pile
  members only through group reads. Authored noted-pile forms now stop at a
  labeled `engine-piles` lowering seam rather than recreating a core store.
  The existing `engine-piles` ticket was updated to own register writers,
  pile choice, shuffle, and the remaining cross-region noted-pile design.
- STOPs: none.
