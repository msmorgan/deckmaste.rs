import Semantics.Definitions
import Semantics.Check.Phrase

namespace Semantics.Proofs.Lookback

open Semantics Semantics.Templates Semantics.Definitions

/-- "this turn", as a lookback reads it (`Templates.soFarThisTurn`). -/
def soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this game" (`Templates.thisGame`). -/
def thisGame : Duration := .between none (some .now)

private def relativeCheck (kind : Kind) (clause : DidClause) : List Refusal :=
  Predicate.checkAt kind (DRS.ofList []) (.did clause)

private def withinThisTurn : LookbackSpan := .duration soFarThisTurn none

/-- `relativeCheck` after an earlier action of the same ability, whose event a "this way" reads
back. -/
private def relativeCheckAfter (earlier : Action) (kind : Kind) (clause : DidClause) :
    List Refusal :=
  Predicate.checkAt kind (nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) earlier).1)
    (.did clause)

theorem scopedPlayerDraw :
    relativeCheck .player { subj := .actor .which
                            action := .draw (.exactlyOf .anyAmount)
                            within := some withinThisTurn } = [] := by decide

theorem playerGapCannotStandForObject :
    relativeCheck .object { subj := .actor .which
                            action := .draw (.exactlyOf .anyAmount)
                            within := some withinThisTurn } =
      [.kindMismatch .player .object] := by decide

theorem gapRequiresLookback :
    GameEvent.check (DRS.ofList []) (.does (.actor .which) (.draw (.exactlyOf .anyAmount))) =
      [.anaphor (.word .permanent) .one 0] := by decide

theorem scopedCreatureDeath :
    relativeCheck .object
      { subj := .any
        action := .move (.headed (.type .creature) .which)
          (.zone .battlefield none) (some (.zone .graveyard none)) .departure []
        within := some withinThisTurn } = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    relativeCheck .player
      { subj := .actor .which
        action := .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did { subj := .any
                     action := .move (.headed (.type .creature) .which)
                       (.zone .battlefield none) (some (.zone .graveyard none))
                       .departure []
                     within := some withinThisTurn }]))
          .itsOwnCost none none none
        within := some withinThisTurn } = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    relativeCheck .player
      { subj := .actor .which
        action := .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did { subj := .actor .which
                     action := .draw (.exactlyOf .anyAmount)
                     within := some withinThisTurn }]))
          .itsOwnCost none none none
        within := some withinThisTurn } = [.kindMismatch .player .object] := by decide

