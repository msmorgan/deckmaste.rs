import Semantics.Macros.Declared
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Deontic

Port of `idris/src/Experimental/Proofs/Deontic.idr`: the pins of the Deontic family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Deontic

private def instructionResult (drs : DRS) (instruction : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs instruction).1

/-- An activated ability with no timing, limit, guard, or activator. -/
def activatedAbility (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none

/-- "you may play …" with no zone, limit, timing, or payment named. -/
def playRider : DeonticRider := .play none none none false .itsOwnCost

/-- A cause qualifier narrows the ordinary loss prohibition. -/
theorem okStateBasedLossGate :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor .you) (.conclude .loseGame) { rider := (.stateBased .nonpositiveLife) })) = [] := by decide

theorem okStateBasedLossForPlayerGroup :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor (.playerGroup .yourOpponents)) (.conclude .loseGame) { rider := (.stateBased .nonpositiveLife) })) = [] := by decide

theorem badStateBasedLossOnCreature :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor thisCreature) (.conclude .loseGame) { rider := (.stateBased .nonpositiveLife) })) = [.deedFits] := by decide

/-- Targets belong to the enclosing spell, not to a static ability. -/
theorem okTargetedContinuousStateBasedLossGate :
    Instruction.check (DRS.ofList []) (.establish
      (.cant (.actor (target .anyPlayer)) (.conclude .loseGame) { rider := (.stateBased .nonpositiveLife) }) (some thisTurn)) = [] := by decide

theorem badStateBasedCauseWithoutDeed :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor .you) (.sequentially []) { rider := (.stateBased .nonpositiveLife) })) = [.nonEmpty] := by decide

theorem badStateBasedLossPatientRole :
    Ability.check (DRS.ofList []) (.static
      (.cant .any (.conclude .loseGame) { rider := (.stateBased .nonpositiveLife) })) = [.deedFits] := by decide

/-- "Target creature can't be blocked this turn." -/
theorem okCantBeBlocked :
    Instruction.check (DRS.ofList [])
      (.establish (.cant .any (.block (target creature)) { rider := .noRider })
        (some thisTurn)) = [] := by
  decide

/-- "Target creature can't be attacked this turn." -/
theorem badCantBeAttacked :
    Instruction.check (DRS.ofList [])
      (.establish (.cant .any (.attack (some (target creature))) { rider := .noRider })
        (some thisTurn)) = [.deedFits] := by
  decide

/-- "Target creature card in a graveyard can't block this turn." -/
theorem badCantInGraveyard :
    Instruction.check (DRS.ofList []) (forbidBlock (target (.and [creature, .inZone graveyard])) (some
        soFarThisTurn))
      = [.deedFits] := by
  decide

/-- "creature with power 2 or less" -/
theorem okCreaturePower :
    Predicate.checkAt .object (DRS.ofList []) (.and [.compare [.stat .power] .atMost (.lit 2), creature])
      = [] := by
  decide

/-- "noncreature with power 2 or less" -/
theorem badNoncreaturePower :
    Predicate.checkAt .object (DRS.ofList []) (.and [.compare [.stat .power] .atMost (.lit 2), .not creature])
      = [.contradictionFree] := by
  decide

/-- "with power 2 or less or with toughness 2 or less" -/
theorem okDistinctComparisonDisjuncts :
    Predicate.checkAt .object (DRS.ofList [])
      (.or
        [ .compare [.stat .power] .atMost (.lit 2),
          .compare [.stat .toughness] .atMost (.lit 2) ]) = [] := by
  decide

/-- "with power 2 or less or with power 2 or less" -/
theorem okRepeatedComparisonDisjunct :
    Predicate.checkAt .object (DRS.ofList [])
      (.or [.compare [.stat .power] .atMost (.lit 2), .compare [.stat .power] .atMost (.lit 2)])
      = [] := by
  decide

/-- "with power 2 or less or mana value 3 or less" -/
theorem okMixedCharacteristicDisjunct :
    Predicate.checkAt .object (DRS.ofList [])
      (.or
        [ .compare [.stat .power] .atMost (.lit 2),
          .compare [.stat .manaValue] .atMost (.lit 3) ]) = [] := by
  decide

