---
needs: []
---
**Engine: a token made as a copy keeps the copied mana cost (and so its colour
and mana value) and loyalty.** Reachable: Offspring mints `Copy(source: SelfCard)`,
so live Offspring cards (Agate Instigator, {1}{R}) make a colourless token with
mana value 0; Populate is affected the same way.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #5.

## What is wrong at head

- `crates/deckmaste_engine/src/copy.rs:278-311` `token_from_copiable`: "`Token` has
  no `mana_cost`/`loyalty`/`defense` slot … those `CopiableValues` fields are dropped
  here"; builds `Token { name, color_indicator, supertypes, types, subtypes,
  abilities, power, toughness }`.
- `object.rs:142` `push_token`: `mana_cost: ManaCost::default()`, and `:151-152`
  `loyalty: None, defense: None`.
- `crates/deckmaste_core/src/token.rs:319-320` doc: "a token has no mana cost".
- `layer.rs:248-264` `base_colors` reads the mana cost and uses the colour indicator
  only `if colors.is_empty()`, a fallback rather than a union.

[CR#707.2]: copiable values include "name, mana cost, color indicator, card type,
subtype, supertype, rules text, power, toughness, and/or loyalty". [CR#202.1b]: a
token has no mana cost "unless the effect that creates them specifies otherwise".
[CR#202.2] derives colour from the mana cost; [CR#202.2e]: an object "is each color
denoted by that color indicator". Cackling Counterpart's ruling ("If the copied
creature has {X} in its mana cost, X is considered to be 0") presupposes the cost
carries over.

## Change

1. Mint copy tokens with the copied mana cost and loyalty: either give `Token`
   those slots or build the object straight from `CopiableValues`. Colour and mana
   value then derive from the copied cost. [CR#707.2] does not list defense;
   decide by reading the CR whether a battle copy carries its printed defense
   before adding that slot, and register it as underdetermined if the CR is silent.
2. Make `base_colors` the union of the mana cost's colours and the colour indicator
   ([CR#202.2,202.2e]).
3. Embalm ([CR#702.128a], "it has no mana cost") and Eternalize: today
   `Embalm.ron:17` / `Eternalize.ron:31` set colour with
   `Modify(Colors(Set([…])))`, which writes the colour indicator (`copy.rs:334`) and
   is right only because the cost is dropped. Give them an explicit "no mana cost"
   copy exception, and make the colour `Set` win over the cost-derived colour.
4. Correct the `token.rs` and `copy.rs` docs.

## Tests

- An Offspring token of a {1}{R} creature is red with mana value 2.
- A copy of a creature with {X} in its cost has mana value counting X as 0.
- An embalmed token is white with mana value 0.
