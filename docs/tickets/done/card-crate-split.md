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

- Move the card definition types (`Card`/`CardFace` and direct dependents)
  into the new crate; `card → core` dependency only, never the reverse;
  engine depends on both.
- **`Token` and `TokenSpec` STAY in core**: the grammar itself creates
  tokens (`TokenSpec` is referenced from `copy.rs`/`continuous.rs`), so
  token definitions are grammar-adjacent, not loader artifacts.
- **Interface contract**: core carries NO characteristics abstraction.
  `trait BaseCharacteristics` is ENGINE-owned — defined in
  `deckmaste_engine` and implemented there for `card`'s types and for
  core's `Token`/`FaceDownCharacteristics` (local trait, foreign types;
  the orphan rule permits it) — unifying the three existing base sources
  behind one interface, with `layer::Characteristics` as the computed end
  (the layers pipeline reads base-in/computed-out).
  `CardRef<T: BaseCharacteristics>` is likewise engine-local, kept at the
  object/state boundary (object bases, stack copies, layers input) — NOT
  inside grammar enums, which stay monomorphic (grammar reaches
  definitions only via `TokenSpec` and registry names, the
  `tokens-predefined-registry` direction). A future shared
  characteristics-atoms crate may become the trait's home if it lands
  (`characteristics-atoms-crate`, design-gated); mechanics within this
  contract are the claimant's.
- Coordination: independent of the authoring fork — whichever of this and
  `authoring-crate-fork`/`core-demacro` lands second adapts mechanically
  (the fork mirrors whatever crate layout exists; demacro strips both
  halves if this has landed).
- No semantic change.

## Gates

Standard constraints apply. Zero behavior change: full workspace suites,
idris-check, fidelity all unchanged; `cargo tree` proves core does not
depend on card.
