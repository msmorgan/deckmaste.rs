import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Turn

Port of `idris/src/Experimental/Proofs/Turn.idr`: the pins of the Turn family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Turn

private def instructionResult (drs : DRS) (instruction : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs instruction).1

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

/-- An activated ability with no limit, guard, or activator. -/
def activatedAbility (cost : Cost) (instruction : Action) (timing : Option Timing := none) : Ability :=
  .activated cost instruction timing none none none

/-- "at the beginning of the end step" -/
def atTheEndStep : GameEvent := .beginningOf .endStep none (.each none)

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefield : Action.check (DRS.ofList []) (sacrifice (a creature)) = []
    := by
  decide

/-- The Idris pin refutes the "other" anchor; the targeting law the delayed clause loses is
listed too. -/
theorem badDelayedOther :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage (.lit 2) (target anyTarget) .any)),
          delay atTheEndStep (Action.instruct .this (.dealDamage (.lit 1) (target anyOtherTarget) .any)) ])
      = [.anyTargeted (.join .object .player), .otherAnchored] := by
  decide

theorem badStaleCarrier :
    Action.check (DRS.ofList [])
      (.sequentially [exile (target creatureYouControl), move (that (.type .creature)) battlefield])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okActivatedCostAndEffect :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a creature))) (draw (.lit 1))) = [] := by
  decide

/-- "Return a creature to its owner's hand: Tap it." The Idris pin refutes the anaphor. -/
theorem badHiddenCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (move (a creature) hand)) (.setStatus .tapped it))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Discard a card, Sacrifice a creature: Exile it." -/
theorem badTwoCostMentions :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound
          [.perform (discard (a (.and [.isCard, .in_ yourHand]))), .perform (sacrifice (a creature))])
        (exile it)) = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature you control. Sacrifice it." -/
theorem okSacrificeOnBattlefield :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target creatureYouControl), sacrifice it])
      = [] := by
  decide

theorem badDeadCreatureRead :
    Action.check (DRS.ofList [])
      (.delay (dies (target creature)) [] (some thisTurn)
        (move (that (.type .creature)) battlefield))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Discard a card: Return the discarded card to the battlefield." -/
theorem okVerbedDiscardedCard :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (a (.and [.isCard, .in_ yourHand]))))
        (move (theVerbed (.action "discard") .one (some .card)) battlefield)) = [] := by
  decide

/-- "Discard a card: Return the sacrificed card to the battlefield." -/
theorem badVerbedWrongVerb :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (a (.and [.isCard, .in_ yourHand]))))
        (move (theVerbed (.action "sacrifice") .one (some .card)) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some .card) .attributive) .one 0] := by
  decide

/-- "Sacrifice an artifact: Return the sacrificed creature to the battlefield." -/
theorem badVerbedWrongNoun :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (sacrifice (a artifact)))
        (move (theVerbed (.action "sacrifice") .one (some (.type .creature))) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some (.type .creature)) .attributive) .one 0] := by
  decide

theorem badVerbedAmbig :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound [.perform (sacrifice (a creature)), .perform (sacrifice (a
            creature))])
        (move (theVerbed (.action "sacrifice") .one (some .card)) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some .card) .attributive) .one 2] := by
  decide

theorem okBareCardRead :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (sacrifice (a creature)))
        (.sequentially
          [exile (target creature), delay atTheEndStep (move (that .card) battlefield)]))
      = [] := by
  decide

private def bareCardReadContext : DRS :=
  let afterCost := (stepCost 64 (DRS.ofList []) (.perform (sacrifice (a creature)))).1
  (stepAction 64 .postEvent .obj (nextClause (publicOnly afterCost))
    (exile (target creature))).1

theorem bareCardReadSelectsExiledTarget :
    (firstReach (.word .card) .one bareCardReadContext).map (·.id) =
      some (bareCardReadContext.nextId - 1) ∧
      (firstReach (.word .card) .one bareCardReadContext).bind (·.zone) = some .exile := by decide

