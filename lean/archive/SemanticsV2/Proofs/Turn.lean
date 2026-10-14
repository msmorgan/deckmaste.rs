import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Turn

Port of `idris/src/Experimental/Proofs/Turn.idr`: the pins of the Turn family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Turn

/-- An activated ability with no limit, guard, or activator. -/
def act (cost : Cost) (instruction : Instruction) (timing : Option Timing := none) : Ability :=
  .activated cost instruction timing none none none

/-- "at the beginning of the end step" -/
def atTheEndStep : GameEvent := .beginningOf .endStep none (.each none)

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefield : Instruction.check [] (sacrifice (a creature)) = []
    := by
  decide

/-- The Idris pin refutes the "other" anchor; the targeting law the delayed clause loses is
listed too. -/
theorem badDelayedOther :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 2) (target anyTarget),
          delay atTheEndStep (.dealDamage .this (.lit 1) (target anyOtherTarget)) ])
      = [.anyTargeted (.join .object (.join .object (.join .object .player))), .otherAnchored] := by
  decide

theorem badStaleCarrier :
    Instruction.check []
      (.sequentially [exile (target creatureYouControl), move (that (.type .creature)) battlefield])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okActivatedCostAndEffect :
    Ability.check [] (act (.perform (sacrifice (a creature))) (draw (.lit 1))) = [] := by
  decide

/-- "Return a creature to its owner's hand: Tap it." The Idris pin refutes the anaphor. -/
theorem badHiddenCost :
    Ability.check [] (act (.perform (move (a creature) hand)) (.setStatus .tapped it))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Discard a card, Sacrifice a creature: Exile it." -/
theorem badTwoCostMentions :
    Ability.check []
      (act
        (.compound
          [.perform (discard (a (.and [.isCard, .inZone yourHand]))), .perform (sacrifice (a creature))])
        (exile it)) = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature you control. Sacrifice it." -/
theorem okSacrificeOnBattlefield :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creatureYouControl), sacrifice it])
      = [] := by
  decide

theorem badDeadCreatureRead :
    Instruction.check []
      (.delay (Primitives.GameEvent.dies (target creature)) [] (some thisTurn)
        (move (that (.type .creature)) battlefield))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Discard a card: Return the discarded card to the battlefield." -/
theorem okVerbedDiscardedCard :
    Ability.check []
      (act (.perform (discard (a (.and [.isCard, .inZone yourHand]))))
        (move (theVerbed (.action "discard") .one (some .card)) battlefield)) = [] := by
  decide

/-- "Discard a card: Return the sacrificed card to the battlefield." -/
theorem badVerbedWrongVerb :
    Ability.check []
      (act (.perform (discard (a (.and [.isCard, .inZone yourHand]))))
        (move (theVerbed (.action "sacrifice") .one (some .card)) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some .card) .attributive) .one 0] := by
  decide

/-- "Sacrifice an artifact: Return the sacrificed creature to the battlefield." -/
theorem badVerbedWrongNoun :
    Ability.check []
      (act (.perform (sacrifice (a artifact)))
        (move (theVerbed (.action "sacrifice") .one (some (.type .creature))) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some (.type .creature)) .attributive) .one 0] := by
  decide

theorem badVerbedAmbig :
    Ability.check []
      (act
        (.compound [.perform (sacrifice (a creature)), .perform (sacrifice (a
            creature))])
        (move (theVerbed (.action "sacrifice") .one (some .card)) battlefield))
      = [.anaphor (.verbed (.action "sacrifice") (some .card) .attributive) .one 2] := by
  decide

theorem badBareCardRead :
    Ability.check []
      (act (.perform (sacrifice (a creature)))
        (.sequentially
          [exile (target creature), delay atTheEndStep (move (that .card) battlefield)]))
      = [.anaphor (.word .card) .one 2] := by
  decide

