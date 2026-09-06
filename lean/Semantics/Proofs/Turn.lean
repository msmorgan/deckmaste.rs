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
def atTheEndStep : GameEvent := .beginningOf .the .endStep .noPossessor

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefield : Instruction.check [] (sacrifice (a creature) (agent := .you)) = []
    := by
  decide

/-- "Destroy target creature. At the beginning of the end step, sacrifice it." -/
theorem badStale :
    Instruction.check []
      (.sequentially [destroy (target creature), delay atTheEndStep (sacrifice it (agent := .you))])
      = [.zoneFits] := by
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
    Ability.check [] (act (.perform (sacrifice (a creature) (agent := .you))) (draw (.lit 1) (agent
        := .you))) = [] := by
  decide

/-- "Return a creature to its owner's hand: Tap it." The Idris pin refutes the anaphor; the
unresolved `it` has no zone either. -/
theorem badHiddenCost :
    Ability.check [] (act (.perform (move (a creature) hand)) (.setStatus .tapped it))
      = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

/-- "Discard a card, Sacrifice a creature: Exile it." -/
theorem badTwoCostMentions :
    Ability.check []
      (act
        (.compound
          [.perform (discard (a (.inZone hand)) (agent := .you)), .perform (sacrifice (a creature)
              (agent := .you))])
        (exile it)) = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature you control. Sacrifice it." -/
theorem okSacrificeOnBattlefield :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creatureYouControl), sacrifice it (agent := .you)])
      = [] := by
  decide

/-- "Exile target creature. Sacrifice it." -/
theorem badSacrificeExiled :
    Instruction.check [] (.sequentially [exile (target creature), sacrifice it (agent := .you)])
      = [.zoneFits] := by
  decide

theorem badDeadCreatureRead :
    Instruction.check []
      (.delay (.dies (target creature)) [] (some .thisTurn)
        (move (that (.type .creature)) battlefield))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Discard a card: Return the discarded card to the battlefield." -/
theorem okVerbedDiscardedCard :
    Ability.check []
      (act (.perform (discard (a (.inZone hand)) (agent := .you)))
        (move (theVerbed (.action "Discard") .card .attributive .one) battlefield)) = [] := by
  decide

/-- "Discard a card: Return the sacrificed card to the battlefield." -/
theorem badVerbedWrongVerb :
    Ability.check []
      (act (.perform (discard (a (.inZone hand)) (agent := .you)))
        (move (theVerbed (.action "Sacrifice") .card .attributive .one) battlefield))
      = [.anaphor (.verbed (.action "Sacrifice") .card .attributive) .one 0] := by
  decide

/-- "Sacrifice an artifact: Return the sacrificed creature to the battlefield." -/
theorem badVerbedWrongNoun :
    Ability.check []
      (act (.perform (sacrifice (a artifact) (agent := .you)))
        (move (theVerbed (.action "Sacrifice") (.type .creature) .attributive .one) battlefield))
      = [.anaphor (.verbed (.action "Sacrifice") (.type .creature) .attributive) .one 0] := by
  decide

theorem badVerbedAmbig :
    Ability.check []
      (act
        (.compound [.perform (sacrifice (a creature) (agent := .you)), .perform (sacrifice (a
            creature) (agent := .you))])
        (move (theVerbed (.action "Sacrifice") .card .attributive .one) battlefield))
      = [.anaphor (.verbed (.action "Sacrifice") .card .attributive) .one 2] := by
  decide

theorem badBareCardRead :
    Ability.check []
      (act (.perform (sacrifice (a creature) (agent := .you)))
        (.sequentially
          [exile (target creature), delay atTheEndStep (move (that .card) battlefield)]))
      = [.anaphor (.word .card) .one 2] := by
  decide

/-- "Tap target creature." -/
theorem okTapOnBattlefield : Instruction.check [] (.setStatus .tapped (target creature)) = [] := by
  decide

