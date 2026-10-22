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

theorem scopedPlayerDraw :
    relativeCheck .player ⟨.actor .which, .draw (.exactlyOf .anyAmount), some withinThisTurn⟩ = [] := by decide

theorem playerGapCannotStandForObject :
    relativeCheck .object ⟨.actor .which, .draw (.exactlyOf .anyAmount), some withinThisTurn⟩ =
      [.kindMismatch .player .object] := by decide

theorem gapRequiresLookback :
    GameEvent.check (DRS.ofList []) (.does (.actor .which) (.draw (.exactlyOf .anyAmount))) =
      [.anaphor (.word .permanent) .one 0] := by decide

theorem scopedCreatureDeath :
    relativeCheck .object
      ⟨.any, .move (.headed (.type .creature) .which)
        (.zone .battlefield none) (some (.zone .graveyard none)) .departure none [],
        some withinThisTurn⟩ = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did ⟨.any, .move (.headed (.type .creature) .which)
                (.zone .battlefield none) (some (.zone .graveyard none))
                .departure none [], some withinThisTurn⟩]))
          .itsOwnCost none none none,
        some withinThisTurn⟩ = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did ⟨.actor .which, .draw (.exactlyOf .anyAmount), some withinThisTurn⟩]))
          .itsOwnCost none none none,
        some withinThisTurn⟩ = [.kindMismatch .player .object] := by decide

theorem lookbackRequiresBoundParticipant :
    relativeCheck .player ⟨.actor .you, .draw (.exactlyOf .anyAmount), some withinThisTurn⟩ =
      [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    relativeCheck .object
      ⟨.actor .you, .draw (.exactlyOf .anyAmount),
        some (.duration soFarThisTurn
          (some (.onlyIf (.exists_ (.described .bare
            (.and [.hasType .creature,
              .did ⟨.any, .move .which (.zone .battlefield none)
                (some (.zone .graveyard none)) .departure none [], some withinThisTurn⟩]))))))⟩ =
      [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    relativeCheck .object
      ⟨.actor (.headed (.type .creature) .which),
        .dealDamage .anyAmount (.headed (.type .creature) .which),
        some withinThisTurn⟩ = [] := by decide

private def damageHistory : Predicate :=
  .did ⟨.actor (.headed (.type .creature) .which),
    .dealDamage .anyAmount (.described (.a .unmarked) (.isOpponentOf .you)), some withinThisTurn⟩

theorem historyDoesNotIntroduceDamageOutcome :
    countOutcomes .damageDealt (stepPredicate 64 (DRS.ofList []) damageHistory).1 = 0 := by decide

theorem historyDoesNotExportNamedParticipant :
    NounPhrase.checkAt (some .player) (stepPredicate 64 (DRS.ofList []) damageHistory).1
      (.pro (.word .player) .one) = [.anaphor (.word .player) .one 0] := by decide

theorem historyNamedParticipantAccessibleLocally :
    relativeCheck .object
      ⟨.actor .which,
        .dealDamage (.statOf (.playerStat .lifeTotal) (.described (.a .unmarked) (.isOpponentOf .you)))
          (.pro (.word .player) .one),
        some withinThisTurn⟩ = [] := by decide

theorem castCanCarryExcludedOrigins :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast (.described (.a .unmarked) (.and [.isCard, .in_ (.zone .stack none)]))
          .itsOwnCost none (some (.not (.zone .hand (some .you)))) none,
        some withinThisTurn⟩ = [] := by decide

theorem namedActionCanCarryItsLocus :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "search") (.search (.zone .library (some .you)) .any) none
          (some (.zone .library (some .you))),
        some .thisWay⟩ = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "shuffle") (.shuffle (.zone .library (some .you)))
          none (some (.zone .graveyard (some .you))),
        some .thisWay⟩ = [.lookbackLocus] := by decide

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
      ⟨.actor .which,
        .procedure .cast (.headed .spell (.designated (.named "commander") .you))
          .itsOwnCost none none none,
        some (.duration thisGame none)⟩ = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (part : Option TurnPart) (cause : Causing) (fuel : Nat) :
    (GameEvent.nthOccurrence ordinal part event).facts (fuel + 1) = event.facts fuel ∧
      (GameEvent.causes cause event).facts (fuel + 1) = event.facts fuel := ⟨rfl, rfl⟩

/-! ## Unwritten spans

A lookback with no span written is an attributive participle, read only by a Participial
Description ("the destroyed creature"). A condition and a tally always write their span, and so
does a relative clause under any other determiner. -/

theorem unanchoredConditionRefused :
    Condition.check (DRS.ofList []) (.did ⟨.any, destroy (a isCreature), none⟩) =
      [.unanchoredLookback] := by decide

theorem unanchoredTallyRefused :
    Amount.check (DRS.ofList []) (.tally ⟨.any, destroy (a isCreature), none⟩ .events) =
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

end Semantics.Proofs.Lookback
