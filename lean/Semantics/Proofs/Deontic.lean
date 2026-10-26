import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Deontic
import Semantics.Cards.Damage
import Semantics.Cards.Cost
import Semantics.Cards.Mana
import Semantics.Proofs.Helpers

/-!
# Semantics.Proofs.Deontic

Port of `idris/src/Experimental/Proofs/Deontic.idr`: the pins of the Deontic family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Deontic

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
      (.cant (.actor (target .isPlayer)) (.conclude .loseGame (some .nonpositiveLife)) {}) (some thisTurn)) = [] := by decide

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
theorem okPayLifeCost : Action.check (DRS.ofList []) (payLife 2) = [] :=
    by decide

private def costResult (cost : Cost) : DRS := (stepCost 64 (DRS.ofList []) cost).1

/-- "Pay 2 life: You gain that much life.": in a cost slot `payLife 2` is coerced to
`perform (payLife 2)`, and paying it is one payment of 2 life [CR#118.1,119.4]. It publishes one
life-lost amount and no repeat count, as `perform (loseLife 2)` does, and "that much" reads it. -/
theorem payLifeInACostSlotCountsOnce :
    [costResult (payLife 2), costResult (perform (loseLife 2))].map (fun after =>
      (countOutcomes .lifeLost after, countOutcomes .repeatCount after)) = [(1, 0), (1, 0)] ∧
      Ability.check (DRS.ofList []) (activated (payLife 2) (gainLife thatMuch)) = [] := by
  decide

/-- Paying a cost that is itself a payment of life pays that life once, never a payment of a
payment [CR#118.1,119.4] (`Cost.asPaid`): "you may pay 1 life", the shape of *Carnophage*'s "unless
you pay 1 life" (`doUnless`), of ward's "unless that player pays [Pay 5 life]" and of cumulative
upkeep's "unless you pay [Pay 2 life] for each age counter" each pay the life itself. A cost that
is not a payment is left as written. -/
theorem payingALifePaymentPaysItOnce :
    pay (payLife 1) .once = payLife 1 ∧
      doUnless (tap thisCreature) (payLife 1) =
        .withContinuation .optional (payLife 1) none (tap thisCreature) ∧
      pay (.inCaller (payLife 5)) .once = pay (.inCaller (perform (loseLife 5))) .once ∧
      doUnless (sacrifice thisPermanent) (.scaled (.inCaller (payLife 2)) (.lit 1)) =
        doUnless (sacrifice thisPermanent) (.scaled (.inCaller (perform (loseLife 2))) (.lit 1)) ∧
      pay (.compound [[generic 1], payLife 2]) .once =
        pay (.compound [[generic 1], perform (loseLife 2)]) .once ∧
      pay (perform (sacrifice (a isCreature))) .once =
        .pay (.perform (sacrifice (a isCreature))) .once .paid none none :=
  ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

/-- "Players can't pay life" (*Angel of Jubilation*) forbids the payment, not every loss of life
[CR#119.4,119.8]: it checks clean, and it is not "players can't lose life", which would also forbid
a *Lightning Bolt*'s. -/
theorem cantPayLifeIsNotCantLoseLife :
    Ability.check (DRS.ofList []) (static (cant (a isPlayer) (payLife anyAmount))) = [] ∧
      cant (a isPlayer) (payLife anyAmount) ≠ cant (a isPlayer) (loseLife anyAmount) := by
  refine ⟨by decide, fun h => ?_⟩
  cases h

/-- "Sacrifice a creature: Draw a card." -/
theorem okDeedAsCost :
    Ability.check (DRS.ofList []) (activated (.perform (sacrifice (a isCreature))) (draw 1)) = [] := by
  decide

/-- a reflexive trigger written as a cost: the reflexive trigger's enclosure law fails on the
term. -/
theorem badReflexiveAsCost :
    Ability.check (DRS.ofList [])
      (activated (.perform (.triggerReflexively (gainLife 2) none (draw 1)))
        (draw 1)) = [.reflexEnclosure] := by
  decide

/-- "Discard a card, Sacrifice a creature: Draw a card." -/
theorem okCompoundCost :
    Ability.check (DRS.ofList [])
      (activated
        (.compound
          [.perform (discard (a (.and [.isCard, .in_ yourHand]))), .perform (sacrifice (a isCreature))])
        (draw 1)) = [] := by
  decide

/-- a compound cost of no components -/
theorem badEmptyCompound :
    Ability.check (DRS.ofList []) (activated (.compound []) (draw 1)) = [.nonEmpty] := by decide

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
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "ringBearer" (.heldBy .object)) (some .you)) = [] := by decide

/-- "your monarch": the monarch IS a player, so the designation has no possessor, refused at
the holder slot, not at the designation [CR#725.1]. -/
theorem badMonarchHolder :
    Predicate.checkAt .player (DRS.ofList []) (.hasDesignation (.named "monarch" (.heldBy .player)) (some .you))
      = [.designationPossessorFits (.named "monarch" (.heldBy .player))] := by
  decide

/-- "target creature you control that is your Ring-bearer" -/
theorem okRingBearerOnBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.and [isCreature, .hasDesignation (.named "ringBearer" (.heldBy .object)) (some .you), .in_ battlefield]) = [] := by
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

/-- "The damage can't be prevented": a prohibition over preventing "that damage", the damage in
scope [CR#615.12]. -/
def theDamageCantBePrevented : StaticSpec := .cant .any (.preventDamage .all none none) {}

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAfterDealing :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 4 (target isCreature))),
          .establish theDamageCantBePrevented none ]) = [] := by
  decide