theorem lookbackRequiresBoundParticipant :
    relativeCheck .player { subj := .actor .you
                            action := .draw (.exactlyOf .anyAmount)
                            within := some withinThisTurn } =
      [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    relativeCheck .object
      { subj := .actor .you
        action := .draw (.exactlyOf .anyAmount)
        within := some (.duration soFarThisTurn
          (some (.onlyIf (.exists_ (.described (.a .unmarked)
            (.and [.hasType .creature,
              .did { subj := .any
                     action := .move .which (.zone .battlefield none)
                       (some (.zone .graveyard none)) .departure []
                     within := some withinThisTurn }])))))) } =
      [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    relativeCheck .object
      { subj := .actor (.headed (.type .creature) .which)
        action := .dealDamage .anyAmount (.headed (.type .creature) .which)
        within := some withinThisTurn } = [] := by decide

private def damageHistory : Predicate :=
  .did { subj := .actor (.headed (.type .creature) .which)
         action := .dealDamage .anyAmount (.described (.a .unmarked) (.isOpponentOf .you))
         within := some withinThisTurn }

theorem historyDoesNotIntroduceDamageOutcome :
    countOutcomes .damageDealt (stepPredicate 64 (DRS.ofList []) damageHistory).1 = 0 := by decide

theorem historyDoesNotExportNamedParticipant :
    NounPhrase.checkAt (some .player) (stepPredicate 64 (DRS.ofList []) damageHistory).1
      (.pro (.word .player) .one) = [.anaphor (.word .player) .one 0] := by decide

theorem historyNamedParticipantAccessibleLocally :
    relativeCheck .object
      { subj := .actor .which
        action := .dealDamage (.statOf (.playerStat .lifeTotal) (.described (.a .unmarked) (.isOpponentOf .you)))
          (.pro (.word .player) .one)
        within := some withinThisTurn } = [] := by decide

theorem castCanCarryExcludedOrigins :
    relativeCheck .player
      { subj := .actor .which
        action := .procedure .cast (.described (.a .unmarked) (.and [.isCard, .in_ (.zone .stack none)]))
          .itsOwnCost none (some (.not (.zone .hand (some .you)))) none
        within := some withinThisTurn } = [] := by decide

theorem namedActionCanCarryItsLocus :
    relativeCheckAfter (search yourLibrary) .player
      { subj := .actor .which
        action := .enact (.action "search") (.search (.zone .library (some .you)) .any) none
          (some (.zone .library (some .you)))
        within := some .thisWay } = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    relativeCheckAfter shuffle .player
      { subj := .actor .which
        action := .enact (.action "shuffle") (.shuffle (.zone .library (some .you)))
          none (some (.zone .graveyard (some .you)))
        within := some .thisWay } = [.lookbackLocus] := by decide

theorem nestedLookbackPreservesOuterBinder (drs : DRS) (clause : DidClause) :
    (stepPredicate 64 drs (.did clause)).1.relativeBinder = drs.relativeBinder ∧
      (stepPredicate 64 drs (.did clause)).1.relativeUses = drs.relativeUses := by
  constructor <;> rfl

theorem unspecifiedSpellDoesNotIntroduceAReference :
    (stepGameEvent 64 (DRS.ofList [])
      (.does .any (.procedure .cast .any .itsOwnCost none none none))).1.referents = [] := by rfl

theorem designatedSpellIsWellKinded :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.headed .spell (.designated (.named "commander") .you)) = [] := by decide

theorem designatedSpellKeepsSelectedIdentity :
    let result := stepNounPhrase 64 (DRS.ofList []) .other false
      (.headed .spell (.designated (.named "commander") .you))
    result.2.1 = [1] ∧ (result.1.findById? 1).bind (·.zone) = some .stack := by decide

theorem designatedSpellCanBeMatched :
    relativeCheck .player
      { subj := .actor .which
        action := .procedure .cast (.headed .spell (.designated (.named "commander") .you))
          .itsOwnCost none none none
        within := some (.duration thisGame none) } = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (per : Option Period) (cause : Causing) (fuel : Nat) :
    (GameEvent.nthOccurrence ordinal per event).facts (fuel + 1) = event.facts fuel ∧
      (GameEvent.causes cause event).facts (fuel + 1) = event.facts fuel := ⟨rfl, rfl⟩

/-- "the first spell you've cast this game": a lookback's ordinal is an event ordinal, which
counts within a period an event ordinal names, and which singles out the one event the
description reads, so a `the`-headed description with it uniquifies. -/
theorem lookbackOrdinalIsAnEventOrdinal :
    relativeCheck .object
        { subj := .actor .you, action := cast .which, within := some (.duration thisGame none),
          ordinal := some (.nth 1, none) } = [] ∧
      relativeCheck .object
        { subj := .actor .you, action := cast .which, within := some (.duration thisGame none),
          ordinal := some (.nth 1, some .thisObject) } = [.eventOrdinalPeriod .thisObject] ∧
      NounPhrase.check (DRS.ofList [])
        (the (and [isSpell, .did { subj := .actor .you, action := cast .which,
                                    within := some (.duration thisGame none),
                                    ordinal := some (.nth 1, none) }])) = [] ∧
      NounPhrase.check (DRS.ofList [])
        (the (and [isSpell, .did { subj := .actor .you, action := cast .which,
                                    within := some (.duration thisGame none) }])) =
        [.uniquifying] := by decide

/-! ## Unwritten spans

A lookback with no span written is an attributive participle, read only by a Participial
Description ("the destroyed creature"). A condition and a tally always write their span, and so
does a relative clause under any other determiner. -/

theorem unanchoredConditionRefused :
    Condition.check (DRS.ofList []) (.did { subj := .any
                                            action := destroy (a isCreature)
                                            within := none }) =
      [.unanchoredLookback] := by decide

theorem unanchoredTallyRefused :
    Amount.check (DRS.ofList []) (.tally { subj := .any
                                           action := destroy (a isCreature)
                                           within := none } .events) =
      [.unanchoredLookback] := by decide

/-- "target creature destroyed" names no span and no destruction to read back. -/
theorem unanchoredRelativeClauseRefused :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (and [isCreature, was (destroy which)])) = [.unanchoredLookback] := by decide