/-- "You pay 2 life." -/
theorem okPayLifeCost : Instruction.check (DRS.ofList []) (.pay (payLife 2) .once .paid none none) = [] :=
    by decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okDeedAsCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a creature))) (draw (.lit 1))) = [] := by
  decide

/-- a reflexive trigger written as a cost: the reflexive trigger's enclosure law fails on the
term. -/
theorem badReflexiveAsCost :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (.triggerReflexively (gainLife (.lit 2)) (draw (.lit 1))))
        (draw (.lit 1))) = [.reflexEnclosure] := by
  decide

/-- "Discard a card, Sacrifice a creature: Draw a card." -/
theorem okCompoundCost :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound
          [.perform (discard (a (.and [.isCard, .inZone yourHand]))), .perform (sacrifice (a creature))])
        (draw (.lit 1))) = [] := by
  decide

/-- a compound cost of no components -/
theorem badEmptyCompound :
    Ability.check (DRS.ofList []) (activatedAbility (.compound []) (draw (.lit 1))) = [.nonEmpty] := by decide

/-- "Creatures can't attack." -/
theorem okStaticUntargeting :
    Ability.check (DRS.ofList []) (.static (.cant (.actor (allOf creature)) (.attack none) { rider := .noRider }))
      = [] := by
  decide

/-- "You may play a card in your graveyard this turn." -/
theorem okPlayFromGraveyard :
    Instruction.check (DRS.ofList [])
      (.establish
        (.may (.actor .you) (.procedure .play (a (.and [.isCard, .inZone graveyard])) .itsOwnCost none none none) { rider := playRider }) (some thisTurn)) = [] := by
  decide

/-- "You may play a land card from your graveyard this turn." -/
theorem okPlayLandFromGraveyard :
    Instruction.check (DRS.ofList [])
      (.establish
        (.may (.actor .you) (.procedure .play (a (.and [land, .inZone (graveyardOf .you)])) .itsOwnCost none none none) { rider := (.play (some (graveyardOf .you)) none none false .itsOwnCost) }) (some thisTurn))
      = [] := by
  decide

/-- "You may cast a creature card from your graveyard this turn." -/
theorem okCastCreatureFromGraveyard :
    Instruction.check (DRS.ofList [])
      (.establish
        (.may (.actor .you) (.procedure .cast (a (.and [creature, .inZone (graveyardOf .you)])) .itsOwnCost none none none) { rider := (.play (some (graveyardOf .you)) none none false .itsOwnCost) }) (some thisTurn))
      = [] := by
  decide

/-- "unless" on a negated condition -/
theorem okUnlessOnNegated :
    Ability.check (DRS.ofList [])
      (.static
        (.conditional (.cant (.actor thisCreature) (.attack none) { rider := .noRider })
          (.not (exists_ (.and [artifact, .hasPossessor .controller .you]))) .unless_)) = [] := by
  decide

/-- "unless" -/
theorem badUnlessOnPositive :
    Ability.check (DRS.ofList [])
      (.static
        (.conditional (.cant (.actor thisCreature) (.attack none) { rider := .noRider })
          (exists_ (.and [artifact, .hasPossessor .controller .you])) .unless_))
      = [.markingOk] := by
  decide

/-- "Target creature attacks each combat if able." -/
theorem okMustAttackCreature :
    Instruction.check (DRS.ofList [])
      (.establish (.must (.actor (target creature)) (.attack none) { rider := .noRider })
        (some thisTurn)) = [] := by
  decide