/-- "Tap target creature card in your graveyard." -/
theorem badTapGraveyard :
    Instruction.check []
      (.setStatus .tapped (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okTriggerAtYourUpkeep :
    Ability.check []
      (at_ (.beginningOf .the .upkeep (.byPlayer .you)) (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "At the beginning of your turn, draw a card." -/
theorem badTriggerAtYourTurn :
    Ability.check []
      (at_ (.beginningOf .the .turn (.byPlayer .you)) (draw (.lit 1) (agent := .you)))
      = [.windowOk] := by
  decide

/-- "Whenever a creature dies, tap it." -/
theorem badTriggerTapsDeadCreature :
    Ability.check [] (whenever (.dies (a creature)) (.setStatus .tapped it))
      = [.zoneIs .battlefield] := by
  decide

/-- "Whenever a creature leaves the battlefield, tap it." -/
theorem badLeavesThenTap :
    Ability.check [] (whenever (leavesBattlefield (a creature)) (.setStatus .tapped it))
      = [.zoneIs .battlefield] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okUntilEndOfTurnDuration :
    Instruction.check []
      (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Target creature gets +3/+3 until the beginning of your next upkeep." -/
theorem badUntilBeginningOfUpkeep :
    Instruction.check []
      (get (target creature) (.up (.lit 3)) (.up (.lit 3))
        (some (.untilEvent (.beginningOf .the .upkeep (.byPlayer .you))))) = [.durationOk] := by
  decide

/-- "Sacrifice a creature. When you do, draw a card." -/
theorem okReflexiveOnSacrifice :
    Instruction.check [] (.triggerReflexively (sacrifice (a creature) (agent := .you)) (draw (.lit
        1) (agent := .you)))
      = [] := by
  decide

/-- "This creature deals 3 damage to any target. When you do, draw a card." -/
theorem badReflexiveOnSourceDeed :
    Instruction.check []
      (.triggerReflexively (.dealDamage .this (.lit 3) (target anyTarget)) (draw (.lit 1) (agent :=
          .you)))
      = [.reflexEnclosure] := by
  decide

/-- "You gain 2 life. When you do, draw a card." -/
theorem badReflexiveOnLifeGain :
    Instruction.check [] (.triggerReflexively (gainLife (.lit 2) (agent := .you)) (draw (.lit 1)
        (agent := .you)))
      = [.reflexEnclosure] := by
  decide

/-- "Draw a card, then sacrifice a creature. When you do, draw a card." -/
theorem badReflexiveOnSequence :
    Instruction.check []
      (.triggerReflexively (.sequentially [draw (.lit 1) (agent := .you), sacrifice (a creature) (agent
          := .you)])
        (draw (.lit 1) (agent := .you))) = [.reflexEnclosure] := by
  decide

theorem badReflexiveOnDelayed :
    Instruction.check []
      (.triggerReflexively (delay (.beginningOf .the .endStep (.byPlayer .you)) (draw (.lit 1)
          (agent := .you)))
        (draw (.lit 1) (agent := .you))) = [.reflexEnclosure] := by
  decide

/-- "Regenerate this creature. If it regenerates this way, draw a card." -/
theorem okThisWayOnRegenerate :
    Instruction.check []
      (.triggerThisWay (regenerate thisCreature) (regenerates thisCreature) (draw (.lit 1) (agent :=
          .you)))
      = [] := by
  decide

theorem badThisWayOnDelayed :
    Instruction.check []
      (.triggerThisWay (delay (.beginningOf .the .endStep (.byPlayer .you)) (draw (.lit 1) (agent :=
          .you)))
        (.draws .you) (draw (.lit 1) (agent := .you))) = [.thisWayOutcome] := by
  decide

theorem badReflexiveOnBranchedMay :
    Instruction.check []
      (.triggerReflexively (Primitives.Instruction.offer (sacrifice (a creature) (agent := .you)) (some (draw (.lit 1)
          (agent := .you))) none (agent := .you))
        (draw (.lit 1) (agent := .you))) = [.reflexEnclosure] := by
  decide

/-- The Idris pin refutes the anaphor; the unresolved "that creature" has no zone either. -/
theorem badAfterReflexiveReadsTrigger :
    Instruction.check []
      (.sequentially
        [ .triggerReflexively (mill (.lit 4) .you (agent := .you))
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          .setStatus .tapped (that (.type .creature)) ])
      = [.anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield] := by
  decide

/-- "Sacrifice a creature. When you do, tap it." -/
theorem badReflexiveTapsSacrificed :
    Instruction.check [] (.triggerReflexively (sacrifice (a creature) (agent := .you)) (.setStatus
        .tapped it))
      = [.zoneIs .battlefield] := by
  decide

/-- "target creature blocking this creature" -/
theorem okBlockingThisCreature :
    NounPhrase.check (some .object) []
      (target (.and [creature, .inCombat .blockerOf (some thisCreature)])) = [] := by
  decide

/-- "target creature blocking target creature card in your graveyard" -/
theorem badBlockingGraveyardRelatum :
    NounPhrase.check (some .object) []
      (target
        (.and
          [ creature,
            .inCombat .blockerOf
              (some (target (.and [creature, .inZone (graveyardOf .you)]))) ]))
      = [.combatRelOk] := by
  decide

/-- "Activate only during each player's end step." -/
theorem okDistributivePartWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1) (agent := .you))
        (some (.duringPart .endStep (some (each .anyPlayer))))) = [] := by
  decide

/-- "{2}: Draw a card. Activate only during all players' end step." [CR#102.1] -/
theorem badPluralPartWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1) (agent := .you))
        (some (.duringPart .endStep (some (allOf .anyPlayer))))) = [.windowOk] := by
  decide

