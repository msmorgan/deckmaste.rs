---
needs: []
---
**A static that changes a modal spell's own mode choice.** Ruling 2026-10-09
("A mode choice is changed by a static over the spell's own `chooseModes`",
`docs/decisions/semantics-v2.md` §7). Bodies entwine, escalate, spree and
tiered, four of the census residues. Standard constraints apply.

## Shape

- `Instruction.chooseModes (quantity) (modes : List (Option Cost ×
  Instruction))` already carries the quantity and per-mode costs
  [CR#700.2h]. Spree's "+ {cost} —" and tiered's "Name — {cost} —" bullets
  are authored on the modes; no new node for them.
- `StaticSpec.modeChoice (change : ModeChange)`, modifying the `chooseModes`
  of the spell it is printed on. `ModeChange` rows:
  - `allInstead (cost : Option Cost) (condition : Option Condition)`:
    entwine [CR#702.42a] and the 18 non-keyword "if COND, you may choose
    both instead" cards;
  - `extraModesCost (cost : Cost)`: escalate [CR#702.120a];
  - `setQuantity (quantity : Quantity)`: spree `atLeast 1` [CR#702.172a],
    tiered `exactly 1` [CR#702.183a];
  - `repeatable`: "you may choose the same mode more than once" [CR#700.2d]
    (22 cards);
  - `notChosenBefore (within : Duration)`: "that hasn't been chosen [this
    turn]" (21 cards).
- Mode weights: the mode entry gains `weight : Option Nat`, and
  `Quantity.worth (n : Nat)` writes "choose up to N {P} worth of modes"
  [CR#700.2i] (5 cards).
- Checker: kinds and references only. `modalFrameOk` stays a kind/shape
  check; it does not learn the choice's arithmetic. A `modeChoice` on an
  ability that has no `chooseModes` is refused as a shape.
- Rust mirror, RON helpers, bench macros spelled like the RON target.

## Re-spells

Every `chooseModes` site in `lean/Semantics/Cards/`, the bench and canon
gains the empty weight; same asserted outcome. Pins naming the modes list
keep their statements.

## Bodies

- entwine: `static(modeChoice(allInstead(some(Param(0)), none)))`
  [CR#702.42a].
- escalate: `static(modeChoice(extraModesCost(Param(0))))` [CR#702.120a].
  Its paid count reads `timesPaid(byKeyword(escalate))`: the existing
  facet, renamed `LinkedFacet.timesPaid` once `semantics-v2-linked-readback`
  lands. Use whichever spelling the tree has at claim time and name the
  other in the landing record.
- spree: `static(modeChoice(setQuantity(atLeast(1))))` [CR#702.172a].
- tiered: `static(modeChoice(setQuantity(exactly(1))))` [CR#702.183a].

## Witnesses (canon)

One canon card each for entwine, escalate, spree and tiered if canon lacks
one; a conditional "you may choose both instead" card (`allInstead(none,
some(COND))`); a pawprint card (`worth`); a "same mode more than once" card;
a "that hasn't been chosen this turn" card. Pins: each row checks; a
`modeChoice` with no modes to change is refused.

## Siblings

No dependency. `semantics-v2-linked-readback` renames the facet escalate
reads (above). All seven 2026-10-09 tickets touch
`Words`/`Phrase`/`Abilities`: refresh before integrate and expect conflicts.
