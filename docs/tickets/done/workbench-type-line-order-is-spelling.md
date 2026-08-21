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

## As landed (2026-08-21)

**Distinctness replaces order.** `typesDistinct : List CardType -> Bool`
(`Words.idr`, beside `colorsDistinct`, same pairwise `not (elem …)` idiom, on
the `Eq CardType` instance) is now the gate at both sites: `CardLine`'s
`{auto 0 dst : So (typesDistinct l.tys)}` (`Experimental.idr`, binder renamed
from `ord`) and `tokenCanonical` (`colorsDistinct t.colors && typesDistinct
t.line.tys`). `typesOrdered` is deleted; no order predicate over `CardType`
remains in `idris/src/Experimental*`.

**The order table's home is still pending.** `crates/deckmaste_english_v2`
builds no type line today — no `TypeLine`, `type_line`, or card-type spelling
anywhere in its `src/` (its only card-type contact is a `kinds = [Type,
Subtype]` construction field and subtype-lexeme scan tests). Nothing was
invented there. Per the ticket's fallback the table stays in `Words.idr`,
renamed `typeRank` -> `typePrintOrder`, with a docstring marking it
spelling-only, consumed by no gate, and awaiting english_v2's type-line
construction. **Follow-up:** move `typePrintOrder` to english_v2 when that
crate gains type-line construction; a dropped row is a silent yes.

**Pins.** Two order pins existed, not one — the ticket named only the card
side. Both retire, each replaced by its duplicate-type analogue in the
neighbouring house idiom (`badTokenDuplicateColor` is the model), and neither
carries a CR cite: [CR#205.1], [CR#205.2a], and [CR#205.2b] were read and none
of them states that a card's types are a set, so the claim is spelling's, not
the CR's.

- `ProofsD.badCardTypeOrder` (`[Creature, Artifact]`, cited [CR#205.1])
  -> `badCardDuplicateType` (`[Creature, Creature]`).
- `ProofsB.badTokenTypeOrder` (`[Creature, Artifact]`, cited [CR#111.3])
  -> `badTokenDuplicateType` (`[Creature, Creature]`).

Both bite: swapped to the now-admissible `[Artifact, Creature]` each fails with
`is not a valid impossible case`.

**Bench.** No card or token in `Experimental/` writes a duplicate type; the two
`[Creature, Artifact]` occurrences in the tree were exactly the two retired
pins. All 502 type-line sites keep elaborating unchanged.

**Not updated:** `docs/idris-workbench-closure-tables.md` rows 280 and 603 still
describe `typeRank`/`typesOrdered` as the census measured them; that document is
a historical measurement record, not a mirror of current source.

`idris/scripts/build` 18/18 clean, exit 0. `cargo xtask cite check
--list-noncompliant` 0; `cargo xtask cite check` 0 stale; audit selected 0 sites
(the diff removes cites and adds none).
