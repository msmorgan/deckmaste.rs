import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Deontic

Port of `idris/src/Experimental/Proofs/Deontic.idr`: the pins of the Deontic family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Deontic

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- An activated ability with no timing, limit, guard, or activator. -/
def activatedAbility (cost : Cost) (action : Action) : Ability :=
  .activated cost action none none none none

/-- A cause qualifier narrows the ordinary loss prohibition. -/
theorem okStateBasedLossGate :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor .you) (.conclude .loseGame (some .nonpositiveLife)) {})) = [] := by decide

theorem okStateBasedLossForPlayerGroup :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor (.described .all (.isOpponentOf .you))) (.conclude .loseGame (some .nonpositiveLife)) {})) = [] := by decide

/-- Targets belong to the enclosing spell, not to a static ability. -/
theorem okTargetedContinuousStateBasedLossGate :
    Action.check (DRS.ofList []) (.establish
      (.cant (.actor (target .anyPlayer)) (.conclude .loseGame (some .nonpositiveLife)) {}) (some thisTurn)) = [] := by decide

/-- A prohibition over an empty action names no deed. -/
theorem badEmptyProhibitedAction :
    Ability.check (DRS.ofList []) (.static
      (.cant (.actor .you) (.sequentially []) {})) = [.eventHead, .nonEmpty] := by decide

/-- "Target creature can't be blocked this turn." -/
theorem okCantBeBlocked :
    Action.check (DRS.ofList [])
      (.establish (.cant .any (.block (target isCreature)) {})
        (some thisTurn)) = [] := by
  decide

/-- "creature with power 2 or less" -/
theorem okCreaturePower :
    Predicate.checkAt .object (DRS.ofList []) (.and [.compare [.stat .power] .atMost 2, isCreature])
      = [] := by
  decide

/-- "with power 2 or less or with toughness 2 or less" -/
theorem okDistinctComparisonDisjuncts :
    Predicate.checkAt .object (DRS.ofList [])
      (.or
        [ .compare [.stat .power] .atMost 2,
          .compare [.stat .toughness] .atMost 2 ]) = [] := by
  decide

/-- "with power 2 or less or with power 2 or less" -/
theorem okRepeatedComparisonDisjunct :
    Predicate.checkAt .object (DRS.ofList [])
      (.or [.compare [.stat .power] .atMost 2, .compare [.stat .power] .atMost 2])
      = [] := by
  decide

/-- "with power 2 or less or mana value 3 or less" -/
theorem okMixedCharacteristicDisjunct :
    Predicate.checkAt .object (DRS.ofList [])
      (.or
        [ .compare [.stat .power] .atMost 2,
          .compare [.stat .manaValue] .atMost 3 ]) = [] := by
  decide

/-- "You pay 2 life." -/
theorem okPayLifeCost : Action.check (DRS.ofList []) (.pay (payLife 2) .once .paid none none) = [] :=
    by decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okDeedAsCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a isCreature))) (draw 1)) = [] := by
  decide

/-- a reflexive trigger written as a cost: the reflexive trigger's enclosure law fails on the
term. -/
theorem badReflexiveAsCost :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (.triggerReflexively (gainLife 2) none (draw 1)))
        (draw 1)) = [.reflexEnclosure] := by
  decide

/-- "Discard a card, Sacrifice a creature: Draw a card." -/
theorem okCompoundCost :
    Ability.check (DRS.ofList [])
      (activatedAbility
        (.compound
          [.perform (discard (a (.and [.isCard, .in_ yourHand]))), .perform (sacrifice (a isCreature))])
        (draw 1)) = [] := by
  decide

/-- a compound cost of no components -/
theorem badEmptyCompound :
    Ability.check (DRS.ofList []) (activatedAbility (.compound []) (draw 1)) = [.nonEmpty] := by decide

/-- "Creatures can't attack." -/
theorem okStaticUntargeting :
    Ability.check (DRS.ofList []) (.static (.cant (.actor (allOf isCreature)) (.attack none) {}))
      = [] := by
  decide