/-- "Destroy target creature. Its controller gains life equal to the destroyed creature's
power.": "the destroyed creature" reads back what the destruction acted on. -/
theorem participialDescriptionReadsBackItsDeed :
    Action.check (DRS.ofList [])
      (sequentially
        [ destroy (target isCreature),
          instruct (controllerOf it)
            (gainLife (powerOf (the (and [was (destroy which), isCreature])))) ]) = [] := by
  decide

/-- With "this way" written the relative clause still anchors the description: "the creature
destroyed this way". -/
theorem participialThisWayReadsBackItsDeed :
    Action.check (DRS.ofList [])
      (sequentially
        [ destroy (target isCreature),
          gainLife (powerOf (the (and [isCreature, was (destroy which) thisWay]))) ]) = [] := by
  decide

/-- Nothing was destroyed: the description has nothing to read back. -/
theorem participialDescriptionWithoutItsDeedRefused :
    Action.check (DRS.ofList [])
      (sequentially
        [ tap (target isCreature),
          gainLife (powerOf (the (and [was (destroy which), isCreature]))) ]) =
      [.participialDescription (.deed (.action "destroy")) 0] := by
  decide

/-- A clause whose `which` is the deed's subject anchors no description: "each player who
sacrificed a permanent this way" describes players. -/
theorem subjectRelativeClauseAnchorsNoDescription :
    (Predicate.participialAnchor?
      (and [isPlayer, did which (sacrifice (a isPermanent)) thisWay])).isNone = true := by
  decide

/-! ## Participial groups

A group determiner reads every object its deed acted on, together; a Card Type in it picks
members of that group, and asks nothing of the group itself. -/

/-- Culmination of Studies: "Exile the top X cards of your library. For each land card exiled
this way, create a Treasure token." -/
theorem okEachLandCardExiledThisWay :
    Action.check (DRS.ofList [letterB .x])
      (sequentially
        [ exile (topSlice (letter .x)),
          doForEach (each (and [isLand, isCard, was (exile which) thisWay]))
            (createToken treasureToken) ]) = [] := by
  decide

/-- Rally the Horde: "Exile the top card of your library. Exile the top card of your library.
Exile the top card of your library. … Create a 1/1 red Warrior creature token for each nonland
card exiled this way.": the group is every card the three exiles acted on. -/
theorem okEachCardOfSeveralExiles :
    Action.check (DRS.ofList [])
      (sequentially
        [ exile (topSlice 1), exile (topSlice 1), exile (topSlice 1),
          doForEach (each (and [non isLand, isCard, was (exile which) thisWay]))
            (createToken (creatureToken 1 1 [.red] [warrior])) ]) = [] := by
  decide

