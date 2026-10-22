import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Turn

Port of `idris/src/Experimental/Proofs/Turn.idr`: the pins of the Turn family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Turn

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

/-- An activated ability with no limit, guard, or activator. -/
def activatedAbility (cost : Cost) (action : Action) (timing : Option Timing := none) : Ability :=
  .activated cost action timing none none none

/-- "at the beginning of the end step" -/
def atTheEndStep : GameEvent := .beginningOf .endStep none (.each none)

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefield : Action.check (DRS.ofList []) (sacrifice (a isCreature)) = []
    := by
  decide

/-- A delayed trigger reads the earlier target as a settled particular [CR#603.7c]: an earlier
mention in scope, so the delayed "any other target" contrasts with it, not with the Source
(`semantics-v3.md` §0.1). -/
theorem okDelayedOtherAnchorsOnTheSettledTarget :
    let action := Action.sequentially
      [ (Action.instruct .this (.dealDamage 2 (target isAnyTarget))),
        delay atTheEndStep (Action.instruct .this (.dealDamage 1 (target isAnyOtherTarget))) ]
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) action).1
    Action.check (DRS.ofList []) action = [] ∧ after.contrastAnchors = [.mention 2] ∧
      (after.findById? 2).map (·.kind) = some (.join .object .player) := by
  decide

theorem badStaleCarrier :
    Action.check (DRS.ofList [])
      (.sequentially [exile (target (and [isCreature, youControl])), putInto (that (.type .creature)) battlefield])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okActivatedCostAndEffect :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a isCreature))) (draw 1)) = [] := by
  decide

/-- "Return a creature to its owner's hand: Tap it." The Idris pin refutes the anaphor. -/
theorem badHiddenCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (putInto (a isCreature) hand)) (.setStatus .tapped it))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Discard a card, Sacrifice a creature: Exile it." -/
theorem badTwoCostMentions :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound
          [.perform (discard (a (.and [.isCard, .in_ yourHand]))), .perform (sacrifice (a isCreature))])
        (exile it)) = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature you control. Sacrifice it." -/
theorem okSacrificeOnBattlefield :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target (and [isCreature, youControl])), sacrifice it])
      = [] := by
  decide

theorem badDeadCreatureRead :
    Action.check (DRS.ofList [])
      (.delay (does (target isCreature) die) [] (some thisTurn)
        (putInto (that (.type .creature)) battlefield))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Discard a card: Return the discarded card to the battlefield." -/
theorem okParticipialDiscardedCard :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (a (.and [.isCard, .in_ yourHand]))))
        (putInto (the (and [was (discard which), isCard])) battlefield)) = [] := by
  decide

/-- "Discard a card: Return the sacrificed card to the battlefield." -/
theorem badParticipialWrongVerb :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (a (.and [.isCard, .in_ yourHand]))))
        (putInto (the (and [was (sacrifice which), isCard])) battlefield))
      = [.participialDescription (.deed (.action "sacrifice")) 0] := by
  decide

/-- "Sacrifice an artifact: Return the sacrificed creature to the battlefield." -/
theorem badParticipialWrongNoun :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (sacrifice (a isArtifact)))
        (putInto (the (and [was (sacrifice which), isCreature])) battlefield))
      = [.participialDescription (.deed (.action "sacrifice")) 0] := by
  decide

theorem badParticipialAmbig :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound [.perform (sacrifice (a isCreature)), .perform (sacrifice (a
            isCreature))])
        (putInto (the (and [was (sacrifice which), isCard])) battlefield))
      = [.participialDescription (.deed (.action "sacrifice")) 2] := by
  decide

theorem okBareCardRead :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (sacrifice (a isCreature)))
        (.sequentially
          [exile (target isCreature), delay atTheEndStep (putInto (that .card) battlefield)]))
      = [] := by
  decide

private def bareCardReadContext : DRS :=
  let afterCost := (stepCost 64 (DRS.ofList []) (.perform (sacrifice (a isCreature)))).1
  (stepAction 64 .postEvent .obj (nextClause (publicOnly afterCost))
    (exile (target isCreature))).1