/-- "until the beginning of your next upkeep" -/
theorem okDurationEndYourUpkeep :
    DurationEnd.check [] (.startOf .upkeep (some .you)) = [] := by decide

/-- "until the beginning of each player's next upkeep" -/
theorem badDurationEndEachPlayers :
    DurationEnd.check [] (.startOf .upkeep (some (each .anyPlayer)))
      = [.durationPossessor] := by
  decide

/-- "until the end of your next combat" -/
theorem okDurationEndYourCombat :
    DurationEnd.check [] (.endOf .combat (some .you)) = [] := by decide

/-- "until the end of an opponent's combat" -/
theorem badDurationEndAnOpponent :
    DurationEnd.check [] (.endOf .combat (some anOpponent)) = [.durationPossessor] := by decide

/-- "You get an emblem with 'At the beginning of your end step, draw a card.'" -/
theorem okTriggeredEmblem :
    Instruction.check []
      (.getEmblem
        [at_ (.beginningOf .the .endStep (.byPlayer .you)) (draw (.lit 1) (agent := .you))] (agent
            := .you))
      = [] := by
  decide

/-- "You get an emblem with 'flying'." -/
theorem badKeywordEmblem :
    Instruction.check [] (.getEmblem [keyword "Flying"] (agent := .you)) = [.emblemAbilities] := by
        decide

/-- "You get an emblem." -/
theorem badEmptyEmblem :
    Instruction.check [] (.getEmblem [] (agent := .you)) = [.emblemAbilities] := by decide

/-- "… At the beginning of that turn's end step, you lose the game." -/
theorem okDeicticTurnAfterExtraTurn :
    Instruction.check []
      (.sequentially
        [ .addTurn (.lit 1) (agent := .you),
          delay (.beginningOf .the .endStep (.byTurn thatTurn)) (.conclude .loseGame (agent :=
              .you)) ])
      = [] := by
  decide

/-- "Draw a card. At the beginning of that turn's end step, you lose the game." -/
theorem badDeicticTurnWithoutIntroducer :
    Instruction.check []
      (.sequentially
        [ draw (.lit 1) (agent := .you),
          delay (.beginningOf .the .endStep (.byTurn thatTurn)) (.conclude .loseGame (agent :=
              .you)) ])
      = [.anaphor .thatTurn .one 0] := by
  decide

/-- "After this combat phase, there is an additional upkeep step." -/
theorem okAdditionalUpkeep :
    Instruction.check [] (.addPart .upkeep (some .combat) (.lit 1) none (agent := none)) = [] := by
  decide

/-- "After this combat phase, there is an additional turn." -/
theorem badAdditionalTurn :
    Instruction.check [] (.addPart .turn (some .combat) (.lit 1) none (agent := none))
      = [.windowOk] := by
  decide

/-- "Spells with the chosen name can't be cast." -/
theorem okCastSpellClass :
    StaticSpec.check [qualityB .cardName]
      (objectCant (.action "Cast") (allOf (.and [spell, .named .chosen])))
      = [] := by
  decide

/-- "Spells with the chosen name can't be activated." -/
theorem badActivatedSpellClass :
    StaticSpec.check [qualityB .cardName]
      (objectCant (.action "Activate") (allOf (.and [spell, .named .chosen]))) = [.deedFits] := by
  decide