/-- "You may play a card in your graveyard this turn." -/
theorem okPlayFromGraveyard :
    Action.check (DRS.ofList [])
      (.establish
        (.can (.actor .you) (.procedure .play (a (.and [.isCard, .in_ graveyard])) .itsOwnCost none none none) {}) (some thisTurn)) = [] := by
  decide

/-- "You may play a land card from your graveyard this turn." -/
theorem okPlayLandFromGraveyard :
    Action.check (DRS.ofList [])
      (.establish
        (.can (.actor .you) (.procedure .play (a (.and [isLand, .in_ (graveyardOf .you)])) .itsOwnCost none (some (graveyardOf .you)) none) {}) (some thisTurn))
      = [] := by
  decide

/-- "You may cast a creature card from your graveyard this turn." -/
theorem okCastCreatureFromGraveyard :
    Action.check (DRS.ofList [])
      (.establish
        (.can (.actor .you) (.procedure .cast (a (.and [isCreature, .in_ (graveyardOf .you)])) .itsOwnCost none (some (graveyardOf .you)) none) {}) (some thisTurn))
      = [] := by
  decide

/-- "Target creature attacks each combat if able." -/
theorem okMustAttackCreature :
    Action.check (DRS.ofList [])
      (.establish (.must (.actor (target isCreature)) (.attack none) {})
        (some thisTurn)) = [] := by
  decide

/-- "Target land attacks each combat if able.": a land can become a creature while retaining its
land type [CR#205.1b]. This fixture keeps the requirement available if it can attack
[CR#508.1a]. -/
theorem okMustAttackLand :
    Action.check (DRS.ofList [])
      (.establish (.must (.actor (target isLand)) (.attack none) {})
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
      (.and [isCreature, .hasDesignation (.named "ringBearer") (some .you), .in_ battlefield]) = [] := by
  decide

/-- "This creature can't attack target planeswalker this turn." -/
theorem okForbidAttackPlaneswalker :
    Action.check (DRS.ofList [])
      (.establish
        (.cant (.actor thisCreature) (.attack (some (target (.hasType .planeswalker)))) {}) (some thisTurn)) = [] := by
  decide

theorem bareBlockPatientCannotReadItsCoargument :
    Action.check (DRS.ofList [])
      (.establish (.must (.actor (target isCreature)) (.block it) {})
        (some thisTurn)) = [.anaphor .bare .one 0] := by decide

/-- "Enchanted creature can't block." -/
theorem okEnchantedCreatureCantBlock :
    Ability.check (DRS.ofList [])
      (.static
        (.cant (.actor (.attachHost .enchanted (.type .creature))) (.block .any) {}))
      = [] := by
  decide

/-- "Enchanted land gets +1/+1 and can't block.": the power/toughness effect is created even for a
noncreature land [CR#208.3a]. If it becomes a creature, it can also be considered when blockers
are chosen [CR#205.1b,509.1a]. -/
theorem okCoordinatedLandHostBlocks :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ Templates.modification (.attachHost .enchanted (.type .land)) .power (.up 1),
            Templates.modification it .toughness (.up 1),
            .cant (.actor it) (.block .any) {} ])) = [] := by
  decide

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAfterDealing :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 4 (target isCreature))),
          .establish (.preventionBan .thatDamage .noPreventionOnly) none ]) = [] := by
  decide

/-- "You gain 3 life. The damage can't be prevented." -/
theorem badTheDamageAfterLifeGain :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .changeLife (.up 3),
          .establish (.preventionBan .thatDamage .noPreventionOnly) none ])
      = [.damageDealtInScope] := by
  decide

def afterDamageDealt : DRS :=
  actionResult (DRS.ofList []) (Action.instruct .this (.dealDamage 4 (target isCreature)))

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAnnounced :
    StaticSpec.check afterDamageDealt (.preventionBan .thatDamage .noPreventionOnly)
      = [] := by
  decide