theorem bareCardReadSelectsExiledTarget :
    (firstReach (.word .card) .one bareCardReadContext).map (·.id) =
      some (bareCardReadContext.nextId - 1) ∧
      (firstReach (.word .card) .one bareCardReadContext).bind (·.zone) = some .exile := by decide

/-- "Tap target creature." -/
theorem okTapOnBattlefield : Action.check (DRS.ofList []) (.setStatus .tapped (target isCreature)) = [] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okTriggerAtYourUpkeep :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw 1)) = [] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okUntilEndOfTurnDuration :
    Action.check (DRS.ofList [])
      (get (target isCreature) (.up 3) (.up 3) (some untilEndOfTurn)) = [] := by
  decide

/-- "Sacrifice a creature. When you do, draw a card." -/
theorem okReflexiveOnSacrifice :
    Action.check (DRS.ofList []) (.triggerReflexively (sacrifice (a isCreature)) none
        (draw 1))
      = [] := by
  decide

/-- "This creature deals 3 damage to any target. When you do, draw a card." -/
theorem badReflexiveOnSourceDeed :
    Action.check (DRS.ofList [])
      (.triggerReflexively (Action.instruct .this (.dealDamage 3 (target isAnyTarget)))
        none (draw 1))
      = [.reflexEnclosure] := by
  decide

/-- "You gain 2 life. When you do, draw a card." -/
theorem badReflexiveOnLifeGain :
    Action.check (DRS.ofList []) (.triggerReflexively (gainLife 2) none (draw 1))
      = [.reflexEnclosure] := by
  decide

/-- "Draw a card, then sacrifice a creature. When you do, draw a card." -/
theorem badReflexiveOnSequence :
    Action.check (DRS.ofList [])
      (.triggerReflexively (.sequentially [draw 1, sacrifice (a isCreature)]) none
        (draw 1)) = [.reflexEnclosure] := by
  decide

theorem badReflexiveOnDelayed :
    Action.check (DRS.ofList [])
      (.triggerReflexively (delay (.beginningOf .endStep (some .you) (.next none)) (draw 1))
        none (draw 1)) = [.reflexEnclosure] := by
  decide

/-- "Regenerate this creature. If it regenerates this way, draw a card." -/
theorem okThisWayOnRegenerate :
    Action.check (DRS.ofList [])
      (.triggerReflexively (regenerate thisCreature)
        (some (does any (regeneration thisCreature))) (draw 1))
      = [] := by
  decide

theorem badThisWayOnDelayed :
    Action.check (DRS.ofList [])
      (.triggerReflexively (delay (.beginningOf .endStep (some .you) (.next none)) (draw 1))
        (some (.does (.actor .you) (.draw (.exactlyOf .anyAmount)))) (draw 1)) = [.thisWayOutcome] := by
  decide

theorem badReflexiveOnBranchedMay :
    Action.check (DRS.ofList [])
      (.triggerReflexively (may (sacrifice (a isCreature)) (some (draw 1))) none
        (draw 1)) = [.reflexEnclosure] := by
  decide

/-- The Idris pin refutes the anaphor. -/
theorem badAfterReflexiveReadsTrigger :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .triggerReflexively (mill 4) none
            (create 1 (.written (creatureToken 1 1 [.white] [soldier]))),
          .setStatus .tapped (that (.type .creature)) ])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "target creature blocking this creature" -/
theorem okBlockingThisCreature :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isCreature, .inCombat .blockerOf (some thisCreature)])) = [] := by
  decide

/-- "Activate only during each player's end step." -/
theorem okDistributivePartWindow :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw 1)
        (some (duringPart .endStep (some (each .anyPlayer))))) = [] := by
  decide

/-- "until the beginning of your next upkeep" -/
theorem okDurationEndYourUpkeep :
    Duration.check (DRS.ofList []) (untilEvent (.beginningOf .upkeep (some .you) (.next none))) = [] := by decide

/-- "until the beginning of each player's next upkeep" -/
theorem badDurationEndEachPlayers :
    Duration.check (DRS.ofList []) (untilEvent (.beginningOf .upkeep (some (each .anyPlayer)) (.next none)))
      = [.durationPossessor] := by
  decide

/-- "until the end of your next combat" -/
theorem okDurationEndYourCombat :
    Duration.check (DRS.ofList []) (untilEvent (.endOf .combat (some .you) (.next none))) = [] := by decide

