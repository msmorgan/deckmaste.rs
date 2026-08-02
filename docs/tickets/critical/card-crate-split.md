---
needs: []
---
**Split `deckmaste_card` out of `deckmaste_core`: the engine's unit of card
definitions, sitting directly above the loose primitives.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§1). Core keeps the actual primitives the engine interprets
(Action/OneShotEffect/Reference/Predicate/…), testable independently; card
holds `Card`/`CardFace`/`Token`, layouts, type lines — the packaging of
primitives into a playable unit, and the natural home for the card-shapes
program (transform/saga/adventure/split; census §4).

## Scope

- Move the card/token definition types (and their direct dependents) into
  the new crate; `card → core` dependency only, never the reverse; engine
  depends on both.
- Coordination: independent of the authoring fork — whichever of this and
  `authoring-crate-fork`/`core-demacro` lands second adapts mechanically
  (the fork mirrors whatever crate layout exists; demacro strips both
  halves if this has landed).
- No semantic change.

## Gates

Standard constraints apply. Zero behavior change: full workspace suites,
idris-check, fidelity all unchanged; `cargo tree` proves core does not
depend on card.
