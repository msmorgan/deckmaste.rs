---
needs: []
---
# Type-line print order is spelling, not a core gate

`typeRank` and `typesOrdered` (`idris/src/Experimental/Words.idr`) freeze the
printed order of card types — Kindred, Enchantment, Artifact, Land, Creature,
Planeswalker, Battle, Instant, Sorcery — into a `So` gate on the card line
(`Experimental.idr`, `So (typesOrdered l.tys)`) and on token canonical form
(`tokenCanonical`). The pin `ProofsD.badCardTypeOrder` refuses
`[Creature, Artifact]`, a term with the same meaning as `[Artifact, Creature]`.

A type line's meaning is a **set** of card types. Which order they print in is a
spelling fact — the style guide's, not the CR's: [CR#205.1] says the type line
contains the card types and states no order, so the pin's cite is off-topic
besides. Under
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
this is English's marking inside the meaning, the named error.

## The change

- Core keeps the one semantic obligation strict ascent was smuggling:
  distinctness. Replace `So (typesOrdered …)` at both gates with a
  `typesDistinct` gate (pairwise, house style beside `colorsDistinct`).
- `typeRank`/`typesOrdered` leave core for the spelling boundary, where the
  linearizer orders the words; find the home in `crates/deckmaste_english_v2`'s
  constructions and move the table verbatim — a dropped row is a silent yes.
- `badCardTypeOrder` retires; a `badCardDuplicateType` pin (`[Creature,
  Creature]`) takes its place. Any bench card that relied on order for
  distinctness keeps elaborating unchanged.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/ProofsD.idr`, and the spelling-side home of the order
table in `crates/deckmaste_english_v2`.

## Acceptance

- No order predicate on card types remains in `idris/src/Experimental*`;
  distinctness is gated; the order table exists at the spelling boundary.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing;
  `cargo xtask cite check` 0/0 after the off-topic cite is removed.

Standard constraints apply.