/-- "Tap target creature." -/
theorem okTapOnBattlefield : Action.check (DRS.ofList []) (.setStatus .tapped (target creature)) = [] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okTriggerAtYourUpkeep :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw (.lit 1))) = [] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okUntilEndOfTurnDuration :
    Action.check (DRS.ofList [])
      (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Sacrifice a creature. When you do, draw a card." -/
theorem okReflexiveOnSacrifice :
    Action.check (DRS.ofList []) (.triggerReflexively (sacrifice (a creature)) (draw (.lit
        1)))
      = [] := by
  decide

/-- "This creature deals 3 damage to any target. When you do, draw a card." -/
theorem badReflexiveOnSourceDeed :
    Action.check (DRS.ofList [])
      (.triggerReflexively (Action.instruct .this (.dealDamage (.lit 3) (target anyTarget) .any)) (draw (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "You gain 2 life. When you do, draw a card." -/
theorem badReflexiveOnLifeGain :
    Action.check (DRS.ofList []) (.triggerReflexively (gainLife (.lit 2)) (draw (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "Draw a card, then sacrifice a creature. When you do, draw a card." -/
theorem badReflexiveOnSequence :
    Action.check (DRS.ofList [])
      (.triggerReflexively (.sequentially [draw (.lit 1), sacrifice (a creature)])
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

theorem badReflexiveOnDelayed :
    Action.check (DRS.ofList [])
      (.triggerReflexively (delay (.beginningOf .endStep (some .you) (.next none)) (draw (.lit 1)))
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- "Regenerate this creature. If it regenerates this way, draw a card." -/
theorem okThisWayOnRegenerate :
    Action.check (DRS.ofList [])
      (.triggerThisWay (regenerate thisCreature) (regenerates thisCreature) (draw (.lit 1)))
      = [] := by
  decide

theorem badThisWayOnDelayed :
    Action.check (DRS.ofList [])
      (.triggerThisWay (delay (.beginningOf .endStep (some .you) (.next none)) (draw (.lit 1)))
        (.does (.actor .you) (.draw .anyAmount)) (draw (.lit 1))) = [.thisWayOutcome] := by
  decide

theorem badReflexiveOnBranchedMay :
    Action.check (DRS.ofList [])
      (.triggerReflexively (Primitives.Action.offer (sacrifice (a creature)) (some (draw (.lit 1))) none)
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- The Idris pin refutes the anaphor. -/
theorem badAfterReflexiveReadsTrigger :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .triggerReflexively (mill (.lit 4) .you)
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          .setStatus .tapped (that (.type .creature)) ])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "target creature blocking this creature" -/
theorem okBlockingThisCreature :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [creature, .inCombat .blockerOf (some thisCreature)])) = [] := by
  decide

/-- "Activate only during each player's end step." -/
theorem okDistributivePartWindow :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw (.lit 1))
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
      (Primitives.Action.getEmblem
        [at_ (.beginningOf .endStep (some .you) (.each none)) (draw (.lit 1))])
      = [] := by
  decide

/-- "You get an emblem." -/
theorem badEmptyEmblem :
    Action.check (DRS.ofList []) (Primitives.Action.getEmblem []) = [.nonEmpty] := by decide

/-- "… At the beginning of that turn's end step, you lose the game." -/
theorem okDeicticTurnAfterExtraTurn :
    Action.check (DRS.ofList [])
      (.sequentially
        [ extraTurn (.lit 1),
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [] := by
  decide

/-- "Draw a card. At the beginning of that turn's end step, you lose the game." -/
theorem badDeicticTurnWithoutIntroducer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ draw (.lit 1),
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [.turnAntecedent 0] := by
  decide

/-- "After this combat phase, there is an additional upkeep step." -/
theorem okAdditionalUpkeep :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep (some .combat) (.lit 1) none) = [] := by
  decide

/-- "Spells with the chosen name can't be cast." -/
theorem okCastSpellClass :
    StaticSpec.check [qualityB .cardName]
      (objectCant (.action "cast") (allOf (.and [spell, .named .chosen])))
      = [] := by
  decide

/-- "Spells with the chosen name can't be activated." -/
theorem badActivatedSpellClass :
    StaticSpec.check [qualityB .cardName]
      (objectCant (.action "activate") (allOf (.and [spell, .named .chosen]))) = [.deedFits] := by
  decide

/-- "Activated abilities of artifacts can't be cast." -/
theorem badCastAbilityClass :
    StaticSpec.check (DRS.ofList [])
      (objectCant (.action "cast")
        (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf artifact)])))
      = [.deedFits] := by
  decide

/-- "Activated and triggered abilities can't be activated.": a disjunction seeds an ability
when every arm does, so the deed's ability role is met [CR#113.1c]. -/
theorem okAbilityDisjunctionActivated :
    StaticSpec.check (DRS.ofList [])
      (objectCant (.action "activate")
        (allOf (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered])))
      = [] := by
  decide

/-- "Activated abilities and creatures can't be activated.": one arm is no ability, so the
disjunction seeds none. -/
theorem badMixedDisjunctionActivated :
    StaticSpec.check (DRS.ofList [])
      (objectCant (.action "activate") (allOf (.or [.abilityHead .anyActivated, creature])))
      = [.deedFits] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okSingularPartPossessor :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw (.lit 1))) = [] := by
  decide