/-- "Activated abilities of artifacts can't be cast." -/
theorem badCastAbilityClass :
    StaticSpec.check []
      (objectCant (.action "Cast")
        (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf artifact)])))
      = [.deedFits] := by
  decide

/-- "Activated and triggered abilities can't be activated.": a disjunction seeds an ability
when every arm does, so the deed's ability role is met [CR#113.1c]. -/
theorem okAbilityDisjunctionActivated :
    StaticSpec.check []
      (objectCant (.action "Activate")
        (allOf (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered])))
      = [] := by
  decide

/-- "Activated abilities and creatures can't be activated.": one arm is no ability, so the
disjunction seeds none, and the arms do not parallel each other either. -/
theorem badMixedDisjunctionActivated :
    StaticSpec.check []
      (objectCant (.action "Activate") (allOf (.or [.abilityHead .anyActivated, creature])))
      = [.parallelDisjuncts, .deedFits] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okSingularPartPossessor :
    Ability.check []
      (at_ (.beginningOf .the .upkeep (.byPlayer .you)) (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "At the beginning of all players' upkeep, draw a card." [CR#102.1] -/
theorem badPluralPartPossessor :
    Ability.check []
      (at_ (.beginningOf .the .upkeep (.byPlayer (allOf .anyPlayer)))
        (draw (.lit 1) (agent := .you))) = [.windowOk] := by
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

/-- "player attacked by target creature card in your graveyard" -/
theorem badAttackedByGraveyardRelatum :
    Predicate.check .player []
      (.inCombat .attackedBy (some (target (.and [creature, .inZone (graveyardOf .you)]))))
      = [.combatRelOk] := by
  decide

/-- "creature that could block target creature card in your graveyard" -/
theorem badCouldBlockGraveyardRelatum :
    Predicate.check .object []
      (.inCombat .couldBlock (some (target (.and [creature, .inZone (graveyardOf .you)]))))
      = [.combatRelOk] := by
  decide

/-- "Sacrifice a creature. If you do, draw a card." -/
theorem okIfDoneWithArm :
    Instruction.check [] (Primitives.Instruction.doIfDone (sacrifice (a creature) (agent := .you)) (some (draw (.lit 1)
        (agent := .you))) none)
      = [] := by
  decide

/-- "Sacrifice a creature." -/
theorem badIfDoneWithNeitherArm :
    Instruction.check [] (Primitives.Instruction.doIfDone (sacrifice (a creature) (agent := .you)) none none) =
        [.ifDoneArmed] := by
  decide

/-- "This creature deals 3 damage to any target. If you do, draw a card." -/
theorem badIfDoneOverAgentlessBody :
    Instruction.check []
      (Primitives.Instruction.doIfDone (.dealDamage thisCreature (.lit 3) (target anyTarget)) (some (draw (.lit 1) (agent
          := .you)))
        none) = [.reflexEnclosure] := by
  decide

/-- "Take an extra turn after this one. If you do, draw a card." -/
theorem badIfDoneOverScheduledBody :
    Instruction.check [] (Primitives.Instruction.doIfDone (.addTurn (.lit 1) (agent := .you)) (some (draw (.lit 1) (agent
        := .you))) none)
      = [.reflexEnclosure] := by
  decide

def oneExtraTurn : Bindings := Instruction.intro [] (.addTurn (.lit 1) (agent := .you))

/-- "Take an extra turn after this one. Skip the draw step of that turn." -/
theorem okThatTurnAfterASingleTurn :
    NounPhrase.check (some .turnRef) oneExtraTurn thatTurn = [] := by decide

def twoExtraTurns : Bindings := Instruction.intro oneExtraTurn (.addTurn (.lit 1) (agent := .you))

theorem badThatTurnAfterTwoTurns :
    NounPhrase.check (some .turnRef) twoExtraTurns thatTurn = [.anaphor .thatTurn .one 2] := by
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
            power := some (.lit 20), toughness := some (.lit 20) } }) = [.tokenCanonical] := by
  decide

/-! ## Constructor fields share the preceding discourse -/

theorem okExtraTurnAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.addTurn (lifeTotalOf they) (agent := (target .opponent))) = [] := by decide