/-- "until the end of an opponent's combat" -/
theorem badDurationEndAnOpponent :
    Duration.check (DRS.ofList []) (untilEvent (.endOf .combat (some anOpponent) (.next none)))
      = [.durationPossessor] := by decide

/-- "You get an emblem with 'At the beginning of your end step, draw a card.'" -/
theorem okTriggeredEmblem :
    Action.check (DRS.ofList [])
      (getEmblem
        [at_ (.beginningOf .endStep (some .you) (.each none)) (draw 1)])
      = [] := by
  decide

/-- "You get an emblem." -/
theorem badEmptyEmblem :
    Action.check (DRS.ofList []) (getEmblem []) = [.nonEmpty] := by decide

/-- "… At the beginning of that turn's end step, you lose the game." -/
theorem okDeicticTurnAfterExtraTurn :
    Action.check (DRS.ofList [])
      (.sequentially
        [ extraTurn 1,
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [] := by
  decide

/-- "Draw a card. At the beginning of that turn's end step, you lose the game." -/
theorem badDeicticTurnWithoutIntroducer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ draw 1,
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [.turnAntecedent 0] := by
  decide

/-- "After this combat phase, there is an additional upkeep step." -/
theorem okAdditionalUpkeep :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep (some .combat) 1 none) = [] := by
  decide

/-- "Spells with the chosen name can't be cast." -/
theorem okCastSpellClass :
    StaticSpec.check [qualityB .cardName]
      (cant any (cast (allOf (.and [isSpell, .named .chosen]))))
      = [] := by
  decide

/-- "Activated and triggered abilities can't be activated.": a disjunction seeds an ability
when every arm does, so the deed's ability role is met [CR#113.1c]. -/
theorem okAbilityDisjunctionActivated :
    StaticSpec.check (DRS.ofList [])
      (cant any (activate (allOf (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered]))))
      = [] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okSingularPartPossessor :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw 1)) = [] := by
  decide

/-- "At the beginning of combat on your turn, draw a card." The beginning of combat step is
its own part of the combat phase [CR#506.1]. -/
theorem okBeginningOfCombatPossessor :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .beginningOfCombat (some .you) (.each none)) (draw 1))
      = [] := by
  decide

/-- "{2}: Draw a card. Activate only during each player's ending phase." The ending phase is the
last of the turn's five phases [CR#500.1]. -/
theorem okEndingPhaseWindow :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw 1)
        (some (duringPart .endingPhase (some (each .anyPlayer))))) = [] := by
  decide

/-- "After this postcombat main phase, there is an additional ending phase." [CR#500.1] -/
theorem okAdditionalEndingPhase :
    Action.check (DRS.ofList [])
      (Primitives.Action.addTurnPart .endingPhase (some .postcombatMain) 1 none) = [] := by
  decide

/-- "creature that could block each attacking creature" -/
theorem okCouldBlockAttacker :
    Predicate.checkAt .object (DRS.ofList []) (.inCombat .couldBlock (some (allOf (.and [isCreature, isAttacking]))))
      = [] := by
  decide

/-- "player an opponent is attacking": the attacker of an attacked player may be a player
[CR#506.2]. -/
theorem okAttackedByPlayer :
    Predicate.checkAt .player (DRS.ofList []) (.inCombat .attackedBy (some anOpponent)) = [] := by decide

/-- "Sacrifice a creature. If you do, draw a card." -/
theorem okIfDoneWithArm :
    Action.check (DRS.ofList []) (doIfDone (sacrifice (a isCreature)) (some (draw 1)))
      = [] := by
  decide

/-- "Sacrifice a creature." -/
theorem badIfDoneWithNeitherArm :
    Action.check (DRS.ofList []) (doIfDone (sacrifice (a isCreature))) =
        [.ifDoneArmed] := by
  decide

/-- "This creature deals 3 damage to any target. If you do, draw a card." -/
theorem badIfDoneOverAgentlessBody :
    Action.check (DRS.ofList [])
      (doIfDone (Action.instruct thisCreature (.dealDamage 3 (target isAnyTarget))) (some (draw 1))) = [.reflexEnclosure] := by
  decide

/-- "Take an extra turn after this one. If you do, draw a card." -/
theorem badIfDoneOverScheduledBody :
    Action.check (DRS.ofList []) (doIfDone (extraTurn 1) (some (draw 1)))
      = [.reflexEnclosure] := by
  decide

def oneExtraTurn : DRS := actionResult (DRS.ofList []) (extraTurn 1)

/-- "Take an extra turn after this one. Skip the draw step of that turn." -/
theorem okThatTurnAfterASingleTurn :
    TurnRef.check oneExtraTurn .that = [] := by decide

def twoExtraTurns : DRS := actionResult oneExtraTurn (extraTurn 1)

theorem badThatTurnAfterTwoTurns :
    TurnRef.check twoExtraTurns .that = [.turnAntecedent 2] := by
  decide

/-- "create a legendary 20/20 black Avatar creature token named Marit Lage" -/
theorem okTokenSingleSupertype :
    Action.check (DRS.ofList [])
      (create 1
        (.written { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary],
            types := [.creature], subtypes := [avatar],
            power := some 20, toughness := some 20 } })) = [] := by
  decide