/-- "The damage can't be prevented." -/
theorem badTheDamageUnannounced :
    StaticSpec.check (DRS.ofList []) (.preventionBan .thatDamage .noPreventionOnly)
      = [.damageDealtInScope] := by
  decide

/-- "You may cast spells as though they had flash." -/
theorem okObjectPremiseAtCast :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor .you) (.procedure .cast (allOf isSpell) .itsOwnCost none none none) { asThough := (some (.condition (.matches (.pro .bare .many) (.hasKeyword (.the (.named "flash")))))) }) = [] := by
  decide

/-- "This creature can't be blocked by more than one creature." -/
theorem okBlockBoundOnBlock :
    StaticSpec.check (DRS.ofList [])
      (.cant (.by (counted (.moreThanOf 1) isCreature)) (.block thisCreature) {}) = [] := by
  decide

/-- "Spells and abilities can't be countered.": an effect that says so counters abilities
[CR#113.9], so the Counter deed's patient admits a spell or an ability. -/
theorem okCantCounterSpellsAndAbilities :
    StaticSpec.check (DRS.ofList [])
      (cant any (counter (allOf (.or [isSpell, .abilityHead .anyActivated]))))
      = [] := by
  decide

/-- "Activated abilities can't be countered." -/
theorem okCantCounterAbilities :
    StaticSpec.check (DRS.ofList []) (cant any (counter (allOf (.abilityHead .anyActivated))))
      = [] := by
  decide

/-- "Each opponent discards a card. If those cards are creature cards, you gain 1 life." -/
theorem distributedDeedReadsBackPluralUnderCondition :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Templates.instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))),
          discard them,
          doIf (.matches (those .card) isCreature) (gainLife 1) ]) = [] := by
  decide

/-- "... sacrificed permanents can't be regenerated." -/
theorem distributedDeedRiderReadsBackPlural :
    Action.check (DRS.ofList [])
      (doUnderProhibition
        (Templates.instruct (each (.isOpponentOf .you)) (sacrifice (a isCreature)))
        (regeneration (bare (and [was (sacrifice which), isPermanent])))) = [] := by
  decide

/-- "Destroy target creature. It can't be regenerated.": later text that modifies the destroy, a
prohibition of the regeneration of what it destroys in force only for the destroy's own event
rather than following it, so no regeneration shield replaces that destruction; its own clause,
"It" read back from the destroy [CR#608.2c,701.19c]. -/
theorem destroyCantRegenerateRidesTheDestroy :
    destroyCantRegenerate (target isCreature) =
      .sequentially [destroy (target isCreature),
        .establish (.cant .any (regeneration it) {}) (some .modifiesPrecedingAction)] := rfl

/-- "This deals 3 damage to target creature. It can't be regenerated this turn.": a printed
"this turn" keeps its duration, the damage and then a prohibition until end of turn. -/
theorem printedThisTurnProhibitionLastsTheTurn :
    doAndForbid (instruct .this (.dealDamage 3 (target isCreature))) (regeneration it) =
      .sequentially [instruct .this (.dealDamage 3 (target isCreature)),
        .establish (.cant .any (regeneration it) {}) (some untilEndOfTurn)] := rfl

/-- The prohibition binds the destroyed object: "it" reads the one destroyed creature and "them"
the destroyed group, and a pronoun of the other number finds nothing to read, refused once. -/
theorem regenerationProhibitionBindsTheDestroyed :
    Action.check (DRS.ofList []) (destroyCantRegenerate (target isCreature)) = [] ∧
      Action.check (DRS.ofList []) (destroyCantRegenerate (allOf isCreature)) = [] ∧
      Action.check (DRS.ofList [])
        (doUnderProhibition (destroy (target isCreature)) (regeneration them)) =
        [.anaphor .bare .many 0] := by
  decide

/-- "... dealt damage, they lose half their life, rounded up." -/
theorem enchantedPlayerDamageReadsBackAsThey :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount (.attachHost .enchanted .player)))
        (Templates.instruct they (loseLife (.half .up (lifeTotalOf they))))) = [] := by
  decide