/-- "Tap target creature." -/
theorem okTapOnBattlefield : Instruction.check [] (.setStatus .tapped (target creature)) = [] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okTriggerAtYourUpkeep :
    Ability.check []
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw (.lit 1))) = [] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okUntilEndOfTurnDuration :
    Instruction.check []
      (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Sacrifice a creature. When you do, draw a card." -/
theorem okReflexiveOnSacrifice :
    Instruction.check [] (.triggerReflexively (sacrifice (a creature)) (draw (.lit
        1)))
      = [] := by
  decide

/-- "This creature deals 3 damage to any target. When you do, draw a card." -/
theorem badReflexiveOnSourceDeed :
    Instruction.check []
      (.triggerReflexively (.dealDamage .this (.lit 3) (target anyTarget)) (draw (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "You gain 2 life. When you do, draw a card." -/
theorem badReflexiveOnLifeGain :
    Instruction.check [] (.triggerReflexively (gainLife (.lit 2)) (draw (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "Draw a card, then sacrifice a creature. When you do, draw a card." -/
theorem badReflexiveOnSequence :
    Instruction.check []
      (.triggerReflexively (.sequentially [draw (.lit 1), sacrifice (a creature)])
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

theorem badReflexiveOnDelayed :
    Instruction.check []
      (.triggerReflexively (delay (.beginningOf .endStep (some .you) (.next none)) (draw (.lit 1)))
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- "Regenerate this creature. If it regenerates this way, draw a card." -/
theorem okThisWayOnRegenerate :
    Instruction.check []
      (.triggerThisWay (regenerate thisCreature) (regenerates thisCreature) (draw (.lit 1)))
      = [] := by
  decide

theorem badThisWayOnDelayed :
    Instruction.check []
      (.triggerThisWay (delay (.beginningOf .endStep (some .you) (.next none)) (draw (.lit 1)))
        (.draws .you) (draw (.lit 1))) = [.thisWayOutcome] := by
  decide

theorem badReflexiveOnBranchedMay :
    Instruction.check []
      (.triggerReflexively (Primitives.Instruction.offer (sacrifice (a creature)) (some (draw (.lit 1))) none)
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- The Idris pin refutes the anaphor. -/
theorem badAfterReflexiveReadsTrigger :
    Instruction.check []
      (.sequentially
        [ .triggerReflexively (mill (.lit 4) .you)
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          .setStatus .tapped (that (.type .creature)) ])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "target creature blocking this creature" -/
theorem okBlockingThisCreature :
    NounPhrase.check (some .object) []
      (target (.and [creature, .inCombat .blockerOf (some thisCreature)])) = [] := by
  decide

/-- "Activate only during each player's end step." -/
theorem okDistributivePartWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1))
        (some (duringPart .endStep (some (each .anyPlayer))))) = [] := by
  decide

/-- "until the beginning of your next upkeep" -/
theorem okDurationEndYourUpkeep :
    Duration.check [] (untilEvent (.beginningOf .upkeep (some .you) (.next none))) = [] := by decide

/-- "until the beginning of each player's next upkeep" -/
theorem badDurationEndEachPlayers :
    Duration.check [] (untilEvent (.beginningOf .upkeep (some (each .anyPlayer)) (.next none)))
      = [.durationPossessor] := by
  decide

/-- "until the end of your next combat" -/
theorem okDurationEndYourCombat :
    Duration.check [] (untilEvent (.endOf .combat (some .you) (.next none))) = [] := by decide

/-- "until the end of an opponent's combat" -/
theorem badDurationEndAnOpponent :
    Duration.check [] (untilEvent (.endOf .combat (some anOpponent) (.next none)))
      = [.durationPossessor] := by decide

/-- "You get an emblem with 'At the beginning of your end step, draw a card.'" -/
theorem okTriggeredEmblem :
    Instruction.check []
      (Primitives.Instruction.getEmblem
        [at_ (.beginningOf .endStep (some .you) (.each none)) (draw (.lit 1))])
      = [] := by
  decide

/-- "You get an emblem." -/
theorem badEmptyEmblem :
    Instruction.check [] (Primitives.Instruction.getEmblem []) = [.nonEmpty] := by decide

/-- "… At the beginning of that turn's end step, you lose the game." -/
theorem okDeicticTurnAfterExtraTurn :
    Instruction.check []
      (.sequentially
        [ extraTurn (.lit 1),
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [] := by
  decide

/-- "Draw a card. At the beginning of that turn's end step, you lose the game." -/
theorem badDeicticTurnWithoutIntroducer :
    Instruction.check []
      (.sequentially
        [ draw (.lit 1),
          delay (.beginningOf .endStep none .that) (.conclude .loseGame) ])
      = [.turnAntecedent 0] := by
  decide

/-- "After this combat phase, there is an additional upkeep step." -/
theorem okAdditionalUpkeep :
    Instruction.check [] (Primitives.Instruction.addTurnPart .upkeep (some .combat) (.lit 1) none) = [] := by
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
    StaticSpec.check []
      (objectCant (.action "cast")
        (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf artifact)])))
      = [.deedFits] := by
  decide

/-- "Activated and triggered abilities can't be activated.": a disjunction seeds an ability
when every arm does, so the deed's ability role is met [CR#113.1c]. -/
theorem okAbilityDisjunctionActivated :
    StaticSpec.check []
      (objectCant (.action "activate")
        (allOf (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered])))
      = [] := by
  decide

/-- "Activated abilities and creatures can't be activated.": one arm is no ability, so the
disjunction seeds none. -/
theorem badMixedDisjunctionActivated :
    StaticSpec.check []
      (objectCant (.action "activate") (allOf (.or [.abilityHead .anyActivated, creature])))
      = [.deedFits] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okSingularPartPossessor :
    Ability.check []
      (at_ (.beginningOf .upkeep (some .you) (.each none)) (draw (.lit 1))) = [] := by
  decide

/-- "At the beginning of combat on your turn, draw a card." The beginning of combat step is
its own part of the combat phase [CR#506.1]. -/
theorem okBeginningOfCombatPossessor :
    Ability.check []
      (at_ (.beginningOf .beginningOfCombat (some .you) (.each none)) (draw (.lit 1)))
      = [] := by
  decide

/-- "{2}: Draw a card. Activate only during each player's ending phase." The ending phase is the
last of the turn's five phases [CR#500.1]. -/
theorem okEndingPhaseWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1))
        (some (duringPart .endingPhase (some (each .anyPlayer))))) = [] := by
  decide

/-- "After this postcombat main phase, there is an additional ending phase." [CR#500.1] -/
theorem okAdditionalEndingPhase :
    Instruction.check []
      (Primitives.Instruction.addTurnPart .endingPhase (some .postcombatMain) (.lit 1) none) = [] := by
  decide

/-- "creature that could block each attacking creature" -/
theorem okCouldBlockAttacker :
    Predicate.check .object [] (.inCombat .couldBlock (some (allOf (.and [creature, attacking]))))
      = [] := by
  decide

/-- "player an opponent is attacking": the attacker of an attacked player may be a player
[CR#506.2]. -/
theorem okAttackedByPlayer :
    Predicate.check .player [] (.inCombat .attackedBy (some anOpponent)) = [] := by decide

/-- "Sacrifice a creature. If you do, draw a card." -/
theorem okIfDoneWithArm :
    Instruction.check [] (Primitives.Instruction.doIfDone (sacrifice (a creature)) (some (draw (.lit 1))) none)
      = [] := by
  decide

/-- "Sacrifice a creature." -/
theorem badIfDoneWithNeitherArm :
    Instruction.check [] (Primitives.Instruction.doIfDone (sacrifice (a creature)) none none) =
        [.ifDoneArmed] := by
  decide

/-- "This creature deals 3 damage to any target. If you do, draw a card." -/
theorem badIfDoneOverAgentlessBody :
    Instruction.check []
      (Primitives.Instruction.doIfDone (.dealDamage thisCreature (.lit 3) (target anyTarget)) (some (draw (.lit 1)))
        none) = [.reflexEnclosure] := by
  decide

/-- "Take an extra turn after this one. If you do, draw a card." -/
theorem badIfDoneOverScheduledBody :
    Instruction.check [] (Primitives.Instruction.doIfDone (extraTurn (.lit 1)) (some (draw (.lit 1))) none)
      = [.reflexEnclosure] := by
  decide

def oneExtraTurn : Bindings := Instruction.intro [] (extraTurn (.lit 1))

/-- "Take an extra turn after this one. Skip the draw step of that turn." -/
theorem okThatTurnAfterASingleTurn :
    TurnRef.check oneExtraTurn .that = [] := by decide

def twoExtraTurns : Bindings := Instruction.intro oneExtraTurn (extraTurn (.lit 1))

theorem badThatTurnAfterTwoTurns :
    TurnRef.check twoExtraTurns .that = [.turnAntecedent 2] := by
  decide

/-- "create a legendary 20/20 black Avatar creature token named Marit Lage" -/
theorem okTokenSingleSupertype :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary],
            types := [.creature], subtypes := [creatureType "Avatar"],
            power := some (.lit 20), toughness := some (.lit 20) } }) = [] := by
  decide

/-- "create a legendary legendary 20/20 black Avatar creature token" -/
theorem badTokenDuplicateSupertype :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { name := some "Marit Lage", colors := [.black], supertypes := [.legendary, .legendary],
            types := [.creature], subtypes := [creatureType "Avatar"],
            power := some (.lit 20), toughness := some (.lit 20) } }) = [.distinct] := by
  decide

