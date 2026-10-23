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

## Decisions taken in the landing

1. **"Writes no origin" is `.anyLocation` (`Location.writesNoOrigin`), not "`positiveZones?` is
   `none`".** A negated origin ("from anywhere other than the battlefield", `.not`) also has no
   positive zones, but it is a written origin, and the ruling says a written origin decides alone.
   `writtenOriginStillDecides` pins it.
2. **The recorded path ignores the Observation Point.** The put Template marks a move that writes
   no origin `.arrival`, and the Observation Point classes a trigger pattern [CR#603.6c,603.10a],
   not the event a "this way" reads back, whose recorded Zone Change says where the object was.
   So both the `.arrival` and `.departure` forms read historically.
3. **When several matched events record different origins, a battlefield origin wins**
   (`DRS.thisWayRecordedOrigin`). The events record no referent, so the read cannot tell which
   event moved the named object; "that artifact" still constrains the read by its noun.

## Tests

- `Lookback.okThatArtifactPutIntoAGraveyardThisWay`: after `destroy (target isArtifact)`, `did
  (that artifact) …` in the `.arrival` and `.departure` forms, `did it …`, and `did you (putInto
  (that artifact) graveyard) thisWay` (the `.move` arm's patient read) check clean.
- `Lookback.thatArtifactMilledThisWayStaysRefused`: exile, then put from exile into a graveyard
  (the artifact once left the battlefield, but the matched event's recorded origin is exile), and a
  mill, stay refused.
- `Lookback.writtenOriginStillDecides`: a written hand origin, and a written `.not battlefield`
  origin, are not overridden by the recorded destroy.
- [CR#603.6c]: the existing pin `EventPatterns.anywhereAndBattlefieldHaveDifferentObservation`
  (the "from anywhere" pattern is observed on arrival) is unaffected. It pins the Observation
  Point, not a read, so the landing adds `Lookback.fromAnywhereIsNotLeavesTheBattlefield`: a
  lookback over this turn that writes "from anywhere" takes no recorded origin and still refuses
  "that artifact".
- The witness cards moved out: every witness writes "you may destroy target artifact", and a
  target in an optional body is invisible after it, which
  [lean-v3-optional-body-targets-stay-visible](../planned/lean-v3-optional-body-targets-stay-visible.md)
  carries (*Tuktuk Scrapper*, the *Werewolf Ransacker* trigger). *Werewolf Ransacker*'s upkeep
  ability also refuses `singular`, which
  [lean-v3-counted-cast-lookback-singular](../planned/lean-v3-counted-cast-lookback-singular.md)
  carries with the whole *Afflicted Deserter // Werewolf Ransacker* card.