/-- "At the beginning of combat on your turn, draw a card." The beginning of combat step is
its own part of the combat phase [CR#506.1]. -/
theorem okBeginningOfCombatPossessor :
    Ability.check (DRS.ofList [])
      (at_ (.beginningOf .beginningOfCombat (some .you) (.each none)) (draw (.lit 1)))
      = [] := by
  decide

/-- "{2}: Draw a card. Activate only during each player's ending phase." The ending phase is the
last of the turn's five phases [CR#500.1]. -/
theorem okEndingPhaseWindow :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw (.lit 1))
        (some (duringPart .endingPhase (some (each .anyPlayer))))) = [] := by
  decide

/-- "After this postcombat main phase, there is an additional ending phase." [CR#500.1] -/
theorem okAdditionalEndingPhase :
    Action.check (DRS.ofList [])
      (Primitives.Action.addTurnPart .endingPhase (some .postcombatMain) (.lit 1) none) = [] := by
  decide

/-- "creature that could block each attacking creature" -/
theorem okCouldBlockAttacker :
    Predicate.checkAt .object (DRS.ofList []) (.inCombat .couldBlock (some (allOf (.and [creature, attacking]))))
      = [] := by
  decide

/-- "player an opponent is attacking": the attacker of an attacked player may be a player
[CR#506.2]. -/
theorem okAttackedByPlayer :
    Predicate.checkAt .player (DRS.ofList []) (.inCombat .attackedBy (some anOpponent)) = [] := by decide

/-- "Sacrifice a creature. If you do, draw a card." -/
theorem okIfDoneWithArm :
    Action.check (DRS.ofList []) (Primitives.Action.doIfDone (sacrifice (a creature)) (some (draw (.lit 1))) none)
      = [] := by
  decide

/-- "Sacrifice a creature." -/
theorem badIfDoneWithNeitherArm :
    Action.check (DRS.ofList []) (Primitives.Action.doIfDone (sacrifice (a creature)) none none) =
        [.ifDoneArmed] := by
  decide

/-- "This creature deals 3 damage to any target. If you do, draw a card." -/
theorem badIfDoneOverAgentlessBody :
    Action.check (DRS.ofList [])
      (Primitives.Action.doIfDone (Action.instruct thisCreature (.dealDamage (.lit 3) (target anyTarget) .any)) (some (draw (.lit 1)))
        none) = [.reflexEnclosure] := by
  decide

/-- "Take an extra turn after this one. If you do, draw a card." -/
theorem badIfDoneOverScheduledBody :
    Action.check (DRS.ofList []) (Primitives.Action.doIfDone (extraTurn (.lit 1)) (some (draw (.lit 1))) none)
      = [.reflexEnclosure] := by
  decide

def oneExtraTurn : DRS := instructionResult (DRS.ofList []) (extraTurn (.lit 1))

/-- "Take an extra turn after this one. Skip the draw step of that turn." -/
theorem okThatTurnAfterASingleTurn :
    TurnRef.check oneExtraTurn .that = [] := by decide

def twoExtraTurns : DRS := instructionResult oneExtraTurn (extraTurn (.lit 1))

theorem badThatTurnAfterTwoTurns :
    TurnRef.check twoExtraTurns .that = [.turnAntecedent 2] := by
  decide

/-- "create a legendary 20/20 black Avatar creature token named Marit Lage" -/
theorem okTokenSingleSupertype :
    Action.check (DRS.ofList [])
      (create (.lit 1)
        { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary],
            types := [.creature], subtypes := [creatureType "Avatar"],
            power := some (.lit 20), toughness := some (.lit 20) } }) = [] := by
  decide

/-- "create a legendary legendary 20/20 black Avatar creature token" -/
theorem badTokenDuplicateSupertype :
    Action.check (DRS.ofList [])
      (create (.lit 1)
        { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary, .legendary],
            types := [.creature], subtypes := [creatureType "Avatar"],
            power := some (.lit 20), toughness := some (.lit 20) } }) = [.distinct] := by
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
      [extraTurn (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.lit 1))]) =
          [] := by decide

theorem okExtraTurnNestedAmountIntroducesLetterOnce :
    countLetter .x (instructionResult (DRS.ofList [])
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
      [.skipPart .drawStep (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.lit 1))])
      = [] := by
  decide

theorem okSkipNextNestedAmountIntroducesLetterOnce :
    countLetter .x (instructionResult (DRS.ofList [])
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
          (.lit 1))])
      = [] := by
  decide

