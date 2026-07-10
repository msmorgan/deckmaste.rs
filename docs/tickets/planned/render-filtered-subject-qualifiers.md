---
needs: []
---
**The event-subject renderer drops filter qualifiers, so filtered-subject
triggers fail the byte-exact fidelity gate and don't graduate.**

Surfaced repeatedly across the trigger families in [[macro-second-wave]]
(dies / enters / attacks / blocks / leaves-the-battlefield all share it).
Not a live mis-ship — the affected cards simply fail their round-trip and
stay ungraduated — but it caps the yield of every filtered-subject trigger.

## Problem

An event whose subject is a *filtered* predicate renders back to just the
bare card type, silently dropping every narrowing qualifier:

- `ZoneChange(what: And([Creature, Not(Ref(This)), ControlledBy(Ref(You))]), from: Battlefield)`
  should render "Whenever **another creature you control** leaves the
  battlefield, …" but renders "Whenever **a creature** leaves the battlefield, …".

The cause is the shared subject renderer: `find_card_type`
(`crates/deckmaste_cards/src/render/ability.rs`) walks an `And([...])` predicate
with `vs.iter().find_map(find_card_type)` and returns the FIRST card-type member
(`Creature`), so `subject_of` (`crates/deckmaste_cards/src/render/fragment.rs`)
prints only that type and never the sibling `Not(Ref(This))` ("another") or
`ControlledBy(Ref(You))` ("you control") clauses. Every event arm that feeds a
filtered subject through `subject_of` (dies/enters/attacks/blocks/LTB) inherits
this — the self and bare-"a creature" forms round-trip fine; only the qualified
forms fail.

## Fix

- Teach the subject renderer to reprint the common narrowing qualifiers on top
  of the base card type, at minimum:
  - `Not(Ref(This))` → the "another" prefix ("another creature …").
  - `ControlledBy(Ref(You))` → the "you control" suffix ("… you control").
  - (Stretch) `Subtype(...)` / color qualifiers → "a Goblin", "a red creature".
- Keep the existing bare output when no qualifier is present. Prefer a single
  shared subject-phrase builder so every event arm benefits at once (this is a
  render-side fix, not per-arm).
- Guard against double-printing "another" when the predicate already implies
  self-exclusion elsewhere.

## Acceptance

- A "Whenever another creature you control dies/leaves the battlefield, …" card
  round-trips faithfully and graduates.
- Existing self / bare-"a creature" trigger renders are unchanged (regression
  tests for dies/enters/attacks/blocks/LTB self forms still pass).
- Render round-trip tests for the qualified subject on at least dies and
  leaves-the-battlefield.

## Verification

- `cargo test --workspace` green; `cargo clippy --all-targets -- -D warnings` clean.
- Wipe-first wizards regen; graduation count rises as filtered-subject triggers
  round-trip.