/-- Sinister Concierge: "When this creature dies, you may exile it and put three time counters on
it. If you do, exile up to one target creature and put three time counters on it. Each card
exiled this way that doesn't have suspend gains suspend.": both exiles sit inside the optional
body or its "if you do" arm, and "this way" ranges over the events that happened, so the
description reads both exiled cards (owner ruling 2026-10-22). A singular "the" finds the two
and is refused. (Granting a bare "suspend" has no Definition, so the pin reads the group with
another body.) -/
private def sinisterConcierge (rest : Action) : Ability :=
  when (does thisCreature die)
    (sequentially
      [ may (sequentially [exile it, putCounters 3 timeCounter it])
          (some (sequentially
            [exile (targets (upTo 1) isCreature), putCounters 3 timeCounter it])),
        rest ])

theorem eachCardExiledThisWayReadsEveryExile :
    Ability.check (DRS.ofList [])
      (sinisterConcierge (doForEach (each (and [isCard, was (exile which) thisWay]))
        (putCounters 1 timeCounter it))) = [] ∧
    Ability.check (DRS.ofList [])
      (sinisterConcierge (tap (the (and [isCard, was (exile which) thisWay])))) =
      [.participialDescription (.deed (.action "exile")) 2] := by
  decide

/-- What an optional body acted on escapes it only to a Participial Description: after "you may
exile a creature you control", "it" has no antecedent, while "the card exiled this way" reads the
exiled creature (owner ruling 2026-10-22). -/
theorem optionalBodyObjectEscapesOnlyToParticipialReads :
    Action.check (DRS.ofList [])
      (sequentially [may (exile (a (and [isCreature, youControl]))), tap it]) =
      [.anaphor .bare .one 0] ∧
    Action.check (DRS.ofList [])
      (sequentially [ may (exile (a (and [isCreature, youControl]))),
        tap (the (and [isCard, was (exile which) thisWay])) ]) = [] := by
  decide

/-- One object is one candidate. When the optional body and its "if you don't" arm both exile
the same earlier object, a singular "the card exiled this way" reads that one card; so does it
when an object already exiled is exiled again in an optional body. -/
theorem objectStampedOnTwoPathsIsReadOnce :
    Action.check (DRS.ofList [])
      (sequentially
        [ destroy (target isCreature), may (exile it) none (some (exile it)),
          tap (the (and [isCard, was (exile which) thisWay])) ]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [ exile (target (cardIn graveyard)), may (exile it),
          tap (the (and [isCard, was (exile which) thisWay])) ]) = [] := by
  decide

/-- "This way" reads what the ability's costs did as well as its effect: "Sacrifice a creature:
You gain life equal to the power of the creature sacrificed this way." -/
theorem thisWayReadsWhatItsCostDid :
    Ability.check (DRS.ofList [])
      (activated (sacrifice (a isCreature))
        (gainLife (powerOf (the (and [isCreature, was (sacrifice which) thisWay]))))) = [] := by
  decide

/-- The agent of the deed must be one a stamp can stand for: "the card an opponent exiled" names
an agent the checker cannot read back, so it is refused. "The card you exiled" is read. -/
theorem participialAgentRead :
    Action.check (DRS.ofList [])
      (sequentially
        [ exile (target (and [isCard, in_ graveyard])),
          putOntoBattlefield (the (and [isCard, did you (exile which)])) ]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [ exile (target (and [isCard, in_ graveyard])),
          putOntoBattlefield (the (and [isCard, did anOpponent (exile which)])) ]) =
      [.participialAgent] := by
  decide

/-- Gap, recorded in the ticket: a stamp records no agent, so "the card you exiled" reads a card
exiled by whoever this ability's text had exile it, here an opponent. -/
theorem gapYouReadsAnyPerformer :
    Action.check (DRS.ofList [])
      (sequentially
        [ instruct (target isOpponent) (exile (target (and [isCard, in_ graveyard]))),
          putOntoBattlefield (the (and [isCard, did you (exile which)])) ]) = [] := by
  decide