theorem okAdditionalPartNestedAmountIntroducesLetterOnce :
    countLetter .x (instructionResult (DRS.ofList [])
      (Primitives.Action.addTurnPart .upkeep none (plus (.letter .x) (.letter .x)) none))
      = 1 := by
  decide

theorem okUntapAmountReadsSubject :
    Action.check [qualityB .color]
      (.skipUntap (target creature) (powerOf it)) = [] := by decide

theorem untapAmountReadsItsPublishedSelfSubject :
    let before := DRS.ofList [qualityB .color]
    let selected := (stepNounPhrase 64 before .obj true thisCreature).1
    Action.check before (.skipUntap thisCreature (powerOf it)) = [] ∧
      (firstReach .bare .one selected).map (·.id) = some 1 := by decide

theorem okUntapNestedAmountIntroducesLetterOnce :
    countLetter .x (instructionResult (DRS.ofList [])
      (.skipUntap (target creature) (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okUntapAmountIntroducesPlayer :
    Action.check (DRS.ofList []) (.sequentially
      [.skipUntap (target creature) (lifeTotalOf (target (.isOpponentOf .you))), Templates.instruct they (.draw (.lit 1))])
      = [] := by decide

theorem okAdditionalPartWithoutSubjectReadsOuterContext :
    Action.check (nounResult (DRS.ofList []) (target (.isOpponentOf .you)))
      (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none) = [] := by decide

theorem badAdditionalPartWithoutSubjectOrOuterContext :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep none (lifeTotalOf they) none)
      = [.anaphor (.word .player) .one 0] := by decide

/-- Nested amounts preserve the identities and kinds of their introduced targets. -/
theorem okExtraTurnNestedAmountOrder :
    let after := instructionResult [qualityB .color]
      (extraTurn (plus (powerOf (target creature)) (lifeTotalOf (target (.isOpponentOf .you)))))
    after.visibleReferents.map (fun r => (r.id, r.kind)) =
      [(3, .turnRef), (2, .player), (1, .object), (0, .quality .color)] ∧
      (after.findById? 1).map (·.ty) = some [.creature] := by decide

theorem okSkipNextNestedAmountOrder :
    let after := instructionResult (DRS.ofList [])
      (.skipPart .drawStep (plus (powerOf (target creature)) (lifeTotalOf (target (.isOpponentOf .you)))))
    after.visibleReferents.map (fun r => (r.id, r.kind)) = [(1, .player), (0, .object)] ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

theorem okAdditionalPartNestedAmountOrder :
    let after := instructionResult (DRS.ofList [])
      (Primitives.Action.addTurnPart .upkeep none
        (plus (powerOf (target creature)) (lifeTotalOf (target (.isOpponentOf .you)))) none)
    after.visibleReferents.map (fun r => (r.id, r.kind)) = [(1, .player), (0, .object)] ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

theorem okUntapNestedAmountOrder :
    let after := instructionResult (DRS.ofList [])
      (.skipUntap .this (plus (powerOf (target creature)) (lifeTotalOf (target (.isOpponentOf .you)))))
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
theorem okFlashExpansion : Ability.check (DRS.ofList []) flashExpansion = [] := by decide

/-- Sneak {U}'s definition [CR#702.190a]: a cast permission at instant speed in your declare
blockers step, paying {U} and returning an unblocked creature you control. -/
theorem okSneakExpansion : Ability.check (DRS.ofList []) (sneakExpansion (.mana [pip .blue])) = [] := by decide

/-- Mystic Veil: "If you cast it any time a sorcery couldn't have been cast" [CR#307.5a], the
timing qualifier on a lookback. -/
theorem okCastWhenASorceryCouldNotHaveBeen :
    Condition.check (DRS.ofList [])
      (happened (.procedure .cast .this .itsOwnCost none none none) (.actor .you) thisGame (some (.not asSorcery)))
      = [] := by
  decide

/-- "Whenever you cast a spell, you may draw a card. Do this only once each turn." [CR#603.2h]:
the ability triggers every time and the instruction is limited (Iron Man, Bleeding Edge and
Donal, Herald of Wings; the retired `UsageLimit.actionOncePerTurn`). -/
theorem okDoThisOnlyOnceEachTurn :
    Ability.check (DRS.ofList []) (triggered (.does (.actor .you) (.procedure .cast (a spell) .itsOwnCost none none none))
      (doOnlyOnceEachTurn (offer (draw (.lit 1))))) = [] := by
  decide

/-- The limited instruction is checked as the instruction it wraps. -/
theorem limitedChecksItsInstruction :
    Action.check (DRS.ofList []) (.limited (move it hand) oncePerTurn) =
      Action.check (DRS.ofList []) (move it hand) := by
  decide

end Semantics.Proofs.Turn
