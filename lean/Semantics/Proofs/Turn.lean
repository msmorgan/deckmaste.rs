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
theorem okSacrificeBattlefield : Instruction.check [] (sacrifice .you (a creature)) = [] := by
  decide

/-- "Destroy target creature. At the beginning of the end step, sacrifice it." -/
theorem badStale :
    Instruction.check []
      (.sequentially [destroy (target creature), delayed atTheEndStep (sacrifice .you it)])
      = [.zoneFits] := by
  decide

/-- The Idris pin refutes the "other" anchor; the targeting law the delayed clause loses is
listed too. -/
theorem badDelayedOther :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 2) (target anyTarget),
          delayed atTheEndStep (.dealDamage .this (.lit 1) (target anyOtherTarget)) ])
      = [.anyTargeted (.join .object (.join .object (.join .object .player))), .otherAnchored] := by
  decide

theorem badStaleCarrier :
    Instruction.check []
      (.sequentially [exile (target creatureYouControl), move (that (.type .creature)) battlefield])
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okActivatedCostAndEffect :
    Ability.check [] (act (.perform (sacrifice .you (a creature))) (draw .you (.lit 1))) = [] := by
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
          [.perform (discard .you (a (.inZone hand))), .perform (sacrifice .you (a creature))])
        (exile it)) = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature you control. Sacrifice it." -/
theorem okSacrificeOnBattlefield :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creatureYouControl), sacrifice .you it])
      = [] := by
  decide

/-- "Exile target creature. Sacrifice it." -/
theorem badSacrificeExiled :
    Instruction.check [] (.sequentially [exile (target creature), sacrifice .you it])
      = [.zoneFits] := by
  decide

theorem badDeadCreatureRead :
    Instruction.check []
      (.delayed (.dies (target creature)) [] (some .thisTurn)
        (move (that (.type .creature)) battlefield))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Discard a card: Return the discarded card to the battlefield." -/
theorem okVerbedDiscardedCard :
    Ability.check []
      (act (.perform (discard .you (a (.inZone hand))))
        (move (theVerbed "Discard" .card .attributive .one) battlefield)) = [] := by
  decide

/-- "Discard a card: Return the sacrificed card to the battlefield." -/
theorem badVerbedWrongVerb :
    Ability.check []
      (act (.perform (discard .you (a (.inZone hand))))
        (move (theVerbed "Sacrifice" .card .attributive .one) battlefield))
      = [.anaphor (.verbed "Sacrifice" .card .attributive) .one 0] := by
  decide

/-- "Sacrifice an artifact: Return the sacrificed creature to the battlefield." -/
theorem badVerbedWrongNoun :
    Ability.check []
      (act (.perform (sacrifice .you (a artifact)))
        (move (theVerbed "Sacrifice" (.type .creature) .attributive .one) battlefield))
      = [.anaphor (.verbed "Sacrifice" (.type .creature) .attributive) .one 0] := by
  decide

theorem badVerbedAmbig :
    Ability.check []
      (act
        (.compound [.perform (sacrifice .you (a creature)), .perform (sacrifice .you (a creature))])
        (move (theVerbed "Sacrifice" .card .attributive .one) battlefield))
      = [.anaphor (.verbed "Sacrifice" .card .attributive) .one 2] := by
  decide

theorem badBareCardRead :
    Ability.check []
      (act (.perform (sacrifice .you (a creature)))
        (.sequentially
          [exile (target creature), delayed atTheEndStep (move (that .card) battlefield)]))
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
      (triggered .at_ (.beginningOf .the .upkeep (.byPlayer .you)) (draw .you (.lit 1))) = [] := by
  decide

/-- "At the beginning of your turn, draw a card." -/
theorem badTriggerAtYourTurn :
    Ability.check []
      (triggered .at_ (.beginningOf .the .turn (.byPlayer .you)) (draw .you (.lit 1)))
      = [.windowOk] := by
  decide

/-- "Whenever a creature dies, tap it." -/
theorem badTriggerTapsDeadCreature :
    Ability.check [] (triggered .whenever (.dies (a creature)) (.setStatus .tapped it))
      = [.zoneIs .battlefield] := by
  decide