/-- "Target land attacks each combat if able.": a land can become a creature while retaining its
land type [CR#205.1b]. This fixture keeps the requirement available if it can attack
[CR#508.1a]. -/
theorem okMustAttackLand :
    Instruction.check (DRS.ofList [])
      (.establish (.must (.actor (target land)) (.attack none) { rider := .noRider })
        (some thisTurn)) = [] := by
  decide

/-- "your Ring-bearer": a creature holds the Ring-bearer designation for a player, so the
possessive is written [CR#701.54e]. -/
theorem okRingBearerHolder :
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "ringBearer") (some .you)) = [] := by decide

/-- "your monarch": the monarch IS a player, so the designation has no possessor, refused at
the holder slot, not at the designation [CR#725.1]. -/
theorem badMonarchHolder :
    Predicate.checkAt .player (DRS.ofList []) (.hasDesignation (.named "monarch") (some .you))
      = [.designationPossessorFits (.named "monarch")] := by
  decide

/-- "target creature you control that is your Ring-bearer" -/
theorem okRingBearerOnBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.and [creature, .hasDesignation (.named "ringBearer") (some .you), .inZone battlefield]) = [] := by
  decide

/-- "This creature can't attack target planeswalker this turn." -/
theorem okForbidAttackPlaneswalker :
    Instruction.check (DRS.ofList [])
      (.establish
        (.cant (.actor thisCreature) (.attack (some (target (.hasType .planeswalker)))) { rider := .noRider }) (some thisTurn)) = [] := by
  decide

/-- "Target creature blocks itself this turn." -/
theorem badBlocksItself :
    Instruction.check (DRS.ofList [])
      (.establish (.must (.actor (target creature)) (.block (.refl .one)) { rider := .noRider })
        (some thisTurn)) = [.deonticPatientOk] := by
  decide

theorem bareBlockPatientCannotReadItsCoargument :
    Instruction.check (DRS.ofList [])
      (.establish (.must (.actor (target creature)) (.block it) { rider := .noRider })
        (some thisTurn)) = [.anaphor .bare .one 0] := by decide

/-- "Enchanted creature can't block." -/
theorem okEnchantedCreatureCantBlock :
    Ability.check (DRS.ofList [])
      (.static
        (.cant (.actor (.attachHost .enchanted (.type .creature))) (.block .any) { rider := .noRider }))
      = [] := by
  decide

/-- "Enchanted land gets +1/+1 and can't block.": the power/toughness effect is created even for a
noncreature land [CR#208.3a]. If it becomes a creature, it can also be considered when blockers
are chosen [CR#205.1b,509.1a]. -/
theorem okCoordinatedLandHostBlocks :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ .modification (.attachHost .enchanted (.type .land)) .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            .cant (.actor it) (.block .any) { rider := .noRider } ])) = [] := by
  decide

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAfterDealing :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ (Instruction.instruct .this (.dealDamage (.lit 4) (target creature) .any)),
          .establish (.preventionBan .any .thatDamage .noPreventionOnly) none ]) = [] := by
  decide

/-- "You gain 3 life. The damage can't be prevented." -/
theorem badTheDamageAfterLifeGain :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .changeLife (.up (.lit 3)),
          .establish (.preventionBan .any .thatDamage .noPreventionOnly) none ])
      = [.damageDealtInScope] := by
  decide

def afterDamageDealt : DRS :=
  instructionResult (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 4) (target creature) .any))

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAnnounced :
    StaticSpec.check afterDamageDealt (.preventionBan .any .thatDamage .noPreventionOnly)
      = [] := by
  decide

/-- "The damage can't be prevented." -/
theorem badTheDamageUnannounced :
    StaticSpec.check (DRS.ofList []) (.preventionBan .any .thatDamage .noPreventionOnly)
      = [.damageDealtInScope] := by
  decide

/-- "You may cast spells as though they had flash." -/
theorem okObjectPremiseAtCast :
    StaticSpec.check (DRS.ofList [])
      (.may (.actor .you) (.procedure .cast (allOf spell) .itsOwnCost none none none) { asThough := (some (.of (.hasKeyword (.the (.named "flash"))))), rider := playRider }) = [] := by
  decide

/-- "This creature can't be blocked by more than one creature." -/
theorem okBlockBoundOnBlock :
    StaticSpec.check (DRS.ofList [])
      (.cant (.by (allOf creature)) (.block thisCreature) { rider := .noRider, bound := (some (.moreThan (.lit 1))) }) = [] := by
  decide

