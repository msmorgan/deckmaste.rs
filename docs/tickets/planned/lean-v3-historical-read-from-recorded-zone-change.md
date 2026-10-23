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

## Ruling (owner, 2026-10-22, batch 4): approved

When a grounded "this way" lookback's move writes no origin, take the origin from the matched
event's recorded zone change. A written origin still decides on its own, and a recorded event never
overrides it. [CR#603.6c] ("from anywhere") is unaffected, because this applies only to grounded
"this way" lookbacks: a "this way" reads back an event its own text caused, which is a different
question from the trigger rule that "from anywhere" is not leaves-the-battlefield [CR#603.6c].

## Change

Applies to a grounded "this way" lookback whose move writes no origin (`positiveZones?` is
`none`). Take the origin from the zone change recorded by the caused event the lookback matches
(`DRS.causedEvents`, `CausedEvent.zoneChange`), the same event `lookbackGroundRefusals`
(`Check/Phrase.lean:1246`) already finds [CR#608.2c]. If that recorded origin is the battlefield,
read the subject or patient historically, as a written battlefield origin does today. A destroy
records the move from the battlefield to a graveyard, so "that artifact" reads the destroyed
artifact as the permanent it was [CR#608.2h], and the noun still constrains the read. A lookback
with no matching event stays ungrounded, as now.

- **ADR, `semantics-v3.md` §1.5:** the historical-read paragraph says where the origin comes from
  when the lookback writes none, and says the rule is limited to grounded "this way" lookbacks.
- **Code** (line numbers re-verified 2026-10-22): `Location.readsDepartedPermanent`
  (`Check/Phrase.lean:1273`) is keyed to the written origin and has no event; `stepDidClauseWith`
  (`:1286`), where the subject read is set up through `Action.subjectReadsDepartedPermanent`
  (`:1281`); the `.move` arm's `readContext` (`:3474`). Both consult the matched event when
  `from_` has no positive zones.

## Tests

- `okTuktukScrapperThatArtifactPutIntoAGraveyardThisWay` and `okWerewolfRansackerThatArtifact…`:
  *Tuktuk Scrapper* and *Werewolf Ransacker* (Afflicted Deserter // Werewolf Ransacker) spelled in
  full, with the damage clause reading the destroyed artifact's controller.
- The Problem's probes become pins: `did (that artifact) (putGy .arrival) thisWay` and the
  `.departure` variant, after `destroy (target isArtifact)`, check clean.
- `thatArtifactMilledThisWayStaysRefused`: a negative where the matched event's recorded origin is
  not the battlefield (an artifact card milled, then "if that artifact is put into a graveyard this
  way") stays refused.
- `writtenOriginStillDecides`: a lookback with a written non-battlefield origin is not overridden
  by a recorded battlefield event.
- `fromAnywhereIsNotLeavesTheBattlefield` [CR#603.6c]: an existing pin on the trigger rule keeps
  passing (name the pin found by grep in the landing).
