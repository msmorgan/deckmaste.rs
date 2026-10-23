---
needs: [lean-v3-optional-body-targets-stay-visible]
---
# "If a player cast two or more spells last turn" refuses `singular` (Werewolf Ransacker)

Standard constraints apply. Found by
[lean-v3-historical-read-from-recorded-zone-change](../wip/lean-v3-historical-read-from-recorded-zone-change.md).

## Problem

*Werewolf Ransacker*'s second ability: "At the beginning of each upkeep, if a player cast two or
more spells last turn, transform this creature." Spelled as
`did (a isPlayer) (cast (counted (orMore 2) isSpell)) lastTurn`, or with `.any` as the subject,
the condition gives `[singular]`.

## Diagnosis

The checker is right; the spelling is wrong. The refusal is `eventActionRefusals`
(`lean/Semantics/Check/Phrase.lean`): a procedure's patient in an event pattern must be singular,
because one casting casts one spell [CR#601.2a]. The same rule pins
`Trigger.badCastsPluralComplement` ("Whenever you cast all spells"). "Two or more spells" here
counts cast events, not the patient of one event.

Two spellings count events instead:

- `DidClause.ordinal`: "if a player cast a second spell last turn":
  `.did { subj := .actor (a isPlayer), action := cast (a isSpell), within := some (.duration
  lastTurn none), ordinal := some (.nth 2, none) }` checks clean. With it, the whole
  *Afflicted Deserter // Werewolf Ransacker* card refuses only the two `anaphor (word artifact)`
  that [lean-v3-optional-body-targets-stay-visible](lean-v3-optional-body-targets-stay-visible.md)
  carries. To confirm before landing: the ordinal must count the events of the one player the
  subject names, not every player's casts together.
- `eventCount` (`Amount.tally … .events`) compared with 2, for an existentially read player. No
  Template for "a player … two or more times" exists yet.

Pick the spelling whose denotation is "some player cast at least two spells last turn"; if neither
says it, extend the checker so one does.

## Witnesses (data/derived/cards.jsonl)

*Afflicted Deserter // Werewolf Ransacker* (transform). The front face's ability, "At the
beginning of each upkeep, if no spells were cast last turn, transform this creature.", checks
clean as `at_ (beginningOfEach .upkeep) (if_ := some (not (did .any (cast (a isSpell)) lastTurn)))
(transform thisCreature)`. The full card:

```lean
def afflictedDeserter : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Afflicted Deserter", cost := [generic 3, pip .red], types := [.creature],
      subtypes := [human, werewolf],
      text :=
        [ at_ (beginningOfEach .upkeep)
            (if_ := some (not (did .any (cast (a isSpell)) lastTurn)))
            (transform thisCreature) ],
      power := stat 3, toughness := stat 2 } }
  { characteristics :=
    { name := "Werewolf Ransacker", types := [.creature], subtypes := [werewolf],
      text :=
        [ whenever
            (.does .any (.enact (.action "transform") (.turnOver thisCreature)
              (some (named "Werewolf Ransacker")) none))
            (sequentially
              [ may (destroy (target isArtifact)),
                doIf (did (that artifact) (putInto .actor graveyard) thisWay)
                  (instruct thisCreature (dealDamage 3 (controllerOf (that artifact)))) ]),
          at_ (beginningOfEach .upkeep)
            (if_ := some (did (a isPlayer) (cast (counted (orMore 2) isSpell)) lastTurn))
            (transform thisCreature) ],
      power := stat 5, toughness := stat 4 } }
```

## Tests

- `okAfflictedDeserter`: the card above with the chosen spelling of the second upkeep ability, in
  `lean/Semantics/Cards/Faces.lean` or `Cards/Anaphora.lean`.
- A pin that the chosen spelling counts one player's casts.
