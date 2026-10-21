---
needs: []
---
# Unify numeric discourse anaphora (`thatMuch`, `theOutcome`, `groupSize`, `theDifference`)

`Amount` in [`Semantics/Abilities.lean:361-368`](../../../lean/Semantics/Abilities.lean)
currently has five separate constructors that read back a number from prior
discourse or the enclosing event/comparison:

- `Amount.thatMuch` (`"that much"` / `"that many"`)
- `Amount.theOutcome (sort : OutcomeSort)` (where `OutcomeSort` in
  `Semantics/Words.lean` is `rollResult | lifeLost | lifeGained | damagePrevented | damageMarked | excessDamage`)
- `Amount.groupSize` (`"that many"` reading the count of an event/acted-on object
  group, e.g. *Woodland Champion* and *Divine Visitation*)
- `Amount.theDifference` (`"the difference"` from an enclosing comparison, e.g.
  `Payload.difference`)
- `Amount.chosenNumber (ref : ChoiceRef)` (`"the chosen number"`)

## Design

- Assess unifying numeric discourse readback (`thatMuch`, `theOutcome`,
  `groupSize`, `theDifference`) into a single typed numeric-anaphor constructor
  (or parameterizing `thatMuch` / `theOutcome` by an optional `NumericSort`) so
  unqualified `"that much"` / `"that many"` and sort-qualified `"the life lost
  this way"` / `"the damage prevented this way"` / `"the difference"` / `"the
  result"` share one resolution mechanism in `Check/Phrase.lean`.
- Preserve all existing DRT ambiguity and scope laws:
  - Bare `"that much"` refuses when two eligible numeric outcomes are in scope,
    while a sort-qualified read (`lifeLost`, `damagePrevented`, `rollResult`)
    disambiguates by sort.
  - `theDifference` remains scoped to the comparison branch and does not leak
    across negation or disjunction.
  - `groupSize` continues to resolve both proposed event groups (*Divine
    Visitation*) and completed action groups (*Woodland Champion*).