/-- Cascade's run and the card that stopped it are both exiled: "all cards exiled this way" reads
the run once, the stopping card as one of its members [CR#702.85a]. -/
theorem runReadOnceWithItsStoppingCard :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (exile (.run .top (and [non isLand]) (libraryOf .you)))).1
    let found := participialCandidates (.deed (.action "exile")) none (some .card) true after
    found.length = 2 ∧
      (match participialRead none found with
       | .ok read => read.map (·.det)
       | .error _ => []) = [.the] := by
  decide

/-! ## Zone Changes

"Put" and "return" name no deed. A Participial Description whose clause is a move reads the move
itself: the destination it writes, and the origin, if one is written [CR#400.7]. -/

/-- Cloudshift: "Exile target creature you control, then return that card to the battlefield
under your control." -/
private def cloudshiftThen (readBack : Action) : Action :=
  sequentially
    [ exile (target (and [isCreature, youControl])),
      returnToBattlefield (that card) [(under you)],
      readBack ]

/-- After Cloudshift's exile and return, "the one returned this way" and "the one put onto the
battlefield this way" both read the returned card: the two verbs write one move. -/
theorem returnedAndPutReadTheReturnedCard :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (sequentially
        [ exile (target (and [isCreature, youControl])),
          returnToBattlefield (that card) [(under you)] ])).1
    (returnToBattlefield which).participialAnchor? = some (.zoneChange none .battlefield) ∧
    (putOntoBattlefield which).participialAnchor? = some (.zoneChange none .battlefield) ∧
    (participialCandidates (.zoneChange none .battlefield) (some .one) none true after).map
      (·.zone) = [some .battlefield] ∧
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (was (returnToBattlefield which) thisWay)))) = [] ∧
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (was (putOntoBattlefield which) thisWay)))) = [] := by
  decide

/-- A Card Type reads what a move put onto the battlefield as the permanent it is now
[CR#608.2h]: after Cloudshift, "the creature returned this way" and "the creature put onto the
battlefield this way" read the returned creature, as "the permanent returned this way" does. -/
theorem returnedCreatureReadsThePermanentItIs :
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (and [isCreature, was (returnToBattlefield which) thisWay])))) = []
      ∧
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (and [isCreature, was (putOntoBattlefield which) thisWay])))) = []
      ∧
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (and [isPermanent, was (returnToBattlefield which) thisWay])))) =
      [] := by
  decide

/-- An object a move took off the battlefield is read as the permanent it was [CR#608.2h], and
one that never was a permanent is not one: Terastodon's "each permanent put into a graveyard this
way" reads a destroyed land, and nothing after a discard. -/
theorem permanentPutIntoAGraveyardReadsItsOrigin :
    Action.check (DRS.ofList [])
      (sequentially
        [ destroy (target isLand),
          tap (each (and [isPermanent, was (putInto which graveyard) thisWay])) ]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [ discard (a (and [isCard, in_ yourHand])),
          tap (each (and [isPermanent, was (putInto which graveyard) thisWay])) ]) =
      [.participialDescription (.zoneChange none .graveyard) 0] := by
  decide

/-- A written origin narrows the read: Cloudshift returned its card from exile, so "the one
returned from a graveyard this way" reads nothing. -/
theorem zoneChangeReadsItsWrittenOrigin :
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (was (returnToBattlefield which (from_ := some exileZone))
        thisWay)))) = [] ∧
    Action.check (DRS.ofList [])
      (cloudshiftThen (tap (the (was (returnToBattlefield which (from_ := some graveyard))
        thisWay)))) = [.participialDescription (.zoneChange (some .graveyard) .battlefield) 0] := by
  decide

/-! ## Grounding "this way"