/-! ## Constructor fields share the preceding discourse -/

theorem okExtraTurnAmountReadsSubject :
    Instruction.check [qualityB .color]
      (Macros.act (target .opponent) (extraTurn (lifeTotalOf they))) = [] := by decide

theorem badExtraTurnAmountReadsSubject :
    Instruction.check [qualityB .color]
      (extraTurn (lifeTotalOf they)) = [.anaphor (.word .player) .one 0] := by decide

theorem badExtraTurnForwardSubject :
    Instruction.check [] (Macros.act they (extraTurn (lifeTotalOf (target .opponent))))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okExtraTurnAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [extraTurn (lifeTotalOf (target .opponent)), Macros.act they (.draw (.lit 1))]) =
          [] := by decide

theorem okExtraTurnNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (extraTurn (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okSkipNextAmountReadsSubject :
    Instruction.check [qualityB .color]
      (Macros.act (target .opponent) (.skipPart .drawStep (lifeTotalOf they))) = [] := by decide

theorem badSkipNextAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.skipPart .drawStep (lifeTotalOf they))
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badSkipNextForwardSubject :
    Instruction.check [] (Macros.act they (.skipPart .drawStep (lifeTotalOf (target .opponent))))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okSkipNextAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [.skipPart .drawStep (lifeTotalOf (target .opponent)), Macros.act they (.draw (.lit 1))])
      = [] := by
  decide