theorem badExtraTurnAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.addTurn (lifeTotalOf they) (agent := .you)) = [.anaphor (.word .player) .one 0] := by decide

theorem badExtraTurnForwardSubject :
    Instruction.check [] (.addTurn (lifeTotalOf (target .opponent)) (agent := they))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okExtraTurnAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [.addTurn (lifeTotalOf (target .opponent)) (agent := .you), .draw (.lit 1) (agent := they)]) =
          [] := by decide

theorem okExtraTurnNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (.addTurn (plus (.letter .x) (.letter .x)) (agent := .you))) = 1 := by decide

theorem okSkipNextAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.skipPart .drawStep (lifeTotalOf they) (agent := (target .opponent))) = [] := by decide

theorem badSkipNextAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.skipPart .drawStep (lifeTotalOf they) (agent := .you))
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badSkipNextForwardSubject :
    Instruction.check [] (.skipPart .drawStep (lifeTotalOf (target .opponent)) (agent := they))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okSkipNextAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [.skipPart .drawStep (lifeTotalOf (target .opponent)) (agent := .you), .draw (.lit 1) (agent
          := they)])
      = [] := by
  decide

theorem okSkipNextNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (.skipPart .drawStep (plus (.letter .x) (.letter .x)) (agent := .you))) = 1 := by decide

theorem okAdditionalPartAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.addPart .upkeep none (lifeTotalOf they) none (agent := (some (target .opponent))))
      = [] := by
  decide

theorem badAdditionalPartAmountReadsSubject :
    Instruction.check [qualityB .color]
      (.addPart .upkeep none (lifeTotalOf they) none (agent := (some .you)))
      = [.anaphor (.word .player) .one 0] := by
  decide

theorem badAdditionalPartForwardSubject :
    Instruction.check [] (.addPart .upkeep none (lifeTotalOf (target .opponent)) none (agent :=
        (some they)))
      = [.anaphor (.word .player) .one 0] := by decide

theorem okAdditionalPartAmountIntroducesPlayer :
    Instruction.check [] (.sequentially
      [.addPart .upkeep none (lifeTotalOf (target .opponent)) none (agent := (some .you)), .draw
          (.lit 1) (agent := they)])
      = [] := by
  decide

theorem okAdditionalPartNestedAmountIntroducesLetterOnce :
    countLetter .x (Instruction.intro []
      (.addPart .upkeep none (plus (.letter .x) (.letter .x)) none (agent := (some .you))))
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
      [.skipUntap (target creature) (lifeTotalOf (target .opponent)), .draw (.lit 1) (agent :=
          they)])
      = [] := by decide

theorem okAdditionalPartWithoutSubjectReadsOuterContext :
    Instruction.check (nomIntro [] (target .opponent))
      (.addPart .upkeep none (lifeTotalOf they) none (agent := none)) = [] := by decide

theorem badAdditionalPartWithoutSubjectOrOuterContext :
    Instruction.check [] (.addPart .upkeep none (lifeTotalOf they) none (agent := none))
      = [.anaphor (.word .player) .one 0] := by decide

/-- Nested amounts leave their most recent mention first, followed by the outer context. -/
theorem okExtraTurnNestedAmountOrder :
    (Instruction.intro [qualityB .color]
      (.addTurn (plus (powerOf (target creature)) (lifeTotalOf (target .opponent))) (agent :=
          .you)))
      = [turnRefB, ⟨.target, .one, .player false⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1)⟩,
         qualityB .color] := by rfl

theorem okSkipNextNestedAmountOrder :
    (Instruction.intro []
      (.skipPart .drawStep
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent))) (agent := .you)))
      = [⟨.target, .one, .player false⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1)⟩] := by rfl

theorem okAdditionalPartNestedAmountOrder :
    (Instruction.intro []
      (.addPart .upkeep none
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent))) none (agent := none)))
      = [⟨.target, .one, .player false⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1)⟩] := by rfl

theorem okUntapNestedAmountOrder :
    (Instruction.intro []
      (.skipUntap .this
        (plus (powerOf (target creature)) (lifeTotalOf (target .opponent)))))
      = [⟨.target, .one, .player false⟩,
         ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1)⟩] := by rfl

end Semantics.Proofs.Turn
