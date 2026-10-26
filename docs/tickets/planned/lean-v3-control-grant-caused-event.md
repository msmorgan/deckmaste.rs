---
needs: []
---
# A control grant records its caused event, so "if you gain control of a creature this way" reads it

Standard constraints apply. Decided 2026-10-25: the imperative "gain control of" keeps its split.
The resolving imperative stays `establish (controlGrant …)`, because it creates a layer-2
continuous effect [CR#611.2a,613.1b]; the event and deontic form stays `Action.changeControl`
(`gainControlOf`). The fix is that the grant records the same caused event the pattern form names.
Verified by building both changes in a scratch copy of the tree at `pmvrnuxw`: the whole library
and every pin suite pass with `./lean/scripts/build` and the two witness cards below check `[]`.
Paths are under `lean/Semantics/`; line numbers are as of that change.

## The defect

`.controlGrant` (a `StaticSpec`, `Abilities.lean:1010`) is stepped at `Check/Phrase.lean:4567-4570`:

```lean
        | .controlGrant player subj =>
          let (drs1, _, e1) := ops.np drs .subj true player
          let (drs2, ids, e2) := ops.np drs1 .obj true subj
          (applyInPlaceStamp drs2 ids (some (.core .gainControl)), e1 ++ e2)
```

It stamps its object with the gain-control deed but pushes no caused event. Every resolving or paid
action that a lookback can name records one through `recordCaused` at the head of the `inst` step
(`Check/Phrase.lean:3637`, `DRS.withCaused`, `:1297`). `establish` is not an event head
(`Action.eventHeadOk`), so the grant never reaches it. A later "this way" lookback reads
`DRS.causedEvents` [CR#608.2c], finds none, and is refused `thisWayUngrounded`. Probed 2026-10-25
on the tree at `pmvrnuxw`, both refused `[.thisWayUngrounded]`:

- *Coveted Falcon* ({1}{U}{U}, Artifact Creature — Bird 1/4). Oracle text from
  `data/derived/cards.jsonl`: "Flying / Whenever this creature attacks, gain control of target
  permanent you own but don't control. / Disguise {1}{U} / When this creature is turned face up,
  target opponent gains control of any number of target permanents you control. Draw a card for
  each one they gained control of this way."
- *Midnight Crusader Shuttle* ({4}, Artifact — Vehicle 3/4). Oracle text: "Midnight Entity —
  Whenever this Vehicle attacks, defending player faces a villainous choice — That player
  sacrifices a creature of their choice, or you gain control of a creature of your choice that
  player controls until end of turn. If you gain control of a creature this way, tap it, and it's
  attacking that player. / Crew 2"

The Shuttle needs a second change. Its grant sits in an arm of `faceAVillainousChoice`, which is
`chooseOne`, which is `.chooseModes`, and `stepModeListWith` (`Check/Phrase.lean:2137-2145`) returns
`{ drs with nextId := … }` for every arm, dropping the events the arms recorded. The other
construct that steps alternative arms keeps them (`Check/Phrase.lean:4263-4266`: "The body and its
arms wrote their events whether or not the player chooses to act, so a later 'this way' reads
them back"). Modes must too.

## Change 1: the grant pushes the event of `Action.changeControl`

Replace the `.controlGrant` arm (`Check/Phrase.lean:4567-4570`) with exactly:

```lean
        | .controlGrant player subj =>
          let (drs1, pids, e1) := ops.np drs .subj true player
          let (drs2, ids, e2) := ops.np drs1 .obj true subj
          let kindOf (np : NounPhrase) (rids : List ReferentId) (dflt : Kind) : Kind :=
            (rids.head?.bind drs2.findById?).map (·.kind) |>.getD (np.kindIn drs2 dflt)
          let stamped := applyInPlaceStamp drs2 ids (some (.core .gainControl))
          let gained := (Action.changeControl subj none).causedEvent
            (some (kindOf player pids .player)) (some (kindOf subj ids .object))
          (if drs.actionPattern then stamped
            else { stamped with causedEvents := gained :: stamped.causedEvents }, e1 ++ e2)
```

This is the event the pattern form names, not a new shape: `Action.causedEvent` (`:1241`) gives
`deed := some (.core .gainControl)` (from `Action.namedDeed`, `:195`), `zoneChange := none`,
`verbs := [.controlChanged]` (`Action.eventVerbs`, `:1194`), the performer's and the patient's kinds
(`permitted := false`). The performer is the grantee (`player`, a player); the patient is what is
taken (`subj`). The guard `drs.actionPattern` is the one `DRS.withCaused` uses (`:1299`): a pattern
records nothing. The spec step takes no phase argument, so unlike `withCaused` it cannot also test
`phase`; this is exact, because `establish` is refused in every pattern position by the event
gate. The sibling to copy is `DRS.withCaused` (`:1297-1302`); do not call it, since `.establish`
fails its `eventHeadOk` test.

## Change 2: mode arms publish their caused events

In `stepModeListWith` (`Check/Phrase.lean:2145`) replace
`({ drs with nextId := max out.nextId afterBody.nextId }, errors ++ costErrors ++ bodyErrors)) (drs, [])`
with:

```lean
    ({ drs with
        nextId := max out.nextId afterBody.nextId
        causedEvents := newlyCaused branch afterBody ++ out.causedEvents },
      errors ++ costErrors ++ bodyErrors)) (drs, [])
```

(`newlyCaused` is at `:1311`, above this definition.) Each arm's events, a cost's included, join the
list whether or not the player chooses the arm, as `:4263-4266` does for the two arms of a
`doIf`/`otherwise`. Nothing else in the fold changes: the arm still starts from `drs`, so one arm
does not read another's events as its own.

## Docs

- `lean/CONTRACTS.md:1508-1509`: after "The pattern stamps its patient with `.core .gainControl`,
  as the grant does, so one Participial Description reads either." add: "The grant records the
  same event the pattern names (`causedEvents`: the gain-control deed, class `controlChanged`, the
  grantee's kind as its Actor and the kind of what it takes), so 'If you gain control of a creature
  this way' reads it (`controlGrantRecordsItsCausedEvent`)."
- `lean/CONTRACTS.md:297-298`: after "Each resolving or paid action a lookback can name
  (`Action.eventHeadOk`) records its event (`DRS.withCaused`, `DRS.causedEvents`)" add "; a control
  grant, which is an `establish` and no event head, records its own (`controlGrantRecordsItsCausedEvent`);
  the arms of a choice publish theirs (`aModeArmPublishesItsCausedEvent`)".
- Read [CR#608.2c], [CR#611.2a] and [CR#613.1b] in `data/rules/cr.txt` before citing them.

## Pins

Added (names are exact):

1. `controlGrantRecordsItsCausedEvent`, in `Proofs/ControllerSacrifice.lean` beside
   `controlGrantRecordsItsPatientIdentity` (`:90`), the pin that the caused event carries the
   gain-control deed:

   ```lean
   theorem controlGrantRecordsItsCausedEvent :
       (stepAction 64 .postEvent .obj (DRS.ofList [])
           (gainControl (target isCreature) (some untilEndOfTurn))).1.causedEvents =
         [{ deed := some (.core .gainControl), zoneChange := none, verbs := [.controlChanged],
            performer := some .player, patient := some .object }] := by decide
   ```
2. `thisWayReadsOnlyTheDeedWritten`, same file: a grant does not ground another deed's "this way".

   ```lean
   theorem thisWayReadsOnlyTheDeedWritten :
       Action.check (DRS.ofList [])
         (sequentially
           [ gainControl (target isCreature) untilEndOfTurn,
             doIf (did you (draw anyAmount) thisWay) (tap it) ]) = [.thisWayUngrounded] := by decide
   ```
3. `aModeArmPublishesItsCausedEvent`, same file:

   ```lean
   theorem aModeArmPublishesItsCausedEvent :
       Action.check (DRS.ofList [])
         (sequentially
           [ chooseOne (sacrifice (a isCreature)) (gainControl (target isCreature) untilEndOfTurn),
             doIf (did you (gainControlOf (a isCreature)) thisWay) (tap it) ]) = [] := by decide
   ```
4. `covetedFalcon`, in `Cards/Anaphora.lean` before `end Semantics.Cards`, its "this way" checking
   `[]` (the `spelled` elaborator fails the build otherwise):

   ```lean
   /-- Coveted Falcon: "Draw a card for each one they gained control of this way" reads the grant
   the opponent's "gains control of any number of target permanents you control" recorded
   [CR#608.2c,613.1b]. -/
   def covetedFalcon : Spelled := spelled <| singleFacedCard
     { name := "Coveted Falcon", cost := [generic 1, pip .blue, pip .blue],
       types := [.artifact, .creature], subtypes := [bird],
       text :=
         [ flying,
           whenever (does thisCreature attack)
             (gainControl (target (and [isPermanent, youOwn, not (youControl)]))),
           disguise [generic 1, pip .blue],
           when (does any (turnFaceUp thisCreature))
             (sequentially
               [ instruct (target isOpponent)
                   (gainControl (targets anyNumber (and [isPermanent, youControl]))),
                 draw (forEach (eventTally they (gainControlOf (those permanent)) thisWay .events)
                   isPermanent) ]) ],
       power := stat 1, toughness := stat 4 }
   ```
5. `midnightCrusaderShuttle`, same place, its "this way" checking `[]`:

   ```lean
   /-- Midnight Crusader Shuttle: "If you gain control of a creature this way" reads the grant in
   the second arm of the villainous choice [CR#608.2c,613.1b]. "Midnight Entity" is a flavor word
   [CR#207.2d]. -/
   def midnightCrusaderShuttle : Spelled := spelled <| singleFacedCard
     { name := "Midnight Crusader Shuttle", cost := [generic 4], types := [.artifact],
       subtypes := [vehicle],
       text :=
         [ flavorWord "Midnight Entity"
             (whenever (does thisVehicle attack)
               (sequentially
                 [ faceAVillainousChoice
                     (instruct defendingPlayer (sacrifice (a isCreature)))
                     (gainControl (target (and [isCreature, controlledBy defendingPlayer]))
                       untilEndOfTurn),
                   doIf (did you (gainControlOf (a isCreature)) thisWay)
                     (sequentially [tap it, becomeAttacking it (some defendingPlayer)]) ])),
           crew 2 ],
       power := stat 3, toughness := stat 4 }
   ```

   Read [CR#207.2d] before citing it. Both card spellings were compiled in the scratch copy with
   both changes (clean) and refuse `[.thisWayUngrounded]` on the unchanged tree. If a name here
   already exists in `Cards/`, stop: that is a collision this ticket did not foresee.

Existing pins stay: `controlGrantRecordsItsPatientIdentity` (`Proofs/ControllerSacrifice.lean:90`),
`controlChangeStampsItsPatientAsTheGrantDoes` (`Proofs/EventPatterns.lean:287`),
`controlChangeIsOnlyAPattern` (`:271`). No pin is rewritten or retired.

## What must not change

- `Action.changeControl`, its refusal outside patterns (`controlChangePattern`), `gainControlOf`,
  `gainControl` and the `controlGrant` Template. The imperative stays `establish (controlGrant …)`.
- The patient stamp: `applyInPlaceStamp drs2 ids (some (.core .gainControl))`.
- `DRS.withCaused` and `recordCaused`.
- No arm of a mode list reads another arm's events: each arm still starts from `drs`.

## Commit plan

Run the per-commit gate before each commit.

1. `lean: a control grant records its caused event`: Change 1, pins 1, 2 and 4, the first CONTRACTS
   edit (and the `controlGrantRecordsItsCausedEvent` clause of the second).
2. `lean: the arms of a choice publish their caused events`: Change 2, pins 3 and 5, and the rest
   of the second CONTRACTS edit.

## Acceptance

- `./lean/scripts/build` (`--wfail`) with zero warnings and no `sorry`, before each commit.
- `cargo xtask lean check`.
- `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check` 0 stale,
  `cargo xtask cite bless` for newly cited rules, and `jj --no-pager diff --git | cargo xtask cite
  audit --diff` read rule by rule against its claim.