theorem okSkipNextNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (.skipPart .drawStep (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okAdditionalPartAmountReadsSubject :
    Instruction.check [qualityB .color]
      (Macros.act (target .opponent) (Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf they) none))
      = [] := by
  decide

theorem badAdditionalPartAmountReadsSubject :
    Instruction.check [qualityB .color]
      (Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf they) none)
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badAdditionalPartForwardSubject :
    Instruction.check [] (Macros.act they (Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf (target .opponent)) none))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okAdditionalPartAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf (target .opponent)) none, Macros.act they (.draw
          (.lit 1))])
      = [] := by
  decide

theorem okAdditionalPartNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (Primitives.Instruction.addTurnPart .upkeep none (plus (.letter .x) (.letter .x)) none))
      = 1 := by
  decide

theorem okUntapAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.skipUntap (target creature) (powerOf it)) = [] := by decide

theorem badUntapAmountWithoutSubjectMention :
    Instruction.check [qualityB .color]
      (.skipUntap thisCreature (powerOf it)) = [.anaphor .bare .one 0] := by decide

theorem okUntapNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (.skipUntap (target creature) (plus (.letter .x) (.letter .x)))) = 1 := by decide

theorem okUntapAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [.skipUntap (target creature) (lifeTotalOf (target .opponent)), Macros.act they (.draw (.lit 1))])
      = [] := by decide

theorem okAdditionalPartWithoutSubjectReadsOuterContext :
    Instruction.check (nomIntro [] (target .opponent))
      (Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf they) none) = [] := by decide

theorem badAdditionalPartWithoutSubjectOrOuterContext :
    Instruction.check [] (Primitives.Instruction.addTurnPart .upkeep none (lifeTotalOf they) none)
      = [.anaphor (.word .player) .one 0] := by decide

/-- Nested amounts leave their most recent mention first, followed by the outer context. -/
theorem okExtraTurnNestedAmountOrder :
    (Instruction.intro [qualityB .color]
      (extraTurn (plus (powerOf (target creature)) (lifeTotalOf (target .opponent)))))
      = [turnRefB, ⟨.target, .one, .player false, []⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1), []⟩,
         qualityB .color] := by rfl

theorem okSkipNextNestedAmountOrder :
    (Instruction.intro []
      (.skipPart .drawStep
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent)))))
      = [⟨.target, .one, .player false, []⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1), []⟩] := by rfl

