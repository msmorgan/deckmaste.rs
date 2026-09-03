---
needs: [workbench-turn-ref-read]
---
**Retype the turn-part possessor as `Maybe (Noun bs Player)` and delete the
fused pronoun enum `Owner`.** Cleanroom review 2026-09-03, F2. Rules-meaningful
printed headers are unspellable today because the fused word is the only
admitted spelling.

`Words.Owner` (`Words.idr:3829–3832`: `Yours | ThatPlayers | EachPlayers |
EachOpponents | EachYours | AnOpponents | ThatTurns | EachOthers`) is consumed
by `Triggers.HeaderPossessor` (`Triggers.idr:555–559`: `NoPossessor | ByWord
Owner | ByNoun n {PossessorNoun n}`), `Words.DurationEnd` (`Words.idr:3882`),
`Triggers.TriggerWindow`/`Timing` (`Triggers.idr:630–641`) and
`Effect.OnlyDuring` (`Effect.idr:439`), with `possessorB` (`Words.idr:3854`)
re-deriving the bindings a noun would have introduced. `PossessorNoun`
(`Triggers.idr:549–552`) admits exactly one noun, `AttachHost w PlayerW`.
Probes (each twin through `ByWord` typechecks):

```
BeginningOf Upkeep (ByNoun (the ChosenPlayer))          -- Black Vise, Energy Vortex
  Can't find an implementation for PossessorNoun (Described TheDet ChosenPlayer).
BeginningOf Upkeep (ByNoun (controllerOf (AttachHost Enchanted (TypeW Creature))))
  -- Apathy, Curse Artifact, Cursed Land, Erosion, Errant Minion
  Can't find an implementation for PossessorNoun (PossessorOf ControllerAx …).
BeginningOf Upkeep (ByNoun (a Opponent))                -- the compositional AnOpponents
  Can't find an implementation for PossessorNoun (Described (ADet Unmarked) Opponent).
```

## Fix

`HeaderPossessor bs = Maybe (Noun bs Player)`. `DurationEnd`,
`TriggerWindow`, `Timing.DuringPart` / `BeforePoint` and `OnlyDuring` take the
same noun. "That turn's" is a `Noun bs TurnRef` read through the `Pro` read
minted by `workbench-turn-ref-read`, replacing `TurnDeixis`
(`Triggers.idr:588`). `windowOk` / `pointWindowOk` (`Phrase.idr:3079–3095`)
and `durationPossessorOk` (`Words.idr:3866`) become `nounPlur` / `nounIsYou`
checks. Delete `Owner`, `possessorB`, `isTurnDeictic` and `PossessorNoun`.

This is ADR §4 as written: no fused pronoun forms; relational nouns compose
over any noun.

Bench migration: 82 sites spell an `Owner` word (59 through `ByWord`, the rest
in `DurationEnd` / `OnlyDuring` / `Timing`), plus 5 pins.

Size: L.

Done when: the build is 23/23 with 0 errors and 0 warnings; `Owner`,
`possessorB`, `isTurnDeictic` and `PossessorNoun` are gone; every possessor
slot takes a `Noun`; the three probe headers above (Black Vise, Apathy, "an
opponent's upkeep") are typechecking bench witnesses on their printed cards;
all 82 former `Owner` sites and the 5 pins are re-spelled and every one of
those pins still refutes for its named reason. Standard constraints apply.
