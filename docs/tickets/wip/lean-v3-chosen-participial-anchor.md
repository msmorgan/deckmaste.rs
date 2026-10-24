---
needs: [lean-v3-choose-over-coordination]
---
# "The chosen <object>" reads back what a choose chose

Standard constraints apply.

## Problem (probed 2026-10-23)

A Participial Description reads back what a Deed or a Zone Change did
(`Action.participialAnchor?`, `Check/Phrase.lean`). `choose` names no Deed (the `Deed` catalog
in `Words.lean` has none for choosing) and writes no Zone Change, so "the chosen card" cannot
anchor on it. "The chosen player" has its own predicate (`.chosenPlayer .theChoice`); no
object counterpart exists.

Probe (scratch, on `lean-v3-choose-over-coordination`):

```lean
sequentially [choose (a isCreature),
  tap (.described (.the .one) (and [isCreature, .did ⟨.any, choose (a isCreature), none, none⟩]))]
-- [lookbackSubject, unanchoredLookback, uniquifying]
```

The coordinated form, Green Sun's Twilight's "the chosen cards", is refused the same way.

## Witnesses

- *Green Sun's Twilight*: "If X is 5 or more, instead put the chosen cards onto the battlefield
  or into your hand". The choice is announced while the effect applies [CR#608.2d].
- *Stolen Uniform*: "Attach it to the chosen creature." Here the chosen creature is a target,
  chosen as the spell is cast [CR#601.2c].
- *Duel for Dominance*: "put a +1/+1 counter on the chosen creature you control. Then the chosen
  creatures fight each other." The fight half also needs `lean-v3-reciprocal-each-other`.

## Ruling (owner, 2026-10-24): option B, unified

- Replace `Predicate.chosenPlayer (ref : ChoiceRef)` with one sort-polymorphic
  `Predicate.chosen (ref : ChoiceRef)`, mirroring `Predicate.notChosen`, which dispatches on
  `drs.relativeKind.getD .object`.
- Keep the readings as conjunctions: `isChosenPlayer := and [isPlayer, isChosen]`; "the chosen
  creature" is `the (and [isCreature, isChosen])`; "the chosen cards" is the plural
  `the (and [isCard, isChosen])`.
- It reaches both kinds of chosen object: targets announced through "choose target …"
  [CR#601.2c], and choices made during resolution [CR#608.2d].
- Every `chosenPlayer` site migrates, with the same readings.

## Design (superseded by the ruling)

Either give `choose` an anchor that a Participial Description can read (a choosing Deed), or
add an object counterpart of `.chosenPlayer` keyed on the same Choice reference. In both, "the
chosen …" must reach targets that "choose" assigns to roles as well as objects announced on
resolution.

## Tests

- Extend `greenSunsTwilightOpening` and `stolenUniformOpening` (`Cards/Choice.lean`) to the whole
  card, as far as other gaps allow.
- A refusal pin: "the chosen creature" with no choose in scope.
