---
needs: []
---
# Pin "that crewed it this turn" and "that saddled it this turn"

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against `wmyykqnv`. Paths are under `lean/Semantics/`.

## The gap

No card on the bench reads who crewed or saddled a permanent, though 7 Vintage-legal cards print
"… that crewed it this turn" and 5 print "… that saddled it this turn"
[CR#702.122b,702.122c,702.171c]. The crew and saddle keyword bodies (`Definitions/Keywords.lean:468-480`,
`:1536-1547`) pay a plain tap cost and stamp no deed, which looked like a missing antecedent.

It is not, and the bodies do not change. Every printing says "this turn": a lookback over the
turn's history, `did which (crew it) soFarThisTurn`. It reads the `does X (crew vehicle)` event,
which names `Deed.ofAbility (.named "crew")` through `Action.namedDeed`
(`Check/Phrase.lean:191-192`); saddle likewise. A body stamp would be needed only for a timeless
Participial Description read inside the same ability, as devour's "it devoured" is
[CR#702.82b], and no crew or saddle card prints one. Probed on 2026-10-25
(`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/q19-pins-probe.lean`):
both cards below elaborate as `spelled` and check `[]`.

The gap is that nothing pins this, and `lean-v3-deed-duplicates` step 1 deletes the
`EventVerb.crewed`/`.saddled` classes. These pins are its regression guard.

## Change

Add to `Cards/Keyword.lean`, beside Autarch Mammoth (`:1265`) and Debris Beetle (`:1271`):

```lean
/-- Getaway Car: "Whenever this Vehicle attacks or blocks, return up to one target creature
that crewed it this turn to its owner's hand." The creatures that crewed it are read from the
turn's history [CR#702.122b,702.122c], not from its crew ability. -/
def getawayCar : Spelled := spelled <| singleFacedCard
  { name := "Getaway Car", cost := [generic 3], types := [.artifact], subtypes := [vehicle],
    text :=
      [ haste,
        whenever (does thisVehicle attack) (or_ := [does thisVehicle block])
          (returnToHand
            (targets (upTo 1) (and [isCreature, did which (crew it) soFarThisTurn]))),
        crew 1 ],
    power := stat 4, toughness := stat 3 }

/-- Giant Beaver: "Whenever this creature attacks while saddled, put a +1/+1 counter on target
creature that saddled it this turn." [CR#702.171b,702.171c] -/
def giantBeaver : Spelled := spelled <| singleFacedCard
  { name := "Giant Beaver", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [beaver, mount],
    text :=
      [ vigilance,
        whenever (does thisCreature attack)
          (while_ := some (whileTrue (is thisCreature (hasDesignation saddled))))
          (putCounters 1 p1p1Counter
            (target (and [isCreature, did which (saddle it) soFarThisTurn]))),
        saddle 3 ],
    power := stat 4, toughness := stat 4 }
```

Before adding each, confirm its Oracle text, cost and P/T in `data/derived/cards.jsonl`. "While
saddled" follows `autarchMammothLine` (`Cards/Trigger.lean:775-779`).

## What must not change

The crew and saddle keyword bodies; `Action.crew` and `Action.saddle`.

## Pins

- Added: `getawayCar`, `giantBeaver` (spelled; each card checks itself).
- Added: `crewedItNeedsTheVehicle` in `Proofs/Deontic.lean` or beside the cards:
  `Action.check (DRS.ofList []) (returnToHand (targets (upTo 1) (and [isCreature, did which (crew
  it) soFarThisTurn]))) = [.anaphor .bare .one 0]` (probed): with no Vehicle in scope, the
  relative clause's "it" has no antecedent, so in the card it really reads the Vehicle.

## Acceptance

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates (bless
[CR#702.122c] and [CR#702.171b] if newly cited).