/-- "Target creature attacks a player other than you during its controller's next turn if
able." No printed card on the bench. -/
theorem goadedAttacksOther :
    Action.check (DRS.ofList [])
      (.establish
        (.must (.actor (target isCreature)) (.attack (some (a isOtherPlayer))) {})
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
              (countOf (.and [isCreature, isAttacking, .hasPossessor .controller they])))) (.actor (allOf isCreature)) (.attack none) {}) = [] := by
  decide

/-- "{X} for each attacking creature": the same scaled cost with no read. -/
theorem okBareScaledCost :
    Cost.check [letterB .x]
      (Primitives.Cost.scaled (.mana [generic 1]) (times (.letter .x) (countOf (.and [isCreature, isAttacking])))) = [] := by
  decide

/-- The same cost written on its own: outside the clause there is no payer to read, and a cost
carries no agent context of its own. -/
theorem badBareCostReadsPayer :
    Cost.check [letterB .x]
      (Primitives.Cost.scaled (.mana [generic 1])
        (times (.letter .x) (countOf (.and [isCreature, isAttacking, .hasPossessor .controller they]))))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "This card can't be revealed": what is revealed is a card [CR#701.20a], so a deontic rule
names it in the reveal deed's patient role. -/
theorem okCantBeRevealed :
    StaticSpec.check (DRS.ofList []) (.cant .any (reveal .this) {}) = [] := by decide

/-- "Each player can't cast more than one spell each turn.": the bound is the noun phrase's own
quantity, the rate the rule's timing. -/
theorem okMoreThanOneSpellEachTurn :
    StaticSpec.check (DRS.ofList [])
      (.cant (.actor (each .anyPlayer))
        (.procedure .cast (counted (.moreThanOf 1) isSpell) .itsOwnCost none none none)
        { timing := some (.inPart .turn none (.each none)) }) = [] := by decide

/-- "You may play an additional land on each of your turns." -/
theorem okAdditionalLandEachOfYourTurns :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor .you) (.procedure .play (counted (.additional (exactly 1)) isLand) .itsOwnCost none none none)
        { timing := some (.inPart .turn (some .you) (.each (some .you))) }) = [] := by decide

/-- The rule's timing is checked: a "that turn" with no turn named before it reads nothing. -/
theorem badDeonticTimingReadsNoTurn :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor .you) (.procedure .play (a isLand) .itsOwnCost none none none)
        { timing := some (.inPart .turn none .that) }) = [.turnAntecedent 0] := by decide

/-- "This creature can attack as though it didn't have defender.": the "as though" clause's
pronoun reads the rule's subject. -/
theorem okAsThoughReadsTheSubject :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor thisCreature) (.attack none)
        { asThough := some (.condition (.matches it (.not (.hasKeyword (.the (.named "defender")))))) }) =
      [] := by decide

/-- An "as though" clause whose pronoun has no participant to read is refused. -/
theorem badAsThoughPronounWithoutParticipant :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor .you) (.draw (.exactlyOf .anyAmount))
        { asThough := some (.condition (.matches them (.hasKeyword (.the (.named "flash"))))) }) =
      [.anaphor .bare .many 0] := by decide

/-- "This creature must be blocked if able": one requirement on the declaration of blockers,
met when at least one creature blocks it [CR#509.1c]. It is not Lure's "must be blocked by all
creatures", a requirement on each creature able to block it. -/
theorem okMustBeBlocked :
    StaticSpec.check (DRS.ofList []) (must any (block thisCreature)) = [] := by decide

theorem mustBeBlockedIsNotLure :
    (must any (block thisCreature) : StaticSpec) ≠
      must (by_ (allOf isCreature)) (block thisCreature) := by
  intro h
  cases h

end Semantics.Proofs.Deontic