A `thisWay` lookback reads back an event the earlier text of its own spell or ability caused
[CR#608.2c], by its Deed, Zone Change or verb, done by an Actor its subject can name; `triggering`
reads the event its ability triggers on [CR#603.2]. -/

/-- "Draw a card. If you search your library this way, shuffle." and "Draw a card. If you
sacrificed a permanent this way, draw a card.": nothing earlier searched or sacrificed. -/
theorem thisWayNamesNoEarlierEvent :
    Action.check (DRS.ofList [])
      (sequentially [draw 1, doIf (did you (search yourLibrary) thisWay) shuffle]) =
      [.thisWayUngrounded] ∧
    Action.check (DRS.ofList [])
      (sequentially [draw 1, doIf (did you (sacrifice (a isPermanent)) thisWay) (draw 1)]) =
      [.thisWayUngrounded] := by decide

/-- "Search your library for a card. If you search your library this way, shuffle." -/
theorem searchedThisWayReadsTheSearch :
    Action.check (DRS.ofList [])
      (sequentially
        [search yourLibrary (a isCard), doIf (did you (search yourLibrary) thisWay) shuffle]) =
      [] := by decide

/-- "Sacrifice a creature. If you sacrificed a permanent this way, draw a card." reads the
sacrifice [CR#701.21a]; after "Destroy target creature." it reads none. -/
theorem sacrificedThisWayReadsTheSacrifice :
    Action.check (DRS.ofList [])
      (sequentially
        [sacrifice (a isCreature), doIf (did you (sacrifice (a isPermanent)) thisWay) (draw 1)]) =
      [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [destroy (target isCreature),
          doIf (did you (sacrifice (a isPermanent)) thisWay) (draw 1)]) =
      [.thisWayUngrounded] := by decide

/-- "Discard a card. If you discarded a card this way, draw a card." reads the discard; after
"Mill two cards." it reads none, since milling is no discarding. -/
theorem discardedThisWayReadsTheDiscard :
    Action.check (DRS.ofList [])
      (sequentially
        [discard (a (cardIn hand)), doIf (did you (discard (a isCard)) thisWay) (draw 1)]) =
      [] ∧
    Action.check (DRS.ofList [])
      (sequentially [mill 2, doIf (did you (discard (a isCard)) thisWay) (draw 1)]) =
      [.thisWayUngrounded] := by decide

/-- "Exile target creature. If a creature was exiled this way, draw a card." reads the exile
[CR#701.13a]; after "Destroy target creature." it reads none. -/
theorem exiledThisWayReadsTheExile :
    Action.check (DRS.ofList [])
      (sequentially
        [exile (target isCreature), doIf (was (exile (a isCreature)) thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [destroy (target isCreature), doIf (was (exile (a isCreature)) thisWay) (draw 1)]) =
      [.thisWayUngrounded] := by decide

/-- The event is read back with its Actor: "Target player sacrifices a creature. If that player
sacrificed a permanent this way, …" reads the handed-off sacrifice; "If this creature sacrificed
a permanent this way" names an Actor no player's sacrifice has. -/
theorem thisWayReadsTheActorItsSubjectNames :
    Action.check (DRS.ofList [])
      (sequentially
        [instruct (target isPlayer) (sacrifice (a isCreature)),
          doIf (did (that .player) (sacrifice (a isPermanent)) thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [sacrifice (a isCreature),
          doIf (did thisCreature (sacrifice (a isPermanent)) thisWay) (draw 1)]) =
      [.thisWayUngrounded] := by decide

/-- A cost's event is the ability's own: "{T}, Sacrifice a creature: If you sacrificed a
permanent this way, draw a card." -/
theorem thisWayReadsItsCostsEvent :
    Ability.check (DRS.ofList [])
      (activated (compound [tapSymbol, sacrifice (a isCreature)])
        (doIf (did you (sacrifice (a isPermanent)) thisWay) (draw 1))) = [] := by decide

/-- The text grounds "this way", not the game: an optional action the player may decline still
wrote its event, so "You may sacrifice a creature. If you sacrificed a permanent this way, …" is
read; whether the sacrifice happened is the engine's. -/
theorem optionalBodyEventGroundsThisWay :
    Action.check (DRS.ofList [])
      (sequentially
        [may (sacrifice (a isCreature)),
          doIf (did you (sacrifice (a isPermanent)) thisWay) (draw 1)]) = [] := by decide

/-- What a reflexive triggered ability does is its own [CR#603.12]: "Sacrifice a creature. When
you do, draw a card. If you drew a card this way, …" reads no draw of the enclosing ability. -/
theorem reflexiveAbilityEventsStayItsOwn :
    Action.check (DRS.ofList [])
      (sequentially
        [triggerReflexively (sacrifice (a isCreature)) (draw 1),
          doIf (did you (draw anyAmount) thisWay) (gainLife 1)]) = [.thisWayUngrounded] := by
  decide

/-- "When this creature enters, if it entered from your graveyard, …" reads the entry it
triggered on [CR#603.2]; an attack trigger names no entry. -/
theorem triggeringReadsItsTriggerEvent :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (doIf (did it (enter (from_ := yourGraveyard)) triggering) (draw 1))) = [] ∧
    Ability.check (DRS.ofList [])
      (when (does thisCreature attack)
        (doIf (did it (enter (from_ := yourGraveyard)) triggering) (draw 1))) =
      [.triggeringUngrounded] := by decide

/-- The subject of "dies" or "enters" is what the event moved, not its Actor [CR#700.4]: "Destroy
target creature. If it dies this way, …", "Target player puts a land card from their hand onto
the battlefield. If a land enters this way, …" and "Target player sacrifices a creature. If it
dies this way, …" each read the move; a player is no object a destruction moved. -/
theorem thisWayReadsWhatTheEventMoved :
    Action.check (DRS.ofList [])
      (sequentially [destroy (target isCreature), doIf (did it die thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [instruct (target isPlayer) (putOntoBattlefield (a (and [isLand, cardIn hand]))),
          doIf (did (a isLand) enter thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [instruct (target isPlayer) (sacrifice (a isCreature)),
          doIf (did it die thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially [destroy (target isCreature), doIf (did you die thisWay) (draw 1)]) =
      [.kindLte .player (Kind.join .object .pile), .thisWayUngrounded] := by decide

/-- "Destroy target creature. If that creature dies this way, …": a definite noun phrase as the
subject of the move a lookback reads is the moved patient, read as the permanent it was, as "it"
is (`semantics-v3.md`, the historical read) [CR#700.4,608.2h]. -/
theorem okThatCreatureDiesThisWay :
    Action.check (DRS.ofList [])
      (sequentially [destroy (target isCreature), doIf (did (that creature) die thisWay) (draw 1)]) =
      [] := by decide

/-- The subject's historical read keeps its noun: the destroyed creature is no artifact, so "If
that artifact dies this way" finds no antecedent. Where nothing moved the creature the lookback
reads no event, and after the lookback the dead creature is no current creature, so "Untap that
creature" finds none. -/
theorem thatNounDiesThisWayStaysBounded :
    Action.check (DRS.ofList [])
      (sequentially [destroy (target isCreature), doIf (did (that artifact) die thisWay) (draw 1)]) =
      [.anaphor (.word (.type .artifact)) .one 0] ∧
    Action.check (DRS.ofList [])
      (sequentially [tap (target isCreature), doIf (did (that creature) die thisWay) (draw 1)]) =
      [.thisWayUngrounded] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [destroy (target isCreature),
          doIf (did (that creature) die thisWay) (untap (that creature))]) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

/-- A trigger event's expansion is read layer by layer: "Whenever you sacrifice a creature, if it
died …" reads the sacrificed creature's move to a graveyard [CR#603.2,701.21a]. -/
theorem triggeringReadsWhatItsEventMoved :
    Ability.check (DRS.ofList [])
      (when (does you (sacrifice (a isCreature))) (doIf (did it die triggering) (draw 1))) =
      [] := by decide

/-- A permission writes the act it allows: "Exile the top three cards of your library. You may
play those cards this turn. If you cast a spell this way, …" reads the play, casting being how a
card is played as a spell [CR#701.18b], and "each spell cast this way" names what is cast that
way; a prohibition allows no act. -/
theorem thisWayReadsAPermittedAct :
    Action.check (DRS.ofList [])
      (sequentially
        [exile (topSlice 3),
          establishThroughout
            (can you (play (bare (and [isCard, was (exile which) thisWay])))) thisTurn,
          doIf (did you (cast (a isSpell)) thisWay) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [establishThroughout (can you (cast (a isSpell))) thisTurn,
          doForEach (each (and [isSpell, was (cast which) thisWay])) (draw 1)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [establishThroughout (cant you (cast (a isSpell))) thisTurn,
          doIf (did you (cast (a isSpell)) thisWay) (draw 1)]) = [.thisWayUngrounded] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [establishThroughout (cant you (cast (a isSpell))) thisTurn,
          doForEach (each (and [isSpell, was (cast which) thisWay])) (draw 1)]) =
      [.participialDescription (.deed (.action "cast")) 0] := by decide

/-- An event is read back by its class: gaining life is no losing it [CR#119.3], and paying a
cost is a payment however the cost is written [CR#118.1], so "{T}, Pay 2 life: If you paid life
this way, …" reads the cost. Exchanging life totals gains or loses life [CR#701.12c], so "You may
exchange life totals with target opponent. If you lost life this way, …" reads the exchange. -/
theorem thisWayClassesTheEvent :
    Action.check (DRS.ofList [])
      (sequentially [gainLife 2, doIf (did you (loseLife anyAmount) thisWay) (draw 1)]) =
      [.thisWayUngrounded] ∧
    Action.check (DRS.ofList [])
      (sequentially
        [may (exchange (lifeTotals (youAnd (target isOpponent)))),
          doIf (did you (loseLife anyAmount) thisWay) (draw 1)]) = [] ∧
    Ability.check (DRS.ofList [])
      (activated (compound [tapSymbol, payLife 2])
        (doIf (did you (payLife 2) thisWay) (draw 1))) =
      [] := by decide

private def tapAndPayLife (cost : Cost) (lookback : Action) : Ability :=
  activated (compound [tapSymbol, cost]) (doIf (did you lookback thisWay) (draw 1))

/-- "{T}, Pay 2 life: Draw a card if you paid life this way." and "… if you lost life this way.":
paying life is a payment that is a loss of life [CR#119.4], and a later "this way" reads back
what the ability's earlier text did [CR#608.2c], its cost included, so both lookbacks ground; no
sacrifice was caused, so "if you sacrificed a creature this way" does not. The coerced `perform (payLife 2)` records its payment
more than once, but a lookback asks only whether such an event was caused: every reading is the
one the bare `perform (loseLife 2)` gives, and the events recorded are the same events. -/
theorem costPaidLifeGroundsPaidAndLost :
    [payLife anyAmount, loseLife anyAmount, sacrifice (a isCreature)].map (fun lookback =>
      Ability.check (DRS.ofList []) (tapAndPayLife (payLife 2) lookback)) =
      [[], [], [.thisWayUngrounded]] ∧
    [payLife anyAmount, loseLife anyAmount, sacrifice (a isCreature)].map (fun lookback =>
      Ability.check (DRS.ofList []) (tapAndPayLife (perform (loseLife 2)) lookback)) =
      [[], [], [.thisWayUngrounded]] ∧
    ((stepCost 64 (DRS.ofList []) (payLife 2)).1.causedEvents.eraseDups ==
      (stepCost 64 (DRS.ofList []) (perform (loseLife 2))).1.causedEvents.eraseDups) = true := by
  decide

end Semantics.Proofs.Lookback