/-- "You gain 3 life. The damage can't be prevented." -/
theorem badTheDamageAfterLifeGain :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .changeLife (.up 3),
          .establish theDamageCantBePrevented none ])
      = [.damageDealtInScope] := by
  decide

def afterDamageDealt : DRS :=
  actionResult (DRS.ofList []) (Action.instruct .this (.dealDamage 4 (target isCreature)))

/-- "This deals 4 damage to target creature. The damage can't be prevented." -/
theorem okTheDamageAnnounced :
    StaticSpec.check afterDamageDealt theDamageCantBePrevented = [] := by
  decide

/-- "The damage can't be prevented." -/
theorem badTheDamageUnannounced :
    StaticSpec.check (DRS.ofList []) theDamageCantBePrevented = [.damageDealtInScope] := by
  decide

/-- *Excruciator*: "Damage that would be dealt by this creature can't be prevented." A
prohibition over the prevention of the damage it names, `cant any (preventDamage .all (damage :=
…))`: prevention effects still apply to that damage, and their additional effects still happen
[CR#615.12]. The prevention is an event head; a self-replacement is not. -/
theorem cantBePreventedIsADeonticOverPreventDamage :
    let cantBePrevented : StaticSpec :=
      cant any
        (preventDamage allDamage (damage := would (by_ thisCreature) (dealDamage anyAmount)))
    Cards.excruciator.card =
        singleFacedCard
          { name := "Excruciator", cost := [generic 6, pip .red, pip .red], types := [.creature],
            subtypes := [avatar], text := [static cantBePrevented],
            power := stat 7, toughness := stat 7 } ∧
      StaticSpec.check (DRS.ofList []) cantBePrevented = [] ∧
      (Action.preventDamage .all none none).eventHeadOk = true :=
  ⟨rfl, by decide, rfl⟩

/-- *Whippoorwill*: "Damage that would be dealt to that creature this turn can't be prevented or
dealt instead to another permanent or player." Its redirection half is `cant any (redirectDamage
…)`, a prohibition over the redirection [CR#614.9]; the redirect target is one permanent or
player, so a plural one is refused. `redirectDamage` is an event head, and the self-replacement
`Action.replace` is not [CR#614.15]. -/
theorem whippoorwillCantIsADeonticOverRedirectDamage :
    let toThatCreature : WouldClause := would any (dealDamage anyAmount (that creature))
    let afterTarget := nextClause (actionResult (DRS.ofList [])
      (establishThroughout (cant any (regeneration (target isCreature))) thisTurn))
    StaticSpec.check afterTarget
        (cant any
          (redirectDamage allDamage (a (and [or [isPermanent, isPlayer], isOther]))
            (damage := toThatCreature))) = [] ∧
      StaticSpec.check afterTarget
        (cant any
          (redirectDamage allDamage (allOf (or [isPermanent, isPlayer]))
            (damage := toThatCreature))) = [.singular] ∧
      (Action.redirectDamage .all .you none).eventHeadOk = true ∧
      Action.eventHeadOk
          (.replace (.dealDamage .anyAmount .any none) (.instead (.draw (.exactlyOf 1))))
        = false := by
  decide

/-- "You may cast spells as though they had flash." -/
theorem okObjectPremiseAtCast :
    StaticSpec.check (DRS.ofList [])
      (.can (.actor .you) (.procedure .cast (bare isSpell) .itsOwnCost none none none) { asThough := (some (.condition (.matches (.pro .bare .many) (.hasKeyword (.the (.named "flash")))))) }) = [] := by
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
    let duration : Action → Option (Option Duration) := fun
      | .sequentially [_, .establish _ d] => some d
      | _ => none
    duration (doAndForbid (instruct .this (.dealDamage 3 (target isCreature))) (regeneration it)) =
        some (some untilEndOfTurn) ∧
      duration (doAndForbid (instruct .this (.dealDamage 3 (target isCreature))) (regeneration it)) ≠
        duration (destroyCantRegenerate (target isCreature)) :=
  ⟨rfl, by intro h; cases h⟩

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
    Timing.check (DRS.ofList []) (.whileDoing (.does (.actor .you) (scry .anyAmount)))
      = [] := by
  decide

/-- "Creatures can't attack unless their controller pays {X} for each attacking creature they
control." The gated cost sits at the deontic clause's own intro, which is where the payer is
introduced. -/
theorem okGatedCostReadsPayer :
    StaticSpec.check [letterB .x]
      (.gatedBy (scaled (.mana [generic 1])
            (times (.letter .x)
              (countOf (.and [isCreature, isAttacking, .hasPossessor .controller they])))) (.actor (allOf isCreature)) (.attack none) {}) = [] := by
  decide

/-- "{X} for each attacking creature": the same scaled cost with no read. -/
theorem okBareScaledCost :
    Cost.check [letterB .x]
      (scaled (.mana [generic 1]) (times (.letter .x) (countOf (.and [isCreature, isAttacking])))) = [] := by
  decide

/-- The same cost written on its own: outside the clause there is no payer to read, and a cost
carries no agent context of its own. -/
theorem badBareCostReadsPayer :
    Cost.check [letterB .x]
      (scaled (.mana [generic 1])
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
      (.cant (.actor (each .isPlayer))
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

/-- "Destroy target creature. It can't be regenerated.": the prohibition's duration is
`some .modifiesPrecedingAction`, neither `none` nor `restOfGame`, so it binds the destroy's event
only. A regeneration shield made after a destroy that did not destroy the creature (it is
indestructible) is therefore not forbidden; that behaviour is the engine's, this pins the term
[CR#608.2c,701.19c]. -/
theorem indestructibleVictimKeepsLaterRegenerationShield :
    (match destroyCantRegenerate (target isCreature) with
      | .sequentially [_, .establish _ d] => some d
      | _ => none) = some (some Duration.modifiesPrecedingAction) ∧
    (some Duration.modifiesPrecedingAction : Option Duration) ≠ none ∧
    (some Duration.modifiesPrecedingAction : Option Duration) ≠ some restOfGame := by
  exact ⟨rfl, (by intro h; cases h), (by intro h; cases h)⟩

/-- The Duration convention [CR#611.2a,608.2c]: a printed "for the rest of the game" is `some
restOfGame` (epic, Screaming Nemesis); a rider that modifies the preceding action is `some
.modifiesPrecedingAction` (Terror, Pinpoint Avalanche); no printed Duration is `none` (Vexing
Shusher, Boseiju). -/
theorem durationConventionWitnesses :
    (match epic with
      | .keyword _ _ [.spell _ (.establish _ d), _] => some d
      | _ => none) = some (some restOfGame) ∧
    (match printedText Cards.screamingNemesis with
      | [_, .triggered _ _ _ _ _ _ _ (.sequentially [_, .doIf _ (.establish _ d) _ _])] => some d
      | _ => none) = some (some restOfGame) ∧
    (match printedText Cards.terror with
      | [.spell _ (.sequentially [_, .establish _ d])] => some d
      | _ => none) = some (some Duration.modifiesPrecedingAction) ∧
    (match printedText Cards.pinpointAvalanche with
      | [.spell _ (.sequentially [_, .establish _ d])] => some d
      | _ => none) = some (some Duration.modifiesPrecedingAction) ∧
    (match printedText Cards.vexingShusher with
      | [_, .activated _ (.establish _ d) _ _ _ _] => some d
      | _ => none) = some none ∧
    (match Cards.boseijuMana with
      | .activated _ (.addMana _ _ [.onSpent _ _ _ (.establish _ d)]) _ _ _ _ => some d
      | _ => none) = some none := by
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

/-! ## Indicative "as though" statements [CR#609.4] -/

/-- Everlasting Torment and Phyrexian Unlife check, their damage lines written as indicative
"as though" statements and their other lines (the life-gain and damage-prevention prohibitions,
the loses-the-game exception) unchanged. -/
theorem okAsThoughSourceHadKeywordCards :
    Cards.everlastingTorment.card.check = [] ∧ Cards.phyrexianUnlife.card.check = [] :=
  ⟨Cards.everlastingTorment.ok, Cards.phyrexianUnlife.ok⟩

/-- The damage lines name the keywords by reference, as a condition on the damage's source
("as though its source had wither"), so they take wither's and infect's own rows
[CR#702.80a,702.90b]. The condition grants nothing: for a source that has the keyword already it
is simply true, and the keyword's result happens once [CR#120.3d,702.80d,702.90f]. -/
theorem asThoughSourceHadKeywordByReference :
    (match printedText Cards.everlastingTorment with
      | [_, _, .static (.asThough subj action cond)] => some (subj, action, cond)
      | _ => none) =
      some (by_ (a isSource), dealDamage anyAmount, holds (is thatSource (hasKeyword wither))) ∧
    (match printedText Cards.phyrexianUnlife with
      | [_, .static (.conditional (.asThough subj action cond) _ _)] => some (subj, action, cond)
      | _ => none) =
      some (by_ (a isSource), dealDamage anyAmount you,
        holds (is thatSource (hasKeyword infect))) := by
  refine ⟨rfl, rfl⟩

/-- "Its source" reads the passive's agent: with no agent in the clause there is no source for
it to read. -/
theorem badAsThoughSourceWithoutAgent :
    StaticSpec.check (DRS.ofList [])
      (asThough any (dealDamage anyAmount) (holds (is thatSource (hasKeyword wither)))) =
      [.anaphor (.word .source) .one 0] := by decide

/-- Whether stepping `spec` records any act it names as permitted. -/
private def recordsPermitted (spec : StaticSpec) : Bool :=
  (stepStaticSpec 64 (DRS.ofList []) spec).1.causedEvents.any (·.permitted)

/-- An indicative "as though" statement is not a permission [CR#101.2]: "this creature crews
Vehicles as though its power were 2 greater" records no permitted crew, where "this creature can
crew Vehicles as though …" does. -/
theorem asThoughIsNotRecordedAsPermitted :
    recordsPermitted
        (asThough thisCreature (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2)) = false ∧
      recordsPermitted
        (can thisCreature (crew (allOf (hasSubtype vehicle))) (asThough := greaterBy .power 2)) =
        true := by
  decide

/-- Hotshot Mechanic and Cloudspire Captain check in the indicative form. -/
theorem okIndicativeCrewAsThough :
    Cards.hotshotMechanic.card.check = [] ∧
      StaticSpec.check (DRS.ofList []) Cards.cloudspireCaptainCrewLine = [] :=
  ⟨Cards.hotshotMechanic.ok, Cards.okCloudspireCaptainCrewLine⟩

/-- Hotshot Mechanic: "… as though its power were 2 greater." -/
theorem okLiteralAsThoughAmount :
    StaticSpec.check (DRS.ofList [])
      (asThough thisCreature (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2)) = [] := by
  decide

/-- "… as though its power were greater by target creature's power.": an as-though amount reads
a value and introduces no referent; a static ability is none of the targeted spells and abilities
[CR#115.1a..115.1d,609.4]. -/
theorem badAnnouncingAsThoughAmount :
    StaticSpec.check (DRS.ofList [])
      (asThough thisCreature (crew (allOf (hasSubtype vehicle)))
        (greaterBy .power (.statOf (.stat .power) (target isCreature)))) = [.bindingless] := by
  decide

/-! ## Modal boxes close (`semantics-v3.md` §1.3, box 3) -/

private def castFromGraveyard : Action :=
  .procedure .cast (a (.and [isCreature, .in_ (graveyardOf .you)])) .itsOwnCost none
    (some (graveyardOf .you)) none

/-- "You may cast a creature card from your graveyard this turn. Exile it." The card the
permission ranges over is hypothetical, closed with the permission's box, as a prohibition's is
("You can't cast creature cards from your graveyard this turn. Exile it."), so "it" finds
nothing. -/
theorem permissionClosesItsBox :
    Action.check (DRS.ofList [])
        (.sequentially [.establish (.can (.actor .you) castFromGraveyard {}) (some thisTurn),
          exile it]) = [.anaphor .bare .one 0] ∧
      Action.check (DRS.ofList [])
        (.sequentially [.establish (.cant (.actor .you) castFromGraveyard {}) (some thisTurn),
          exile it]) = [.anaphor .bare .one 0] := by
  decide

/-- "Target creature attacks a planeswalker each combat if able. Tap that planeswalker." The
planeswalker a requirement ranges over is closed with its box; the target the requirement
announces is not, chosen as the spell is put on the stack [CR#601.2c], so "Tap it" reads it. -/
theorem requirementClosesItsBoxButPublishesItsTarget :
    let req : StaticSpec := .must (.actor (target isCreature)) (.attack (some (a isPlaneswalker))) {}
    Action.check (DRS.ofList [])
        (.sequentially [.establish req (some thisTurn),
          .setStatus .tapped (that (.type .planeswalker))]) =
        [.anaphor (.word (.type .planeswalker)) .one 0] ∧
      Action.check (DRS.ofList [])
        (.sequentially [.establish req (some thisTurn), .setStatus .tapped it]) = [] := by
  decide

/-- "Target creature can't attack a planeswalker this turn unless its controller pays {2}. Tap
that planeswalker." A prohibition gated by a payment closes its box as a prohibition does,
publishing its target. -/
theorem gatedProhibitionClosesItsBoxButPublishesItsTarget :
    let gate : StaticSpec :=
      .gatedBy (.mana [generic 2]) (.actor (target isCreature)) (.attack (some (a isPlaneswalker))) {}
    Action.check (DRS.ofList [])
        (.sequentially [.establish gate (some thisTurn),
          .setStatus .tapped (that (.type .planeswalker))]) =
        [.anaphor (.word (.type .planeswalker)) .one 0] ∧
      Action.check (DRS.ofList [])
        (.sequentially [.establish gate (some thisTurn), .setStatus .tapped it]) = [] := by
  decide

/-- A prohibition that modifies the preceding action reads that action's own objects by last
known information [CR#608.2h]: Nekrataal's "Destroy target creature. That creature can't be
regenerated." reads the destroyed creature, but after "Exile target creature." or "Sacrifice a
creature." and then "Destroy target artifact.", "that creature" finds no creature the destroy
acted on, and is refused. -/
theorem modifyingProhibitionReadsOnlyThePrecedingActionsObjects :
    let rider := fun (before : List Action) (destroyed : Predicate) =>
      Action.check (DRS.ofList []) (.sequentially (before ++
        [doUnderProhibition (destroy (target destroyed)) (regeneration (that (.type .creature)))]))
    rider [] isCreature = [] ∧
      rider [exile (target isCreature)] isArtifact =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      rider [sacrifice (a isCreature)] isArtifact =
        [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- A universal domain a prohibition publishes keeps its number: "each creature" is singular, as
an action's is, so "Each creature can't block this turn. Tap it." reads "it" as it does in "Tap
each creature. Untap it."; "all creatures" is plural, so "it" finds nothing. -/
theorem publishedEachDomainIsSingular :
    let barred := fun (creatures : NounPhrase) => Action.check (DRS.ofList [])
      (.sequentially [.establish (.cant (.actor creatures) (.block .any) {}) (some thisTurn),
        .setStatus .tapped it])
    barred (each isCreature) = [] ∧ barred (allOf isCreature) = [.anaphor .bare .one 0] ∧
      Action.check (DRS.ofList [])
        (.sequentially [.setStatus .tapped (each isCreature), .setStatus .untapped it]) = [] := by
  decide

/-- A deontic rule and an event read their clause through one spine (`stepActionClauseWith`), so
a performer of the wrong kind is refused alike in both: a player casts a spell [CR#601.2,701.5a],
so "creatures can't cast spells" is refused as "whenever a creature casts a spell" is. -/
theorem prohibitionAndEventRefuseANonPlayerCasterAlike :
    StaticSpec.check (DRS.ofList []) (.cant (.actor (allOf isCreature)) (cast (a isSpell)) {}) =
        [.kindMismatch .player .object] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (cast (a isSpell))) =
        [.kindMismatch .player .object] := by
  decide

/-- A draw checks its own performer as it steps [CR#121.1], so a clause over one refuses a
non-player once, not twice: a prohibition and an event alike. -/
theorem drawRefusesANonPlayerOnceInEveryClause :
    StaticSpec.check (DRS.ofList []) (.cant (.actor (allOf isCreature)) (draw 1) {}) =
        [.kindMismatch .player .object] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (draw 1)) =
        [.kindMismatch .player .object] := by
  decide

/-- A deontic clause constrains any Zone Change, as an event clause watches one:
"can't be returned to its owner's hand" and "can't be put into exile" are admitted, as "can't enter"
is. -/
theorem prohibitionAdmitsAnyZoneChange :
    StaticSpec.check (DRS.ofList []) (.cant .any (returnToHand thisCreature) {}) = [] ∧
      StaticSpec.check (DRS.ofList []) (.cant .any (putInto thisCreature exileZone) {}) = [] := by
  decide

end Semantics.Proofs.Deontic
