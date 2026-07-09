---
needs: []
---
**Add an author-facing way to mark a static `Modify` as a characteristic-defining
ability (CDA) so it settles in layer 7a instead of 7b.** Unblocks the
"power/toughness equal to a count" card family (e.g. "* / * where * is the number
of X"), which was carved out of [[macro-second-wave]] because it cannot be
authored correctly today.

## The gap

A CDA (an ability that defines a characteristic — most commonly a creature whose
printed power/toughness is `*` set to some count) applies in layer 7a, *before*
other power/toughness-setting and -modifying effects. The engine already models
the distinction: the internal layer-pass struct carries `is_cda: bool`
(`crates/deckmaste_engine/src/layer.rs`, on `ContinuousEffect`).

The problem is purely authoring-side: **every construction site that builds a
continuous effect from ability/static data hardcodes `is_cda: false`**
(`crates/deckmaste_engine/src/layer.rs` — the sites that lower `Ability` /
`StaticEffect` / `Modification` into layer effects). There is no field on
`Ability`, `StaticEffect`, or `Modification` in `crates/deckmaste_core/` that an
author (or the macro/parser layer) can set to request 7a. So a card written as
`Static(Modify(This, Power(Set(CountOf(Objects(<filter>))))))` parses and
round-trips fine and produces the correct *value*, but is placed in layer 7b —
wrong ordering the moment it interacts with other P/T effects.

Shipping the 7b form anyway is the "convenient-but-wrong" pattern this project
fixes at source rather than grandfathering, which is why the card family was
deferred here rather than landed in the macro wave.

## Scope

- Add a CDA marker to the core static/modification grammar that the author (and
  the reverse macro index) can set — the minimal shape is a boolean/flag on the
  static-modify carrier, not a whole new effect variant (mirror how existing
  single-component statics extend). Decide the exact carrier during design; keep
  it representable in RON and round-trippable.
- Thread the flag through the engine lowering sites in
  `crates/deckmaste_engine/src/layer.rs` so a flagged static builds a
  `ContinuousEffect` with `is_cda: true` (→ layer 7a), leaving unflagged statics
  at 7b unchanged.
- Confirm the fixpoint layer pass orders a CDA-flagged P/T `Set` ahead of a
  plain 7b `Set`/`+N/+N` modification, with an engine test exercising the
  interaction (a `*`-P/T creature under a separate anthem).

## Then: land the deferred card family

Once the flag exists, author the `PowerAndToughnessCountEqual` / `PowerCountEqual`
`Modification`-family macros (the value shape is already expressible:
`Power(Set(CountOf(Objects(<filter>))))` with the primitives confirmed present)
plus the CDA flag, with an acceptance card that graduates and round-trips. That
card family is ~84 single-blocker cards.

## Verification

- `cargo test --workspace` green; `cargo clippy --all-targets -- -D warnings` clean.
- A layer-interaction test proves 7a-before-7b ordering for a CDA-flagged static.
- Wipe-first wizards regen; graduation count strictly rises once the family lands.