theorem okAdditionalPartNestedAmountOrder :
    (Instruction.intro []
      (Primitives.Instruction.addTurnPart .upkeep none
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent))) none))
      = [⟨.target, .one, .player false, []⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1), []⟩] := by rfl

theorem okUntapNestedAmountOrder :
    (Instruction.intro []
      (.skipUntap .this
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent)))))
      = [⟨.target, .one, .player false, []⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1), []⟩] := by rfl

/-! ## The temporal sorts (`semantics-v2-temporal-suite`)

A timing, a duration and a usage limit are checked for references and kinds only (§7 ruling
2026-10-08, "Time is three sorts"). -/

/-- "Activate only during this creature's upkeep": whose turn it is must be a player's. -/
theorem badTimingWhoseNotAPlayer :
    Timing.check [] (duringPart .upkeep (some thisCreature)) = [.kindMismatch .player .object] := by
  decide

/-- "Activate only during that turn's end step", with no turn introduced before it. -/
theorem badTimingThatTurnWithoutAntecedent :
    Timing.check [] (.inPart .endStep none .that) = [.turnAntecedent 0] := by decide

/-- "… during that turn's end step" after "take an extra turn after this one". -/
theorem okTimingThatTurnAfterExtraTurn :
    Timing.check oneExtraTurn (.inPart .endStep none .that) = [] := by decide

/-- "Activate only as a sorcery" [CR#307.5] and "only as an instant" [CR#117.1a]. -/
theorem okSorceryAndInstantSpeed :
    Timing.check [] asSorcery ++ Timing.check [] asInstant = [] := by decide

/-- "Until your next turn": the end is the beginning of your next turn. -/
theorem okUntilYourNextTurn : Duration.check [] untilYourNextTurn = [] := by decide

/-- "During your next turn": both boundaries are your next turn's. -/
theorem okDuringYourNextTurn : Duration.check [] (duringNextTurnOf .you) = [] := by decide

/-- "On a later turn" [CR#702.170d] and the history windows. -/
theorem okLaterTurnAndHistoryWindows :
    Duration.check [] onALaterTurn ++ Duration.check [] soFarThisTurn ++
      Duration.check [] lastTurn ++ Duration.check [] thisGame = [] := by decide

/-- "Activate only once each turn" [CR#602.5b]. -/
theorem okOncePerTurn : UsageLimit.check [] oncePerTurn = [] := by decide

/-- "the next turn of this creature": a turn reference names a player's turns. -/
theorem badNextTurnOfACreature :
    TurnRef.check [] (.next (some thisCreature)) = [.kindMismatch .player .object] := by decide

/-- Flash's definition [CR#702.8a]: a play permission at instant speed. -/
theorem okFlashExpansion : Ability.check [] flashExpansion = [] := by decide

/-- Sneak {U}'s definition [CR#702.190a]: a cast permission at instant speed in your declare
blockers step, paying {U} and returning an unblocked creature you control. -/
theorem okSneakExpansion : Ability.check [] (sneakExpansion (.mana [pip .blue])) = [] := by decide

/-- Mystic Veil: "If you cast it any time a sorcery couldn't have been cast" [CR#307.5a], the
timing qualifier on a lookback. -/
theorem okCastWhenASorceryCouldNotHaveBeen :
    Condition.check []
      (happened (.casts (relative .player) (some .this) none) .you thisGame (some (.not asSorcery)))
      = [] := by
  decide

/-- "Whenever you cast a spell, you may draw a card. Do this only once each turn." [CR#603.2h]:
the ability triggers every time and the instruction is limited (Iron Man, Bleeding Edge and
Donal, Herald of Wings; the retired `UsageLimit.actionOncePerTurn`). -/
theorem okDoThisOnlyOnceEachTurn :
    Ability.check [] (triggered (.casts .you (some (a spell)) none)
      (doOnlyOnceEachTurn (offer (draw (.lit 1))))) = [] := by
  decide

/-- The limited instruction is checked as the instruction it wraps. -/
theorem limitedChecksItsInstruction :
    Instruction.check [] (.limited (move it hand) oncePerTurn) =
      Instruction.check [] (move it hand) := by
  decide

end Semantics.Proofs.Turn