/-- "create a legendary legendary 20/20 black Avatar creature token" -/
theorem badTokenDuplicateSupertype :
    Action.check (DRS.ofList [])
      (create 1
        (.written { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary, .legendary],
            types := [.creature], subtypes := [avatar],
            power := some 20, toughness := some 20 } })) = [.distinct] := by
  decide

/-! ## Constructor fields share the preceding discourse -/

theorem okExtraTurnAmountReadsSubject :
    Action.check [qualityB .color]
      (Templates.instruct (target (.isOpponentOf .you)) (extraTurn (lifeTotalOf they))) = [] := by decide

theorem badExtraTurnAmountReadsSubject :
    Action.check [qualityB .color]
      (extraTurn (lifeTotalOf they)) = [.anaphor (.word .player) .one 0] := by decide

theorem badExtraTurnForwardSubject :
    Action.check (DRS.ofList []) (Templates.instruct they (extraTurn (lifeTotalOf (target (.isOpponentOf .you)))))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okExtraTurnAmountIntroducesPlayer :
    Action.check (DRS.ofList []) (.sequentially
      [extraTurn (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.exactlyOf 1))]) =
          [] := by decide

theorem okExtraTurnNestedAmountIntroducesLetterOnce :
    countLetter .x (actionResult (DRS.ofList [])
      (extraTurn (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okSkipNextAmountReadsSubject :
    Action.check [qualityB .color]
      (Templates.instruct (target (.isOpponentOf .you)) (.skipPart .drawStep (lifeTotalOf they))) = [] := by decide

theorem badSkipNextAmountReadsSubject :
    Action.check [qualityB .color]
      (.skipPart .drawStep (lifeTotalOf they))
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badSkipNextForwardSubject :
    Action.check (DRS.ofList []) (Templates.instruct they (.skipPart .drawStep (lifeTotalOf (target (.isOpponentOf .you)))))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okSkipNextAmountIntroducesPlayer :
    Action.check (DRS.ofList []) (.sequentially
      [.skipPart .drawStep (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.exactlyOf 1))])
      = [] := by
  decide

theorem okSkipNextNestedAmountIntroducesLetterOnce :
    countLetter .x (actionResult (DRS.ofList [])
      (.skipPart .drawStep (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okAdditionalPartAmountReadsSubject :
    Action.check [qualityB .color]
      (Templates.instruct (target (.isOpponentOf .you)) (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none))
      = [] := by
  decide

theorem badAdditionalPartAmountReadsSubject :
    Action.check [qualityB .color]
      (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none)
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badAdditionalPartForwardSubject :
    Action.check (DRS.ofList []) (Templates.instruct they (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf (target (.isOpponentOf .you))) none))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okAdditionalPartAmountIntroducesPlayer :
    Action.check (DRS.ofList []) (.sequentially
      [Primitives.Action.addTurnPart .upkeep none (lifeTotalOf (target (.isOpponentOf .you))) none, Templates.instruct they (.draw
          (.exactlyOf 1))])
      = [] := by
  decide

theorem okAdditionalPartNestedAmountIntroducesLetterOnce :
    countLetter .x (actionResult (DRS.ofList [])
      (Primitives.Action.addTurnPart .upkeep none (plus (.letter .x) (.letter .x)) none))
      = 1 := by
  decide

theorem okUntapAmountReadsSubject :
    Action.check [qualityB .color]
      (.skipUntap (target isCreature) (powerOf it)) = [] := by decide

theorem untapAmountReadsItsPublishedSelfSubject :
    let before := DRS.ofList [qualityB .color]
    let selected := (stepNounPhrase 64 before .obj true thisCreature).1
    Action.check before (.skipUntap thisCreature (powerOf it)) = [] ∧
      (firstReach .bare .one selected).map (·.id) = some 1 := by decide

theorem okUntapNestedAmountIntroducesLetterOnce :
    countLetter .x (actionResult (DRS.ofList [])
      (.skipUntap (target isCreature) (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okUntapAmountIntroducesPlayer :
    Action.check (DRS.ofList []) (.sequentially
      [.skipUntap (target isCreature) (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.exactlyOf 1))])
      = [] := by decide

theorem okAdditionalPartWithoutSubjectReadsOuterContext :
    Action.check (nounResult (DRS.ofList []) (target (.isOpponentOf .you)))
      (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none) = [] := by decide

theorem badAdditionalPartWithoutSubjectOrOuterContext :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none)
      = [.anaphor (.word .player) .one 0] := by decide

/-- Nested amounts preserve the identities and kinds of their introduced targets. -/
theorem okExtraTurnNestedAmountOrder :
    let after := actionResult [qualityB .color]
      (extraTurn (plus (powerOf (target isCreature)) (lifeTotalOf (target (.isOpponentOf .you)))))
    after.visibleReferents.map (fun r => (r.id, r.kind)) =
      [(3, .turnRef), (2, .player), (1, .object), (0, .quality .color)] ∧
      (after.findById? 1).map (·.ty) = some [.creature] := by decide

theorem okSkipNextNestedAmountOrder :
    let after := actionResult (DRS.ofList [])
      (.skipPart .drawStep (plus (powerOf (target isCreature)) (lifeTotalOf (target (.isOpponentOf .you)))))
    after.visibleReferents.map (fun r => (r.id, r.kind)) = [(1, .player), (0, .object)] ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

theorem okAdditionalPartNestedAmountOrder :
    let after := actionResult (DRS.ofList [])
      (Primitives.Action.addTurnPart .upkeep none
        (plus (powerOf (target isCreature)) (lifeTotalOf (target (.isOpponentOf .you)))) none)
    after.visibleReferents.map (fun r => (r.id, r.kind)) = [(1, .player), (0, .object)] ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

theorem okUntapNestedAmountOrder :
    let after := actionResult (DRS.ofList [])
      (.skipUntap .this (plus (powerOf (target isCreature)) (lifeTotalOf (target (.isOpponentOf .you)))))
    after.visibleReferents.map (fun r => (r.id, r.kind)) =
      [(2, .player), (1, .object), (0, .object)] ∧
      (after.findById? 1).map (·.ty) = some [.creature] ∧
      (after.findById? 0).map (·.ty) = some [] := by decide

/-! ## The temporal sorts (`semantics-v2-temporal-suite`)

A timing, a duration and a usage limit are checked for references and kinds only (§7 ruling
2026-10-08, "Time is three sorts"). -/

/-- "Activate only during this creature's upkeep": whose turn it is must be a player's. -/
theorem badTimingWhoseNotAPlayer :
    Timing.check (DRS.ofList []) (duringPart .upkeep (some thisCreature)) = [.kindMismatch .player .object] := by
  decide

/-- "Activate only during that turn's end step", with no turn introduced before it. -/
theorem badTimingThatTurnWithoutAntecedent :
    Timing.check (DRS.ofList []) (.inPart .endStep none .that) = [.turnAntecedent 0] := by decide

/-- "… during that turn's end step" after "take an extra turn after this one". -/
theorem okTimingThatTurnAfterExtraTurn :
    Timing.check oneExtraTurn (.inPart .endStep none .that) = [] := by decide

/-- "Activate only as a sorcery" [CR#307.5] and "only as an instant" [CR#117.1a]. -/
theorem okSorceryAndInstantSpeed :
    Timing.check (DRS.ofList []) asSorcery ++ Timing.check (DRS.ofList []) asInstant = [] := by decide

/-- "only as a sorcery" requires priority, as "only as an instant" does, besides your main phase
and an empty stack [CR#307.5]; so its negation also holds without priority [CR#307.5a]. No
printed card observes the difference (Mystic Veil ruling, 2022-12-08), so the pin is on the
tree. -/
theorem sorcerySpeedRequiresPriority :
    asSorcery = .and [.onlyIf (.hasPriority .you), .inPart .mainPhase (some .you) .current,
      .onlyIf (.not (exists_ (.in_ stack)))] := rfl

/-- "Until your next turn": the end is the beginning of your next turn. -/
theorem okUntilYourNextTurn : Duration.check (DRS.ofList []) untilYourNextTurn = [] := by decide

/-- "During your next turn": both boundaries are your next turn's. -/
theorem okDuringYourNextTurn : Duration.check (DRS.ofList []) (duringNextTurnOf .you) = [] := by decide

/-- "On a later turn" [CR#702.170d] and the history windows. -/
theorem okLaterTurnAndHistoryWindows :
    Duration.check (DRS.ofList []) onALaterTurn ++ Duration.check (DRS.ofList []) soFarThisTurn ++
      Duration.check (DRS.ofList []) lastTurn ++ Duration.check (DRS.ofList []) thisGame = [] := by decide

/-- "Activate only once each turn" [CR#602.5b]. -/
theorem okOncePerTurn : UsageLimit.check (DRS.ofList []) oncePerTurn = [] := by decide

/-- "the next turn of this creature": a turn reference names a player's turns. -/
theorem badNextTurnOfACreature :
    TurnRef.check (DRS.ofList []) (.next (some thisCreature)) = [.kindMismatch .player .object] := by decide

/-- Flash's definition [CR#702.8a]: a play permission at instant speed. -/
theorem okFlash : Ability.check (DRS.ofList []) flash = [] := by decide

/-- Sneak {U}'s definition [CR#702.190a]: a cast permission at instant speed in your declare
blockers step, paying {U} and returning an unblocked creature you control. -/
theorem okSneak : Ability.check (DRS.ofList []) (sneak (.mana [pip .blue])) = [] := by decide

/-- "Unblocked" is its own combat status, not "not blocked": an attacking creature is neither
blocked nor unblocked until blockers are declared [CR#509.1h], so neither ninjutsu nor sneak can
return an attacker before then, and sneak cannot return a creature that is not attacking. -/
theorem unblockedIsItsOwnStatus :
    (isUnblocked : Predicate) = .inCombat .unblocked none ∧
      (isUnblocked : Predicate) ≠ .not isBlocked :=
  ⟨rfl, fun h => by cases h⟩

/-- Mystic Veil: "If you cast it any time a sorcery couldn't have been cast" [CR#307.5a], the
timing qualifier on a lookback. -/
theorem okCastWhenASorceryCouldNotHaveBeen :
    Condition.check (DRS.ofList [])
      (did (.actor .you) (.procedure .cast .this .itsOwnCost none none none) (within thisGame (some (.not asSorcery))))
      = [] := by
  decide

/-- "Whenever you cast a spell, you may draw a card. Do this only once each turn." [CR#603.2h]:
the limit counts the action, so the ability triggers only if you haven't yet drawn this way
this turn (Iron Man, Bleeding Edge and Donal, Herald of Wings; the retired
`UsageLimit.actionOncePerTurn`). -/
theorem okDoThisOnlyOnceEachTurn :
    Ability.check (DRS.ofList []) (whenever (.does (.actor .you) (.procedure .cast (a isSpell) .itsOwnCost none none none))
      (doOnlyOnceEachTurn (may (draw 1)))) = [] := by
  decide

/-- The limited action is checked as the action it wraps. -/
theorem limitedChecksItsAction :
    Action.check (DRS.ofList []) (.limited (putInto it hand) oncePerTurn) =
      Action.check (DRS.ofList []) (putInto it hand) := by
  decide

end Semantics.Proofs.Turn