/-- "Spells and abilities can't be countered.": an effect that says so counters abilities
[CR#113.9], so the Counter deed's patient admits a spell or an ability. -/
theorem okCantCounterSpellsAndAbilities :
    StaticSpec.check (DRS.ofList [])
      (objectCant (.action "counter") (allOf (.or [spell, .abilityHead .anyActivated])))
      = [] := by
  decide

/-- "Activated abilities can't be countered." -/
theorem okCantCounterAbilities :
    StaticSpec.check (DRS.ofList []) (objectCant (.action "counter") (allOf (.abilityHead .anyActivated)))
      = [] := by
  decide

/-- "Creatures can't be countered.": only a spell or an ability on the stack is countered. -/
theorem badCantCounterCreatures :
    StaticSpec.check (DRS.ofList []) (objectCant (.action "counter") (allOf creature)) = [.deedFits] := by decide

/-- "Each opponent discards a card, if those cards are creature cards." -/
theorem distributedDeedReadsBackPluralUnderCondition :
    Instruction.check (DRS.ofList [])
      (.doOnlyIf (.sequentially [Macros.instruct (each .opponent) (choose (a (.and [.isCard, .inZone hand]))), discard them]) (.matches (those .card)
          creature)
        none) = [] := by
  decide

/-- "... sacrificed permanents can't be regenerated." -/
theorem distributedDeedRiderReadsBackPlural :
    Instruction.check (DRS.ofList [])
      (.doAndForbid (Macros.instruct (each .opponent) (sacrifice (a creature))) (.action "regenerate")
        (theVerbed (.action "sacrifice") .many (some .permanent))) = [] := by
  decide

/-- "... dealt damage, they lose half their life, rounded up." -/
theorem enchantedPlayerDamageReadsBackAsThey :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount (.attachHost .enchanted .player) .any))
        (Macros.instruct they (loseLife (.half .up (lifeTotalOf they))))) = [] := by
  decide

/-- "Target creature attacks a player other than you during its controller's next turn if
able." No printed card on the bench. -/
theorem goadedAttacksOther :
    Instruction.check (DRS.ofList [])
      (.establish
        (.must (.actor (target creature)) (.attack (some (a otherPlayer))) { rider := .noRider })
        (some untilYourNextTurn)) = [] := by
  decide

/-- "as you scry": a concurrent window with no printed card on the bench. -/
theorem whileScrying :
    Timing.check (DRS.ofList []) (.whileDoing (.does (.actor .you) (.enact (.action "scry") (scry .anyAmount) none none)))
      = [] := by
  decide

/-- "Creatures can't attack unless their controller pays {X} for each attacking creature they
control." The gated cost sits at the deontic clause's own intro, which is where the payer is
introduced. -/
theorem okGatedCostReadsPayer :
    StaticSpec.check [letterB .x]
      (.gatedBy (Primitives.Cost.scaled (.mana [generic 1])
            (times (.letter .x)
              (countOf (.and [creature, attacking, .hasPossessor .controller they])))) (.actor (allOf creature)) (.attack none) { rider := .noRider }) = [] := by
  decide

/-- "{X} for each attacking creature": the same scaled cost with no read. -/
theorem okBareScaledCost :
    Cost.check [letterB .x]
      (Primitives.Cost.scaled (.mana [generic 1]) (times (.letter .x) (countOf (.and [creature, attacking])))) = [] := by
  decide

/-- The same cost written on its own: outside the clause there is no payer to read, and a cost
carries no agent context of its own. -/
theorem badBareCostReadsPayer :
    Cost.check [letterB .x]
      (Primitives.Cost.scaled (.mana [generic 1])
        (times (.letter .x) (countOf (.and [creature, attacking, .hasPossessor .controller they]))))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "This card can't be revealed": what is revealed is a card [CR#701.20a], so a deontic rule
names it in the reveal deed's patient role. -/
theorem okCantBeRevealed :
    StaticSpec.check (DRS.ofList []) (.cant .any (revealCards .this) { rider := .noRider }) = [] := by decide

end Semantics.Proofs.Deontic
