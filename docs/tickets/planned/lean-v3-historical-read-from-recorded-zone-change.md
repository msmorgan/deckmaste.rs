---
needs: [lean-v3-that-noun-after-move-historical]
---
# "If that artifact is put into a graveyard this way" finds no artifact (Tuktuk Scrapper)

Standard constraints apply. Found by
[lean-v3-that-noun-after-move-historical](../wip/lean-v3-that-noun-after-move-historical.md).

## Problem

The historical read of a moved object (`docs/decisions/semantics-v3.md` §1.5, the paragraph
beginning "A historical movement pattern such as 'that creature dies this way'") is keyed to the
lookback move's written origin. The trigger is `Location.readsDepartedPermanent` in
`lean/Semantics/Check/Phrase.lean`: a departure-observed move whose written origin includes the
battlefield. "Is put into a graveyard" writes no origin (`.anyLocation`). So after "destroy target
artifact", "that artifact" in the lookback finds nothing. The destroyed artifact is now a card in
a graveyard, and the bare permanent type word "artifact" does not reach it [CR#109.2]. "It" works.

Probe evidence. The probes were scratch `lake env lean` runs after
lean-v3-that-noun-after-move-historical, starting from `DRS.ofList []`, with `putGy at_ := .move
.actor .anyLocation (some graveyard) at_ []`:

- `[destroy (target isArtifact), doIf (did (that artifact) (putGy .arrival) thisWay) (draw 1)]`
  gives `[anaphor (word artifact) one 0]`.
- The same with `putGy .departure` gives the same refusal.
- With `did it (putGy .arrival) thisWay` it checks clean.

## Witnesses (data/derived/cards.jsonl)

- *Tuktuk Scrapper*: "Whenever this creature or another Ally you control enters, you may destroy
  target artifact. If that artifact is put into a graveyard this way, this creature deals damage
  to that artifact's controller equal to the number of Allies you control."
- *Afflicted Deserter // Werewolf Ransacker* (Werewolf Ransacker): "Whenever this creature
  transforms into Werewolf Ransacker, you may destroy target artifact. If that artifact is put
  into a graveyard this way, this creature deals 3 damage to that artifact's controller."

## Recommendation (proposed, not ruled)

This applies to a grounded "this way" lookback whose move writes no origin (`positiveZones?` is
`none`). Take the origin from the zone change recorded by the caused event the lookback matches
(`DRS.causedEvents`, `CausedEvent.zoneChange`), the same event `lookbackGroundRefusals` already
finds [CR#608.2c]. If that recorded origin is the battlefield, read the subject or patient
historically, as a written battlefield origin does today.

A destroy records the move from the battlefield to a graveyard. So "that artifact" would read
the destroyed artifact as the permanent it was [CR#608.2h], and the noun would still constrain
the read. A lookback with no matching event stays ungrounded, as now. A written origin keeps
deciding on its own, and a recorded event never overrides it.

Here a "this way" reads back an event its own text caused, which is a different question from
the trigger rule that "from anywhere" is not leaves-the-battlefield [CR#603.6c]. The ruling
should confirm that the distinction holds.

Changes:

- **ADR, `semantics-v3.md` §1.5:** the historical-read paragraph says where the origin comes from
  when the lookback writes none.
- **Code:** `stepDidClauseWith`, where the subject read is set up, and the `.move` arm's
  `readContext`. Both would consult the matched event when `from_` has no positive zones.
- **Tests:** *Tuktuk Scrapper* and *Werewolf Ransacker* spelled in full. A negative where the
  matched event's recorded origin is not the battlefield (an artifact card milled, then "if that
  artifact is put into a graveyard this way") stays refused.
