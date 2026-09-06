import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Deontic

Port of `idris/src/Experimental/Proofs/Deontic.idr`: the pins of the Deontic family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Deontic

/-- An activated ability with no timing, limit, guard, or activator. -/
def act (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none

/-- "you may play …" with no zone, limit, timing, or payment named. -/
def playRider : DeonticRider := .play none none none false .itsOwnCost

/-- A cause qualifier narrows the ordinary loss prohibition. -/
theorem okStateBasedLossGate :
    Ability.check [] (.static
      (.deonticRule .you .forbid [.core .loseGame] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [] := by decide

theorem okStateBasedLossForPlayerGroup :
    Ability.check [] (.static
      (.deonticRule (.playerGroup .yourOpponents) .forbid [.core .loseGame] .agent none .noPatient
          none
        (.stateBased .nonpositiveLife))) = [] := by decide

theorem badStateBasedLossOnCreature :
    Ability.check [] (.static
      (.deonticRule thisCreature .forbid [.core .loseGame] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [.deedFits] := by decide

theorem badTargetedStateBasedLossGate :
    Ability.check [] (.static
      (.deonticRule (target .anyPlayer) .forbid [.core .loseGame] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [.nontarget] := by decide

/-- Targets belong to the enclosing spell, not to a static ability. -/
theorem okTargetedContinuousStateBasedLossGate :
    Instruction.check [] (.establish
      (.deonticRule (target .anyPlayer) .forbid [.core .loseGame] .agent none .noPatient none
        (.stateBased .nonpositiveLife)) (some .thisTurn)) = [] := by decide

theorem badStateBasedCauseForLifeGain :
    Ability.check [] (.static
      (.deonticRule .you .forbid [.core .gainLife] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [.deonticRiderOk] := by decide

theorem badStateBasedCauseForMixedDeeds :
    Ability.check [] (.static
      (.deonticRule .you .forbid [.core .loseGame, .core .winGame] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [.deonticRiderOk] := by decide

theorem badStateBasedCauseWithoutDeed :
    Ability.check [] (.static
      (.deonticRule .you .forbid [] .agent none .noPatient none
        (.stateBased .nonpositiveLife))) = [.nonEmpty] := by decide

theorem badStateBasedLossPatientRole :
    Ability.check [] (.static
      (.deonticRule .you .forbid [.core .loseGame] .patient none .noPatient none
        (.stateBased .nonpositiveLife))) = [.deedFits] := by decide

/-- "Target creature can't be blocked this turn." -/
theorem okCantBeBlocked :
    Instruction.check []
      (.establish (deontic (target creature) .forbid [.core .block] .patient .noPatient)
        (some .thisTurn)) = [] := by
  decide

/-- "Target creature can't be attacked this turn." -/
theorem badCantBeAttacked :
    Instruction.check []
      (.establish (deontic (target creature) .forbid [.core .attack] .patient .noPatient)
        (some .thisTurn)) = [.deedFits] := by
  decide

/-- "Target creature card in a graveyard can't block this turn." -/
theorem badCantInGraveyard :
    Instruction.check [] (forbidBlock (target (.and [creature, .inZone graveyard])) (some
        .thisTurn))
      = [.deedFits] := by
  decide

/-- "creature with power 2 or less" -/
theorem okCreaturePower :
    Predicate.check .object [] (.and [.compare [.stat .power] .atMost (.lit 2), creature])
      = [] := by
  decide

/-- "noncreature with power 2 or less" -/
theorem badNoncreaturePower :
    Predicate.check .object [] (.and [.compare [.stat .power] .atMost (.lit 2), .not creature])
      = [.contradictionFree] := by
  decide

/-- "with power 2 or less or with toughness 2 or less" -/
theorem okDistinctComparisonDisjuncts :
    Predicate.check .object []
      (.or
        [ .compare [.stat .power] .atMost (.lit 2),
          .compare [.stat .toughness] .atMost (.lit 2) ]) = [] := by
  decide

/-- "with power 2 or less or with power 2 or less" -/
theorem okRepeatedComparisonDisjunct :
    Predicate.check .object []
      (.or [.compare [.stat .power] .atMost (.lit 2), .compare [.stat .power] .atMost (.lit 2)])
      = [] := by
  decide

/-- "with power 2 or less or mana value 3 or less" -/
theorem badMixedCharacteristicDisjunct :
    Predicate.check .object []
      (.or
        [ .compare [.stat .power] .atMost (.lit 2),
          .compare [.stat .manaValue] .atMost (.lit 3) ]) = [.parallelDisjuncts] := by
  decide

/-- "You pay 2 life." -/
theorem okPayLifeCost : Instruction.check [] (.pay (payLife .you 2) .once (agent := .you)) = [] :=
    by decide

/-- "You pay {T}." -/
theorem badPayTapSymbol :
    Instruction.check [] (.pay .tapSymbol .once (agent := .you)) = [.payable] := by decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okDeedAsCost :
    Ability.check [] (act (.perform (sacrifice (a creature) (agent := .you))) (draw (.lit 1) (agent
        := .you))) = [] := by
  decide

/-- "Creatures you control get +1/+1 until end of turn:" -/
theorem badContinuousAsCost :
    Ability.check []
      (act
        (.perform
          (get (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- a replacement written as a cost -/
theorem badInsteadAsCost :
    Ability.check []
      (act (.perform (.replace (destroy (target creature)) (exile (target creature))))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- a delayed trigger written as a cost -/
theorem badDelayedAsCost :
    Ability.check []
      (act (.perform (delay (.beginningOf .the .endStep .noPossessor) (draw (.lit 1) (agent :=
          .you))))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- an "until" rider written as a cost -/
theorem badHeldUntilAsCost :
    Ability.check []
      (act (.perform (.holdUntil (exile (target creature)) (Primitives.GameEvent.dies (a creature))))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- a reflexive trigger written as a cost. The Idris pin refutes the cost; the reflexive
trigger's enclosure law fails on the same term. -/
theorem badReflexiveAsCost :
    Ability.check []
      (act (.perform (.triggerReflexively (gainLife (.lit 2) (agent := .you)) (draw (.lit 1) (agent
          := .you))))
        (draw (.lit 1) (agent := .you))) = [.reflexEnclosure, .costAction] := by
  decide

/-- "You skip your next turn:" -/
theorem badSkipAsCost :
    Ability.check [] (act (.perform (.skipPart .turn (.lit 1) (agent := .you))) (draw (.lit 1)
        (agent := .you)))
      = [.costAction] := by
  decide

/-- "You pay 2 life:" -/
theorem badPayAsCost :
    Ability.check [] (act (.perform (.pay (payLife .you 2) .once (agent := .you))) (draw (.lit 1)
        (agent := .you)))
      = [.costAction] := by
  decide

/-- "Discard a card, then sacrifice a creature:" -/
theorem badSequentialCost :
    Ability.check []
      (act (.perform (.sequentially [discard (a (.inZone hand)) (agent := .you), sacrifice (a creature)
          (agent := .you)]))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- "Discard a card and sacrifice a creature simultaneously:" -/
theorem badSimultaneousCost :
    Ability.check []
      (act
        (.perform (.simultaneously [discard (a (.inZone hand)) (agent := .you), sacrifice (a
            creature) (agent := .you)]))
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- "Repeat this process:" -/
theorem badRepeatAsCost :
    Ability.check [] (act (.perform (.repeat_ .again)) (draw (.lit 1) (agent := .you))) =
        [.costAction] := by
  decide

/-- "Discard a card, Sacrifice a creature: Draw a card." -/
theorem okCompoundCost :
    Ability.check []
      (act
        (.compound
          [.perform (discard (a (.inZone hand)) (agent := .you)), .perform (sacrifice (a creature)
              (agent := .you))])
        (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- a compound cost of no components -/
theorem badEmptyCompound :
    Ability.check [] (act (.compound []) (draw (.lit 1) (agent := .you))) = [.nonEmpty] := by decide

/-- "Creatures can't attack." -/
theorem okStaticUntargeting :
    Ability.check [] (.static (deontic (allOf creature) .forbid [.core .attack] .agent .noPatient))
      = [] := by
  decide

/-- "Target creature can't attack." -/
theorem badStaticTargets :
    Ability.check [] (.static (deontic (target creature) .forbid [.core .attack] .agent .noPatient))
      = [.nontarget] := by
  decide

/-- "You may play a card in your graveyard this turn." -/
theorem okPlayFromGraveyard :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Play"] .agent none
          (.counterpart (a (.inZone graveyard))) none playRider) (some .thisTurn)) = [] := by
  decide

/-- "You may play a creature this turn." -/
theorem badPlayFromBattlefield :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Play"] .agent none (.counterpart (a creature)) none
          playRider)
        (some .thisTurn)) = [.deonticRiderOk] := by
  decide

/-- "You may play a land card from your graveyard this turn." -/
theorem okPlayLandFromGraveyard :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Play"] .agent none
          (.counterpart (a (.and [land, .inZone (graveyardOf .you)]))) none
          (.play (some (graveyardOf .you)) none none false .itsOwnCost)) (some .thisTurn))
      = [] := by
  decide

/-- "You may play a spell this turn." -/
theorem badPlayFromStack :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Play"] .agent none (.counterpart (a spell)) none
            playRider)
        (some .thisTurn)) = [.deonticRiderOk] := by
  decide

/-- "You may cast a creature card from your graveyard this turn." -/
theorem okCastCreatureFromGraveyard :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Cast"] .agent none
          (.counterpart (a (.and [creature, .inZone (graveyardOf .you)]))) none
          (.play (some (graveyardOf .you)) none none false .itsOwnCost)) (some .thisTurn))
      = [] := by
  decide

/-- "You may cast a land card from your graveyard this turn." -/
theorem badCastALand :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Cast"] .agent none
          (.counterpart (a (.and [land, .inZone (graveyardOf .you)]))) none playRider)
        (some .thisTurn)) = [.deonticPatientOk] := by
  decide

/-- "You may play a creature card in exile from your graveyard this turn." -/
theorem badPlayFromWrongZone :
    Instruction.check []
      (.establish
        (.deonticRule .you .permit [.action "Play"] .agent none
          (.counterpart (a (.and [creature, .inZone exileZone]))) none
          (.play (some (graveyardOf .you)) none none false .itsOwnCost)) (some .thisTurn))
      = [.deonticRiderOk] := by
  decide

/-- "unless" on a negated condition -/
theorem okUnlessOnNegated :
    Ability.check []
      (.static
        (.conditional (deontic thisCreature .forbid [.core .attack] .agent .noPatient)
          (.not (exists_ (.and [artifact, .hasPossessor .controller .you]))) .unless_)) = [] := by
  decide

/-- "unless" -/
theorem badUnlessOnPositive :
    Ability.check []
      (.static
        (.conditional (deontic thisCreature .forbid [.core .attack] .agent .noPatient)
          (exists_ (.and [artifact, .hasPossessor .controller .you])) .unless_))
      = [.markingOk] := by
  decide

/-- a trigger header watching a permanent become unflipped. The status law fails for the header
and again for the event it is read as. -/
theorem badUnflipEvent :
    Ability.check []
      (whenever (.statusEvent (a permanent) .unflipped) (draw (.lit 1) (agent := .you)))
      = [.statusMarkable, .statusMarkable] := by
  decide

/-- "Target creature attacks each combat if able." -/
theorem okMustAttackCreature :
    Instruction.check []
      (.establish (deontic (target creature) .require [.core .attack] .agent .noPatient)
        (some .thisTurn)) = [] := by
  decide

/-- "Target land attacks each combat if able.": a land an effect has made a creature can attack
[CR#205.1b], and the requirement is created even while it is not one [CR#208.3a]. -/
theorem okMustAttackLand :
    Instruction.check []
      (.establish (deontic (target land) .require [.core .attack] .agent .noPatient)
        (some .thisTurn)) = [] := by
  decide

/-- "your Ring-bearer": a creature holds the Ring-bearer designation for a player, so the
possessive is written [CR#701.54e]. -/
theorem okRingBearerHolder :
    Predicate.check .object [] (.hasDesignation "Ring-bearer" (some .you)) = [] := by decide

/-- "your monarch": the monarch IS a player, so the designation has no possessor, refused at
the holder slot, not at the designation [CR#725.1]. -/
theorem badMonarchHolder :
    Predicate.check .player [] (.hasDesignation "the monarch" (some .you))
      = [.designationPossessorFits "the monarch"] := by
  decide

/-- "target creature you control that is your Ring-bearer" -/
theorem okRingBearerOnBattlefield :
    Predicate.check .object []
      (.and [creature, .hasDesignation "Ring-bearer" (some .you), .inZone battlefield]) = [] := by
  decide

/-- "target creature card in your graveyard that is your Ring-bearer" -/
theorem badRingBearerInGraveyard :
    Predicate.check .object []
      (.and [creature, .hasDesignation "Ring-bearer" (some .you), .inZone (graveyardOf .you)])
      = [.zoneCoherent] := by
  decide

/-- "This creature can't attack target planeswalker this turn." -/
theorem okForbidAttackPlaneswalker :
    Instruction.check []
      (.establish
        (deontic thisCreature .forbid [.core .attack] .agent
          (.counterpart (target (.hasType .planeswalker)))) (some .thisTurn)) = [] := by
  decide

/-- "This creature can't attack target creature this turn." -/
theorem badForbidAttackWithPatient :
    Instruction.check []
      (.establish
        (deontic thisCreature .forbid [.core .attack] .agent (.counterpart (target creature)))
        (some .thisTurn)) = [.deonticPatientOk] := by
  decide

/-- "Target creature blocks it this turn" -/
theorem badBlocksItself :
    Instruction.check []
      (.establish (deontic (target creature) .require [.core .block] .agent (.counterpart it))
        (some .thisTurn)) = [.deonticPatientOk] := by
  decide

/-- "Enchanted creature can't block." -/
theorem okEnchantedCreatureCantBlock :
    Ability.check []
      (.static
        (deontic (.attachHost .enchanted (.type .creature)) .forbid [.core .block] .agent
          .noPatient))
      = [] := by
  decide

/-- "Enchanted land gets +1/+1 and can't block.": a land an effect has made a creature can
block [CR#205.1b], and the restriction is created even while it is not one [CR#208.3a]. -/
theorem okCoordinatedLandHostBlocks :
    Ability.check []
      (.static
        (.conjunction none
          [ .modification (.attachHost .enchanted (.type .land)) .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            deontic it .forbid [.core .block] .agent .noPatient ])) = [] := by
  decide

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAfterDealing :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 4) (target creature),
          .establish (.preventionBan .any .thatDamage .noPreventionOnly) none ]) = [] := by
  decide

/-- "You gain 3 life. The damage can't be prevented." -/
theorem badTheDamageAfterLifeGain :
    Instruction.check []
      (.sequentially
        [ .changeLife (.up (.lit 3)) (agent := .you),
          .establish (.preventionBan .any .thatDamage .noPreventionOnly) none ])
      = [.damageDealtInScope] := by
  decide

def afterDamageDealt : Bindings :=
  Instruction.intro [] (.dealDamage .this (.lit 4) (target creature))

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAnnounced :
    StaticSpec.check afterDamageDealt (.preventionBan .any .thatDamage .noPreventionOnly)
      = [] := by
  decide

/-- "The damage can't be prevented." -/
theorem badTheDamageUnannounced :
    StaticSpec.check [] (.preventionBan .any .thatDamage .noPreventionOnly)
      = [.damageDealtInScope] := by
  decide

/-- "You may cast spells as though they had flash." -/
theorem okObjectPremiseAtCast :
    StaticSpec.check []
      (.deonticRule .you .permit [.action "Cast"] .agent none (.counterpart (allOf spell))
        (some (.of (.hasKeyword (.the "Flash")))) playRider) = [] := by
  decide

/-- "This creature can attack as though it were mana of any color." -/
theorem badManaPremiseAtAttack :
    StaticSpec.check []
      (.deonticRule thisCreature .permit [.core .attack] .agent none .noPatient
        (some (.mana none .anyColor none)) .noRider) = [.asThoughOk] := by
  decide

/-- "This creature can't be blocked by more than one creature." -/
theorem okBlockBoundOnBlock :
    StaticSpec.check []
      (.deonticRule thisCreature .forbid [.core .block] .patient (some (.moreThan (.lit 1)))
        (.counterpart (allOf creature)) none .noRider) = [] := by
  decide

/-- "This spell can't be countered more than once." -/
theorem badCounterBoundTwice :
    StaticSpec.check []
      (.deonticRule .this .forbid [.action "Counter"] .patient (some (.moreThan (.lit 1)))
          .noPatient
        none .noRider) = [.deonticBoundOk] := by
  decide

/-- "You may spend mana as though it weren't a creature." -/
theorem badObjectPremiseAtSpend :
    StaticSpec.check []
      (.deonticRule .you .permit [.core .spend] .agent none .noPatient (some (.of (.not creature)))
        .noRider) = [.asThoughOk] := by
  decide

/-- "Spells and abilities can't be countered.": an effect that says so counters abilities
[CR#113.9], so the Counter deed's patient admits a spell or an ability. -/
theorem okCantCounterSpellsAndAbilities :
    StaticSpec.check []
      (objectCant (.action "Counter") (allOf (.or [spell, .abilityHead .anyActivated])))
      = [] := by
  decide

/-- "Activated abilities can't be countered." -/
theorem okCantCounterAbilities :
    StaticSpec.check [] (objectCant (.action "Counter") (allOf (.abilityHead .anyActivated)))
      = [] := by
  decide

/-- "Creatures can't be countered.": only a spell or an ability on the stack is countered. -/
theorem badCantCounterCreatures :
    StaticSpec.check [] (objectCant (.action "Counter") (allOf creature)) = [.deedFits] := by decide

/-- "Each opponent discards a card, if those cards are creature cards." -/
theorem distributedDeedReadsBackPluralUnderCondition :
    Instruction.check []
      (.doOnlyIf (discard (a (.inZone hand)) (agent := (each .opponent))) (.matches (those .card)
          creature)
        none) = [] := by
  decide

/-- "... sacrificed permanents can't be regenerated." -/
theorem distributedDeedRiderReadsBackPlural :
    Instruction.check []
      (.doAndForbid (sacrifice (a creature) (agent := (each .opponent))) (.action "Regenerate")
        (theVerbed (.action "Sacrifice") .permanent .attributive .many)) = [] := by
  decide

/-- "... dealt damage, they lose half their life, rounded up." -/
theorem enchantedPlayerDamageReadsBackAsThey :
    Ability.check []
      (whenever (Primitives.GameEvent.isDealtDamage .any (.attachHost .enchanted .player))
        (loseLife (.half .up (lifeTotalOf they)) (agent := they))) = [] := by
  decide

/-- "Target creature attacks a player other than you during its controller's next turn if
able." No printed card on the bench. -/
theorem goadedAttacksOther :
    Instruction.check []
      (.establish
        (deontic (target creature) .require [.core .attack] .agent
          (.defendingPlayer (a otherPlayer)))
        (some untilYourNextTurn)) = [] := by
  decide

/-- "as you scry": a concurrent window with no printed card on the bench. -/
theorem whileScrying :
    Concurrent.check [] (.whileDoing (.verbedEvent (some .you) (.action "Scry") none none none))
      = [] := by
  decide

/-- "Creatures can't attack unless their controller pays {X} for each attacking creature they
control." The gated cost sits at the deontic clause's own intro, which is where the payer is
introduced. -/
theorem okGatedCostReadsPayer :
    StaticSpec.check [letterB .x]
      (deontic (allOf creature)
        (.gatedBy
          (scaledMana .generic
            (times (.letter .x)
              (countOf (.and [creature, attacking, .hasPossessor .controller they])))))
        [.core .attack] .agent .noPatient) = [] := by
  decide

/-- "{X} for each attacking creature": the same scaled cost with no read. -/
theorem okBareScaledCost :
    Cost.check [letterB .x]
      (scaledMana .generic (times (.letter .x) (countOf (.and [creature, attacking])))) = [] := by
  decide

/-- The same cost written on its own: outside the clause there is no payer to read, and a cost
carries no agent context of its own. -/
theorem badBareCostReadsPayer :
    Cost.check [letterB .x]
      (scaledMana .generic
        (times (.letter .x) (countOf (.and [creature, attacking, .hasPossessor .controller they]))))
      = [.anaphor (.word .player) .one 0] := by
  decide

end Semantics.Proofs.Deontic