/-- "Whenever a creature leaves the battlefield, tap it." -/
theorem badLeavesThenTap :
    Ability.check [] (triggered .whenever (leavesBattlefield (a creature)) (.setStatus .tapped it))
      = [.zoneIs .battlefield] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okUntilEndOfTurnDuration :
    Instruction.check []
      (gets (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Target creature gets +3/+3 until the beginning of your next upkeep." -/
theorem badUntilBeginningOfUpkeep :
    Instruction.check []
      (gets (target creature) (.up (.lit 3)) (.up (.lit 3))
        (some (.untilEvent (.beginningOf .the .upkeep (.byPlayer .you))))) = [.durationOk] := by
  decide

/-- "Sacrifice a creature. When you do, draw a card." -/
theorem okReflexiveOnSacrifice :
    Instruction.check [] (.reflexively (sacrifice .you (a creature)) (draw .you (.lit 1)))
      = [] := by
  decide

/-- "This creature deals 3 damage to any target. When you do, draw a card." -/
theorem badReflexiveOnSourceDeed :
    Instruction.check []
      (.reflexively (.dealDamage .this (.lit 3) (target anyTarget)) (draw .you (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "You gain 2 life. When you do, draw a card." -/
theorem badReflexiveOnLifeGain :
    Instruction.check [] (.reflexively (gainsLife .you (.lit 2)) (draw .you (.lit 1)))
      = [.reflexEnclosure] := by
  decide

/-- "Draw a card, then sacrifice a creature. When you do, draw a card." -/
theorem badReflexiveOnSequence :
    Instruction.check []
      (.reflexively (.sequentially [draw .you (.lit 1), sacrifice .you (a creature)])
        (draw .you (.lit 1))) = [.reflexEnclosure] := by
  decide

theorem badReflexiveOnDelayed :
    Instruction.check []
      (.reflexively (delayed (.beginningOf .the .endStep (.byPlayer .you)) (draw .you (.lit 1)))
        (draw .you (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- "Regenerate this creature. If it regenerates this way, draw a card." -/
theorem okThisWayOnRegenerate :
    Instruction.check []
      (.thisWay (regenerate thisCreature) (regenerates thisCreature) (draw .you (.lit 1)))
      = [] := by
  decide

theorem badThisWayOnDelayed :
    Instruction.check []
      (.thisWay (delayed (.beginningOf .the .endStep (.byPlayer .you)) (draw .you (.lit 1)))
        (.draws .you) (draw .you (.lit 1))) = [.thisWayOutcome] := by
  decide

theorem badReflexiveOnBranchedMay :
    Instruction.check []
      (.reflexively (.may .you (sacrifice .you (a creature)) (some (draw .you (.lit 1))) none)
        (draw .you (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- The Idris pin refutes the anaphor; the unresolved "that creature" has no zone either. -/
theorem badAfterReflexiveReadsTrigger :
    Instruction.check []
      (.sequentially
        [ .reflexively (mills .you (.lit 4) .you)
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          .setStatus .tapped (that (.type .creature)) ])
      = [.anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield] := by
  decide

/-- "Sacrifice a creature. When you do, tap it." -/
theorem badReflexiveTapsSacrificed :
    Instruction.check [] (.reflexively (sacrifice .you (a creature)) (.setStatus .tapped it))
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
      (act (.mana [generic 2]) (draw .you (.lit 1))
        (some (.duringPart .endStep (some (each .anyPlayer))))) = [] := by
  decide

/-- "{2}: Draw a card. Activate only during all players' end step." [CR#102.1] -/
theorem badPluralPartWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw .you (.lit 1))
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
      (.getsEmblem .you
        [triggered .at_ (.beginningOf .the .endStep (.byPlayer .you)) (draw .you (.lit 1))])
      = [] := by
  decide

/-- "You get an emblem with 'flying'." -/
theorem badKeywordEmblem :
    Instruction.check [] (.getsEmblem .you [keyword "Flying"]) = [.emblemAbilities] := by decide

/-- "You get an emblem." -/
theorem badEmptyEmblem :
    Instruction.check [] (.getsEmblem .you []) = [.emblemAbilities] := by decide

/-- "… At the beginning of that turn's end step, you lose the game." -/
theorem okDeicticTurnAfterExtraTurn :
    Instruction.check []
      (.sequentially
        [ .extraTurn .you (.lit 1),
          delayed (.beginningOf .the .endStep (.byTurn thatTurn)) (.concludes .loseGame .you) ])
      = [] := by
  decide

/-- "Draw a card. At the beginning of that turn's end step, you lose the game." -/
theorem badDeicticTurnWithoutIntroducer :
    Instruction.check []
      (.sequentially
        [ draw .you (.lit 1),
          delayed (.beginningOf .the .endStep (.byTurn thatTurn)) (.concludes .loseGame .you) ])
      = [.anaphor .thatTurn .one 0] := by
  decide

/-- "After this combat phase, there is an additional upkeep step." -/
theorem okAdditionalUpkeep :
    Instruction.check [] (.additionalPart none .upkeep (some .combat) (.lit 1) none) = [] := by
  decide

/-- "After this combat phase, there is an additional turn." -/
theorem badAdditionalTurn :
    Instruction.check [] (.additionalPart none .turn (some .combat) (.lit 1) none)
      = [.windowOk] := by
  decide

/-- "Spells with the chosen name can't be cast." -/
theorem okCastSpellClass :
    StaticSpec.check [qualityB .cardName] (objectCant "Cast" (allOf (.and [spell, .named .chosen])))
      = [] := by
  decide

/-- "Spells with the chosen name can't be activated." -/
theorem badActivatedSpellClass :
    StaticSpec.check [qualityB .cardName]
      (objectCant "Activate" (allOf (.and [spell, .named .chosen]))) = [.deedFits] := by
  decide

/-- "Activated abilities of artifacts can't be cast." -/
theorem badCastAbilityClass :
    StaticSpec.check []
      (objectCant "Cast" (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf artifact)])))
      = [.deedFits] := by
  decide

/-- "Activated and triggered abilities can't be activated.": a disjunction seeds an ability
when every arm does, so the deed's ability role is met [CR#113.1c]. -/
theorem okAbilityDisjunctionActivated :
    StaticSpec.check []
      (objectCant "Activate" (allOf (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered])))
      = [] := by
  decide

/-- "Activated abilities and creatures can't be activated.": one arm is no ability, so the
disjunction seeds none, and the arms do not parallel each other either. -/
theorem badMixedDisjunctionActivated :
    StaticSpec.check [] (objectCant "Activate" (allOf (.or [.abilityHead .anyActivated, creature])))
      = [.parallelDisjuncts, .deedFits] := by
  decide

/-- "At the beginning of your upkeep, draw a card." -/
theorem okSingularPartPossessor :
    Ability.check []
      (triggered .at_ (.beginningOf .the .upkeep (.byPlayer .you)) (draw .you (.lit 1))) = [] := by
  decide

/-- "At the beginning of all players' upkeep, draw a card." [CR#102.1] -/
theorem badPluralPartPossessor :
    Ability.check []
      (triggered .at_ (.beginningOf .the .upkeep (.byPlayer (allOf .anyPlayer)))
        (draw .you (.lit 1))) = [.windowOk] := by
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
    Instruction.check [] (.ifDone (sacrifice .you (a creature)) (some (draw .you (.lit 1))) none)
      = [] := by
  decide

/-- "Sacrifice a creature." -/
theorem badIfDoneWithNeitherArm :
    Instruction.check [] (.ifDone (sacrifice .you (a creature)) none none) = [.ifDoneArmed] := by
  decide

/-- "This creature deals 3 damage to any target. If you do, draw a card." -/
theorem badIfDoneOverAgentlessBody :
    Instruction.check []
      (.ifDone (.dealDamage thisCreature (.lit 3) (target anyTarget)) (some (draw .you (.lit 1)))
        none) = [.reflexEnclosure] := by
  decide

/-- "Take an extra turn after this one. If you do, draw a card." -/
theorem badIfDoneOverScheduledBody :
    Instruction.check [] (.ifDone (.extraTurn .you (.lit 1)) (some (draw .you (.lit 1))) none)
      = [.reflexEnclosure] := by
  decide

def oneExtraTurn : Bindings := Instruction.intro [] (.extraTurn .you (.lit 1))

/-- "Take an extra turn after this one. Skip the draw step of that turn." -/
theorem okThatTurnAfterASingleTurn :
    NounPhrase.check (some .turnRef) oneExtraTurn thatTurn = [] := by decide

def twoExtraTurns : Bindings := Instruction.intro oneExtraTurn (.extraTurn .you (.lit 1))

theorem badThatTurnAfterTwoTurns :
    NounPhrase.check (some .turnRef) twoExtraTurns thatTurn = [.anaphor .thatTurn .one 2] := by
  decide

/-- "create a legendary 20/20 black Avatar creature token named Marit Lage" -/
theorem okTokenSingleSupertype :
    Instruction.check []
      (create (.lit 1)
        { name := some "Marit Lage", colors := [.black], supertypes := [.legendary],
          types := [.creature], subtypes := [creatureType "Avatar"],
          power := some (.lit 20), toughness := some (.lit 20) }) = [] := by
  decide

/-- "create a legendary legendary 20/20 black Avatar creature token" -/
theorem badTokenDuplicateSupertype :
    Instruction.check []
      (create (.lit 1)
        { name := some "Marit Lage", colors := [.black], supertypes := [.legendary, .legendary],
          types := [.creature], subtypes := [creatureType "Avatar"],
          power := some (.lit 20), toughness := some (.lit 20) }) = [.tokenCanonical] := by
  decide

end Semantics.Proofs.Turn
