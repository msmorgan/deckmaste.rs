---
needs: []
---
# `Location.observedOnDeparture` keeps only the battlefield arm

Standard constraints apply. Decided by the owner in the 2026-10-25 triage follow-up (Q7); the
change was prototyped on a scratch copy and `lake build --wfail` was green. Paths are under
`lean/Semantics/` unless they start with `lean/` or `docs/`; line numbers are as of the tree at
the ticket's minting, so find each site by name.

## The change

`Location.observedOnDeparture` (`Abilities.lean:1568-1573`) classes a written origin as
departure-observed when it names the battlefield, the graveyard, or (to a hand or library) exile,
the stack or the command zone. Keep only the battlefield arm: a put or return Template that
writes a graveyard, exile, stack or command-zone origin is no longer classed departure-observed
by this function. The `destination` parameter becomes unused (`--wfail` rejects it), so drop it.

The battlefield arm is the leaves-the-battlefield ability [CR#603.6c]; the other arms were an
over-reading of the look-back zone changes [CR#603.10a] that the put and return Templates should
not decide for every printed origin.

### 1. Code

- `Abilities.lean:1568-1573`: replace the whole definition with
  ```lean
  def Location.observedOnDeparture (from_ : Location) : Bool :=
    match from_.positiveZones? with
    | some zones => zones.any fun zone => zone.sort == some .battlefield
    | none => false
  ```
- `Templates.lean:1087` (the `zoneChange` Template): `(if from_.observedOnDeparture destination
  then .departure else .arrival) riders` becomes `(if from_.observedOnDeparture then .departure
  else .arrival) riders`.
- `Proofs/HistoricalMove.lean:63` (`mixedOriginReadsThePermanentOnlyOnDeparture`):
  `Location.observedOnDeparture mixedOrigin (.zone .graveyard none) = true ∧` becomes
  `Location.observedOnDeparture mixedOrigin = true ∧`. Nothing else in that theorem changes.

### 2. Docs to rewrite

- `Abilities.lean:1556-1567` (the docstring of `Location.observedOnDeparture`): rewrite it to say
  that a move is observed on its departure when any positive origin it writes is the battlefield,
  since an ability that triggers when a permanent moves from the battlefield is a
  leaves-the-battlefield ability whatever else it watches [CR#603.6c]; that a move from anywhere
  or from a complement is observed on its arrival; and that the put and return Templates write
  this classification into the move and the checker reads a departure-observed move's battlefield
  patient as the permanent it was. Delete the sentences on graveyard, exile, stack and
  command-zone origins and on the destination.
- `Templates.lean:1076-1083` (the docstring of `zoneChange`): drop "or a graveyard (or, to a hand
  or library, from exile, the stack or the command zone)". It reads: a move from the battlefield,
  or from a list naming it, is observed on its departure; "from anywhere" is not a
  leaves-the-battlefield trigger [CR#603.6c].
- `CONTRACTS.md:1452-1458`: the paragraph that says the put and return Templates take the
  Observation Point from `Location.observedOnDeparture` and that it classes a written origin list
  as departure-observed when any of its origins is: restate it with the battlefield as the only
  departure-observed origin.
- These stay true and are not edited: `CONTRACTS.md:1072-1074` and
  `Proofs/DuringResolution.lean:15-17` (a spell put into a graveyard from the stack is observed
  on arrival). Read `Abilities.lean:1495`, `Check/Phrase.lean:1374` and `Check/Phrase.lean:3943`,
  which mention the function by name; they stay as they are unless one names a non-battlefield
  origin, in which case reword that clause only.

## Pins

Rewritten, not flipped to `.arrival`: they asserted the departure classification of graveyard
origins, which is no longer the claim. What they now assert is that the written origin and riders
stay on the move.

- `graveyardOriginsAreDepartureObserved` (`Proofs/CardFidelity.lean:158`) becomes
  `graveyardOriginsAreWrittenOnTheMove`, and `spelledGraveyardOriginsAreDepartureObserved`
  (`:172`) becomes `spelledGraveyardOriginsAreWrittenOnTheMove`. Both assert origin and riders
  through a private `written` projection that drops the Observation Point. Put this before the
  first theorem and replace both theorems exactly:
  ```lean
  /-- The written origin and riders of each zone change, without its Observation Point. -/
  private def written (ms : List (Location × ObservationPoint × List MoveRider)) :
      List (Location × List MoveRider) :=
    ms.map fun (origin, _, riders) => (origin, riders)

  /-- A move from a graveyard keeps its printed origin, "from your graveyard", on the move; the
  target keeps its own `in_ <graveyard>` for target legality [CR#115.1] (owner ruling 2026-10-24),
  and the checker accepts the pair. -/
  theorem graveyardOriginsAreWrittenOnTheMove :
      written (abilityMoves Cards.hammerOfBogardan) = [(yourGraveyard, [])] ∧
        written (moves 64 Cards.stormOfSouls) = [(yourGraveyard, []), (.anyLocation, [])] ∧
        (written (moves 64 Cards.bondOfRevival)).head? = some (yourGraveyard, []) ∧
        (written (moves 64 Cards.everAfter)).head? = some (yourGraveyard, []) ∧
        (written (moves 64 Cards.riseFromTheGrave)).head? = some (graveyard, [(.under .you)]) ∧
        written (moves 64 Cards.hymnOfRebirth) = [(graveyard, [(.under .you)])] ∧
        (written (moves 64 Cards.theLastRoninII)).getLast? = some (yourGraveyard, []) ∧
        Action.check (DRS.ofList []) Cards.bondOfRevival = [] ∧
        Action.check (DRS.ofList []) Cards.hymnOfRebirth = [] := by
    refine ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, by decide, by decide⟩

  /-- The same on the spelled cards: Eerie Ultimatum, Rogues' Gallery, Blood Spatter Analysis's
  reflexive return, and Midnight Scavengers' front face, each "from your graveyard". -/
  theorem spelledGraveyardOriginsAreWrittenOnTheMove :
      written ((faceText Cards.eerieUltimatum.card).flatMap abilityMoves) = [(yourGraveyard, [])] ∧
        written ((faceText Cards.roguesGallery.card).flatMap abilityMoves) = [(yourGraveyard, [])] ∧
        (written ((faceText Cards.bloodSpatterAnalysis.card).flatMap abilityMoves)).getLast? =
          some (yourGraveyard, []) ∧
        written (match Cards.midnightScavengers.card with
          | .transforming front _ => front.characteristics.text.flatMap abilityMoves
          | _ => []) = [(yourGraveyard, [])] := ⟨rfl, rfl, rfl, rfl⟩
  ```
- `graveyardToHandObservedOnDeparture` (`Proofs/DefinitionsAndTemplates.lean:138`) becomes
  `graveyardToHandVerbsExpandToOneMove`, a claim about the two verbs expanding to one move, with
  no Observation Point:
  ```lean
  /-- "Return <subject> from your graveyard to your hand" and "put <subject> into your hand from
  your graveyard" name no deed and expand to the same move, its written origin kept [CR#400.7]. -/
  theorem graveyardToHandVerbsExpandToOneMove :
      returnToHand (target isCreature) yourGraveyard =
        putInto (target isCreature) hand (some yourGraveyard) := rfl
  ```
- No pin is added and none is retired.

## Must stay green, unchanged

`mixedOriginWithTheBattlefieldIsObservedOnDeparture`, `mixedOriginReadsThePermanentOnlyOnDeparture`
(only its call is respelled above), `bareReturnToBattlefieldIsObservedOnArrival`, the look-back
suite (Saw in Half, Kaervek's Purge, Tuktuk Scrapper, `okThatCreatureDiesThisWay`, and the rest
of `Cards/Anaphora.lean` and `Proofs/Lookback.lean`), `ObservationPoint`, the `field` of
`Action.move`, and `Location.readsDepartedPermanent`.

## Acceptance

- `./lean/scripts/build` (`--wfail`) is green; `grep -rn 'observedOnDeparture' lean/Semantics`
  shows no call with two arguments, and `grep -rn 'DepartureObserved' lean/Semantics` is empty.
- `cargo xtask lean check`.
- `cargo xtask cite check --list-noncompliant` is empty, `cargo xtask cite check` reports 0
  stale, and `jj --no-pager diff --git | cargo xtask cite audit --diff` is read rule by rule.

## Follow-up check

After the change, re-read `lean/CONTRACTS.md:1072-1074` ("Battlefield and graveyard departure
patterns observe on departure"). That line is about trigger *patterns*, not move origins, and is
expected to stay true; if the change makes any graveyard-departure pattern stop observing on
departure, reword the line to state what is now true and say so in the report.
