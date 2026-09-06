import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Damage

Port of `idris/src/Experimental/Proofs/Damage.idr`: the pins of the Damage family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Damage

/-- "Target creature fights target creature." -/
theorem okFightCreatures :
    Instruction.check [] (.fight (target creature) (target creature)) = [] := by decide

/-- "Two target creatures fight target creature." -/
theorem badFightGroup :
    Instruction.check [] (.fight (.described (.target (exactly 2)) creature) (target creature))
      = [.singular] := by
  decide

/-- "This deals 2 damage to any target and 1 damage to any other target." -/
theorem okAnyOtherTarget :
    Instruction.check []
      (.sequence
        [ .dealDamage .this (.lit 2) (target anyTarget),
          .dealDamage .this (.lit 1) (target anyOtherTarget) ]) = [] := by
  decide

/-- "This deals 1 damage to any other target." The Idris pin discharged both obligations of
`anyOtherTarget` with one hypothesis; the list form names both. -/
theorem badOther :
    Instruction.check [] (.dealDamage .this (.lit 1) (target anyOtherTarget))
      = [.anyTargeted (.join .object (.join .object (.join .object .player))), .otherAnchored] := by
  decide

/-- "Tap target creature. It gets −1/−1 until end of turn." (U+2212 minus signs: an ASCII
hyphen before the slash would open a comment.) -/
theorem okIt :
    Instruction.check []
      (.sequence
        [ .setStatus .tapped (target creature),
          get it (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "Target creature fights target creature. Tap it." -/
theorem badIt :
    Instruction.check []
      (.sequence [.fight (target creature) (target creature), .setStatus .tapped it])
      = [.anaphor .bare .one 2] := by
  decide

/-- "Tap target creature." -/
theorem okTapBattlefield :
    Instruction.check [] (.setStatus .tapped (target creature)) = [] := by decide

/-- "This deals 3 damage to each creature. Tap it." The Idris pin refutes the anaphor; an
unresolved `it` has no zone either, so the status law cascades. -/
theorem badTheyIt :
    Instruction.check []
      (.sequence [.dealDamage .this (.lit 3) (each creature), .setStatus .tapped it])
      = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

/-- "Choose two target creatures. Tap them." -/
theorem okThem :
    Instruction.check []
      (.sequence [choose (.described (.target (exactly 2)) creature), .setStatus .tapped them])
      = [] := by
  decide

/-- "Choose two target creatures. Choose two target creatures. Tap them." -/
theorem badThemAmbig :
    Instruction.check []
      (.sequence
        [ choose (.described (.target (exactly 2)) creature),
          choose (.described (.target (exactly 2)) creature),
          .setStatus .tapped them ]) = [.anaphor .bare .many 2] := by
  decide

theorem badInnerAmbig :
    Instruction.check []
      (.sequence
        [ .fight (target (.and [creature, .hasPossessor .controller anOpponent]))
            (target (.and [creature, .hasPossessor .controller anOpponent])),
          loseLife (.lit 1) (agent := (that .player)) ]) = [.anaphor (.word .player) .one 2] := by
  decide

/-- "A creature doesn't untap during your untap step." -/
theorem okUntapLockBattlefield :
    Ability.check [] (.static (doesntUntap (a creature) (some .you))) = [] := by decide

theorem badUntapLockGraveyard :
    Ability.check []
      (.static (doesntUntap (a (.and [creature, .inZone (graveyardOf .you)])) (some .you)))
      = [.deedFits] := by
  decide

/-- "Target creature you control fights target creature you don't control." -/
theorem okFightControlledCreatures :
    Instruction.check [] (.fight (target creatureYouControl) (target creatureYouDontControl))
      = [] := by
  decide

/-- "Target creature card in your graveyard fights target creature." -/
theorem badFightGraveyard :
    Instruction.check []
      (.fight (target (.and [creature, .inZone (graveyardOf .you)])) (target creature))
      = [.zoneIs .battlefield] := by
  decide

/-- "Target land fights target creature you don't control.": a land an effect has made a
creature fights, and one that is no longer a creature simply does not [CR#205.1b,701.14b]. -/
theorem okFightLand :
    Instruction.check [] (.fight (target land) (target creatureYouDontControl)) = [] := by
  decide

/-- "Target permanent fights target creature." [CR#701.14a] -/
theorem badFightPermanent :
    Instruction.check [] (.fight (target permanent) (target creature))
      = [.deedNounOk (.core .attack)] := by
  decide

/-- "This deals 3 damage to target creature." -/
theorem okDamageCreature :
    Instruction.check [] (.dealDamage .this (.lit 3) (target creature)) = [] := by decide

/-- "Destroy target creature. This deals 3 damage to it." -/
theorem badDamageGraveyardCard :
    Instruction.check []
      (.sequence [destroy (target creature), .dealDamage .this (.lit 3) it])
      = [.damageRecipient] := by
  decide

/-- "This deals 1 damage to a color." -/
theorem badDamageToColor :
    Instruction.check [] (.dealDamage .this (.lit 1) (a (quality .color)))
      = [.damageRecipient] := by
  decide

/-- "This deals 1 damage to target artifact." -/
theorem badDamageArtifact :
    Instruction.check [] (.dealDamage .this (.lit 1) (target artifact)) = [.damageRecipient] := by
  decide

/-- "... loses 1 life for each attacking creature. You gain that much life." -/
theorem okThatMuchBound :
    Instruction.check []
      (.sequence
        [ loseLife
            (forEach 1 (.and [attacking, creature, .hasPossessor .controller .you])) (agent :=
                (target .opponent)),
          gainLife .thatMuch (agent := .you) ]) = [] := by
  decide

/-- "This deals that much damage to any target." -/
theorem badThatMuchUnbound :
    Instruction.check [] (.dealDamage .this .thatMuch (target anyTarget))
      = [.quantOutcomeInScope 0] := by
  decide

theorem badThatMuchAmbig :
    Instruction.check []
      (.sequence
        [ .dealDamage .this (.lit 3) (target anyTarget),
          loseLife (.lit 2) (agent := .you),
          gainLife .thatMuch (agent := .you) ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "This deals 3 damage to target creature. Destroy it." -/
theorem okDestroyDamagedCreature :
    Instruction.check []
      (.sequence [.dealDamage .this (.lit 3) (target creature), destroy it]) = [] := by
  decide

/-- "This deals 3 damage to any target. Destroy it." -/
theorem badDestroyAnyTargetRemention :
    Instruction.check []
      (.sequence [.dealDamage .this (.lit 3) (target anyTarget), destroy it])
      = [.zoneFits] := by
  decide

/-- "This deals 1 damage to target creature." -/
theorem okDamageOneToCreature :
    Instruction.check [] (.dealDamage .this (.lit 1) (target creature)) = [] := by decide

/-- "This deals 1 damage to this spell." -/
theorem badDamageThis :
    Instruction.check [] (.dealDamage .this (.lit 1) .this) = [.damageRecipient] := by decide

/-- "creature you control or artifact you control" -/
theorem okStructuredDisjunction :
    Predicate.check .object []
      (.or [ .and [creature, .hasPossessor .controller .you],
             .and [artifact, .hasPossessor .controller .you] ]) = [] := by
  decide

/-- "other creature or land" -/
theorem badOtherInOr :
    Predicate.check .object
      [⟨.target, .one, .object (some .creature) (some .battlefield) none none none⟩]
      (.or [.and [creature, .other], land]) = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage to target creature or planeswalker." [CR#120.1] -/
theorem okDamageDisjunctHead :
    Instruction.check []
      (.dealDamage .this (.lit 2) (target (.or [creature, .hasType .planeswalker]))) = [] := by
  decide

/-- "This deals 2 damage to target artifact or enchantment." -/
theorem badDamageDisjunctHead :
    Instruction.check [] (.dealDamage .this (.lit 2) (target (.or [artifact, enchantment])))
      = [.damageRecipient] := by
  decide

/-- "attacking or blocking creature on the battlefield" -/
theorem okAttackingOrBlockingOnBattlefield :
    Predicate.check .object [] (.and [creature, .or [attacking, blocking], .inZone battlefield])
      = [] := by
  decide

/-- "attacking or blocking creature in your graveyard" -/
theorem badAttackingOrBlockingInGraveyard :
    Predicate.check .object [] (.and [creature, .or [attacking, blocking], .inZone graveyard])
      = [.zoneCoherent] := by
  decide

/-- "This deals 2 damage to target creature. Tap it." -/
theorem okTapDamagedCreature :
    Instruction.check []
      (.sequence [.dealDamage .this (.lit 2) (target creature), .setStatus .tapped it])
      = [] := by
  decide

/-- "You gain 2 life if you control a creature. Tap it." The Idris pin refutes the anaphor;
the unresolved `it` has no zone either. -/
theorem badConditionAntecedent :
    Instruction.check []
      (.sequence
        [ .doOnlyIf (gainLife (.lit 2) (agent := .you)) (exists_ creatureYouControl) none,
          .setStatus .tapped it ]) = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

/-- "You may sacrifice a creature. If you don't, exile it." -/
theorem badIfNotReadsMayBody :
    Instruction.check [] (.offer (sacrifice (a creature) (agent := .you)) none (some (exile it))
        (agent := .you))
      = [.anaphor .bare .one 0] := by
  decide

/-- "other than this creature" -/
theorem okComplementThisCreature :
    Predicate.check .object [] (.otherThan thisCreature) = [] := by decide

/-- "other than a creature" -/
theorem badComplementAnchorAnnounces :
    Predicate.check .object [] (.otherThan (a creature)) = [.complementAnchor] := by decide

/-- "other than up to two target creatures" -/
theorem badPluralComplementAnchor :
    Predicate.check .object [] (.otherThan (.described (.target (upTo 2)) creature))
      = [.complementAnchor] := by
  decide

/-- "each creature other than this creature" -/
theorem okComplementSameHead :
    Instruction.check []
      (.dealDamage .this (.lit 1) (each (.and [creature, .otherThan thisCreature]))) = [] := by
  decide

/-- "each creature other than this land" -/
theorem badComplementCrossHead :
    Instruction.check [] (.dealDamage .this (.lit 1) (each (.and [creature, .otherThan thisLand])))
      = [.otherAnchored] := by
  decide

/-- "each creature other than this creature other than this creature" -/
theorem badDoubleComplement :
    Instruction.check []
      (.dealDamage .this (.lit 1)
        (each (.and [creature, .otherThan thisCreature, .otherThan thisCreature])))
      = [.otherAnchored] := by
  decide

/-- "each other creature other than this creature" -/
theorem badOtherAndComplement :
    Instruction.check []
      (.sequence
        [ .setStatus .tapped (target creature),
          .dealDamage .this (.lit 1) (each (.and [creature, .other, .otherThan thisCreature])) ])
      = [.otherAnchored] := by
  decide

/-- "creature or land" -/
theorem okDisjunctWithoutAComplement :
    Predicate.check .object [] (.or [creature, land]) = [] := by decide

/-- "creature other than this creature, or land" -/
theorem badComplementInOr :
    Predicate.check .object [] (.or [.and [creature, .otherThan thisCreature], land])
      = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage divided as you choose among two target creatures." -/
theorem okDivideAmongTargets :
    Instruction.check []
      (dealDivided .this (.lit 2) (.described (.target (oneThrough 2)) creature)) = [] := by
  decide

/-- "This deals 2 damage divided as you choose among each creature." -/
theorem badDivideAmongDescription :
    Instruction.check [] (dealDivided .this (.lit 2) (each creature)) = [.groupMention] := by
  decide

/-- "Put one of them into your hand and the rest into your graveyard." -/
theorem okRestAfterPart :
    Instruction.check []
      (.sequence
        [ lookAt (topSlice (.lit 4)),
          move (someOf (exactly 1) them) hand,
          move (theRest .object) graveyard ]) = [] := by
  decide

/-- "Put the rest into your graveyard." -/
theorem badRestWithoutGroup :
    Instruction.check [] (move (theRest .object) graveyard) = [.theRestFits .object] := by decide

/-- "Look at the top four cards of your library. Put the rest on the bottom." -/
theorem badRestWithoutPart :
    Instruction.check []
      (.sequence [lookAt (topSlice (.lit 4)), move (theRest .object) onBottom])
      = [.theRestFits .object] := by
  decide

theorem badRestDisposedTwice :
    Instruction.check []
      (.sequence
        [ lookAt (topSlice (.lit 4)),
          move (someOf (exactly 1) them) hand,
          move (theRest .object) onBottom,
          move (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

theorem badRestOverTwoAnnouncements :
    Instruction.check []
      (.sequence
        [ .fight (target creatureYouControl) (target creatureYouDontControl),
          move (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

/-- "This creature deals 2 damage to each opponent." -/
theorem okEachOpponentDamage :
    Instruction.check [] (.dealDamage thisCreature (.lit 2) (each .opponent)) = [] := by decide

/-- "This creature deals 2 damage to your opponents." -/
theorem badPluralPlayerDamageRecipient :
    Instruction.check [] (.dealDamage thisCreature (.lit 2) (.playerGroup .yourOpponents))
      = [.perMember] := by
  decide

/-- "Prevent all damage that would be dealt to any player or permanent." -/
theorem okPreventDealtToPermanent :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.prevent .all none) .repeatedly) = [] := by
  decide

/-- "Prevent all damage that would be dealt to this this turn." -/
theorem badPreventedBareThis :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .this) (.prevent .all none) .repeatedly)
      = [.damageRecipient] := by
  decide

/-- "Prevent all damage that would be dealt to target artifact this turn." -/
theorem badPreventDealtToArtifact :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient (target artifact)) (.prevent .all none)
        .repeatedly) = [.damageRecipient] := by
  decide

/-- "… is dealt to target attacking creature instead." -/
theorem okRedirectToSingleCreature :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (target (.and [creature, attacking]))) .repeatedly) = [] := by
  decide

/-- "All damage that would be dealt to you is dealt to target artifact instead." -/
theorem badRedirectToArtifact :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you) (.redirect .all (target artifact))
        .repeatedly) = [.damageRecipient] := by
  decide

theorem badRedirectToPlural :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (allOf creatureYouControl)) .repeatedly) = [.singular] := by
  decide

/-- "… If damage from a red source is prevented this way, you gain 3 life." -/
def preventedFromSourceAnnounced : StaticSpec :=
  .damageRule .any .unattributed (.toRecipient .you)
    (.prevent .all
      (some (doIf (.preventedFromSource (.and [source, .colorIs .red])) (gainLife (.lit 3) (agent :=
          .you)))))
    .repeatedly

theorem okPreventedFromSourceAnnounced :
    StaticSpec.check [] preventedFromSourceAnnounced = [] := by decide

/-- "Prevent all damage that would be dealt to you this turn. If damage from a red source is
prevented this way, you gain 3 life." -/
theorem okPreventedFromSourceInAClause :
    Instruction.check [] (.establish preventedFromSourceAnnounced (some .thisTurn)) = [] := by
  decide

/-- "If damage from a red source is prevented this way, you gain 3 life." -/
theorem badPreventedFromSourceUnannounced :
    Instruction.check []
      (doIf (.preventedFromSource (.and [source, .colorIs .red])) (gainLife (.lit 3) (agent :=
          .you)))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… You gain life equal to the damage prevented this way." -/
def preventedThisWayAnnounced : StaticSpec :=
  .damageRule .any .unattributed (.toRecipient .you)
    (.prevent .all (some (gainLife preventedThisWay (agent := .you)))) .repeatedly

theorem okPreventedThisWayAnnounced : StaticSpec.check [] preventedThisWayAnnounced = [] := by
  decide

/-- "Prevent all damage that would be dealt to you this turn. You gain life equal to the damage
prevented this way." -/
theorem okPreventedThisWayInAClause :
    Instruction.check [] (.establish preventedThisWayAnnounced (some .thisTurn)) = [] := by
  decide

theorem badPreventedThisWayAfterDamage :
    Instruction.check []
      (.sequence
        [ .dealDamage .this (.lit 3) (target creature),
          .changeLife (.up preventedThisWay) (agent := .you) ]) = [.outcomeInScope .damagePrevented
              0] := by
  decide

/-- "You gain life equal to the damage prevented this way." -/
theorem badPreventedThisWayUnannounced :
    Instruction.check [] (.changeLife (.up preventedThisWay) (agent := .you))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… They gain 2 life for each card less than two they drew this way." -/
theorem okShortOfCeilingAnnounced :
    Instruction.check []
      (.sequence
        [ offer (draw (.upTo (.lit 2)) (agent := they)) (agent := (each .anyPlayer)),
          gainLife (times (.lit 2) shortOfCeiling) (agent := they) ]) = [] := by
  decide

/-- "You gain 2 life for each card less than two you draw this way." -/
theorem badShortOfCeilingUnannounced :
    Instruction.check [] (gainLife (times (.lit 2) shortOfCeiling) (agent := .you))
      = [.outcomeInScope .ceilingShortfall 0] := by
  decide

/-- "Prevent the next 3 damage that would be dealt to you this turn." -/
theorem okShieldRepeatedly :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you) (.prevent (.shield (.lit 3)) none)
        .repeatedly) = [] := by
  decide

theorem badShieldNextTimeOnly :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you) (.prevent (.shield (.lit 3)) none)
        .nextTimeOnly) = [.damageOpUse] := by
  decide

theorem badShieldSizedByItsOwnPrevention :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you)
        (.prevent (.shield preventedThisWay) none) .repeatedly)
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "This creature deals 3 damage to target creature." -/
theorem okDamageToCreature :
    Instruction.check [] (.dealDamage .this (.lit 3) (target creature)) = [] := by decide

/-- "This creature deals 3 damage to target source." -/
theorem badDamageToSource :
    Instruction.check [] (.dealDamage .this (.lit 3) (target source)) = [.damageRecipient] := by
  decide

/-- "…target nonartifact…" -/
theorem okNonartifact : Predicate.check .object [] (.not artifact) = [] := by decide

/-- "…target nonsource…" -/
theorem badNonsource : Predicate.check .object [] (.not source) = [.negatable] := by decide

/-- "If damage would be dealt to you, it's dealt to target creature you control instead." -/
theorem okRedirectToOne :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (target creatureYouControl)) .repeatedly) = [] := by
  decide

theorem badRedirectToGroup :
    StaticSpec.check []
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
        .repeatedly) = [.singular] := by
  decide

/-- "Target opponent loses 1 life. You gain that much life." -/
theorem okThatMuchAfterLifeLoss :
    Instruction.check []
      (.sequence [loseLife (.lit 1) (agent := (target .opponent)), gainLife .thatMuch (agent :=
          .you)]) = [] := by
  decide

/-- "If a source would deal damage to a player or permanent, it deals double that damage
instead." -/
theorem okScaleDoubled :
    StaticSpec.check []
      (.damageRule .any (.dealtBy (a source)) (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.scale (.multiplied .doubled)) .repeatedly) = [] := by
  decide

/-- "… it deals that much damage plus that much instead." -/
theorem badScaleShiftByThatMuch :
    StaticSpec.check []
      (.damageRule .any (.dealtBy (a source)) (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.scale (.shifted .up .thatMuch)) .repeatedly) = [.quantOutcomeInScope 0] := by
  decide

theorem badScaleToArtifact :
    StaticSpec.check []
      (.damageRule .any (.dealtBy (a source)) (.toRecipient (target artifact))
        (.scale (.multiplied .doubled)) .repeatedly) = [.damageRecipient] := by
  decide

/-- "Whenever this creature is dealt damage, it deals that much damage to any target." -/
theorem okThatMuchAfterDamageEvent :
    Ability.check []
      (whenever (.isDealtDamage .any thisCreature)
        (.dealDamage thisCreature .thatMuch (target anyTarget))) = [] := by
  decide

theorem badPreventedThisWayAfterDamageEvent :
    Ability.check []
      (whenever (.isDealtDamage .any thisCreature)
        (.dealDamage it preventedThisWay (target anyTarget)))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

theorem badThatMuchAfterDeath :
    Ability.check []
      (whenever (.dies (a creature))
        (.dealDamage thisCreature .thatMuch (target anyTarget))) = [.quantOutcomeInScope 0] := by
  decide

/-- "… That creature deals damage equal to its power to this creature." -/
theorem okThatCreatureAfterDamage :
    Instruction.check []
      (.sequence
        [ dealDamageOwnPower [] thisCreature (target creature),
          .dealDamage (that (.type .creature)) (powerOf it) thisCreature ]) = [] := by
  decide

/-- "When this creature dies, it deals 1 damage to target creature. That creature's controller
loses 1 life." -/
theorem okThatCreatureAfterTargetedDamage :
    Ability.check []
      (when (.dies thisCreature)
        (.sequence
          [ .dealDamage thisCreature (.lit 1) (target creature),
            loseLife (.lit 1) (agent := (controllerOf (that (.type .creature)))) ])) = [] := by
  decide

theorem badThatCreatureIsDamagedSelf :
    Ability.check []
      (whenever (.isDealtDamage .any thisCreature)
        (.dealDamage (that (.type .creature)) .thatMuch (target anyTarget)))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Prevent all damage sources of the last chosen color would deal to you." -/
def preventLastChosenColor : StaticSpec :=
  .damageRule .any (.dealtBy (allOf (.and [source, ofTheLastChosen .color]))) (.toRecipient .you)
    (.prevent .all none) .repeatedly

theorem okLastChosenAfterChooser :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static (.entryChoice thisEnchantment (.quality .color) none .openly),
                .static preventLastChosenColor ] } }) = [] := by
  decide

theorem badLastChosenBeforeChooser :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static preventLastChosenColor,
                .static (.entryChoice thisEnchantment (.quality .color) none .openly) ] } })
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

theorem badLastChosenWrongSort :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static
                  (.entryChoice thisEnchantment (.quality (.subtype .creature)) none .openly),
                .static preventLastChosenColor ] } })
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

/-- "sources of the last chosen color", a colour choice standing -/
theorem okLastChosenColorRead :
    Predicate.check .object [⟨.a, .one, .quality .color⟩] (ofTheLastChosen .color)
      = [] := by
  decide

/-- "sources of the last chosen color" -/
theorem badLastChosenColorNoChooser :
    Predicate.check .object [] (ofTheLastChosen .color)
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

/-- "the greatest life total among all players" -/
theorem okPlayerAggregate :
    Amount.check [] (.aggregate .max (.playerStat .lifeTotal) (allOf .anyPlayer)) = [] := by decide

/-- "the total power of target creature" -/
theorem badSingularAggregate :
    Amount.check [] (.aggregate .sum (.stat .power) (target creature)) = [.plural] := by decide

/-- "the greatest life total among all creatures" -/
theorem badAggregateWrongSort :
    Amount.check [] (.aggregate .max (.playerStat .lifeTotal) (allOf creature))
      = [.projScope .object] := by
  decide

/-- "1-20 | Draw a card." -/
theorem okLiteralRollRow :
    Instruction.check []
      (.sequence [rollDice 1 20 (agent := .you), .applyResultsTable [⟨fromTo 1 20, draw (.lit 1)
          (agent := .you)⟩]])
      = [] := by
  decide

/-- "up to X | Draw a card." -/
theorem badAmountRollRow :
    Instruction.check []
      (.sequence
        [rollDice 1 20 (agent := .you), .applyResultsTable [⟨.upToOf (.letter .x), draw (.lit 1)
            (agent := .you)⟩]])
      = [.quantLiteral] := by
  decide

theorem badCreatureHalfRead :
    Instruction.check []
      (.sequence
        [ .dealDamage .this (.lit 3) (target (.or [.hasType .planeswalker, .anyPlayer])),
          discard
            (a (.inZone hand)) (agent := (.eitherOf (.pro (.unionHalf .player) .one .whole)
              (controllerOf (that (.type .creature))))) ]) = [.anaphor (.word (.type .creature))
                  .one 0] := by
  decide

/-- "This deals 3 damage to any target." -/
theorem okJoinDamageRecipient :
    Instruction.check [] (.dealDamage .this (.lit 3) (target anyTarget)) = [] := by decide

/-- "This deals 3 damage to a land or a land." -/
theorem badSameKindJoinDamage :
    Instruction.check [] (.dealDamage .this (.lit 3) (a (.or [land, land])))
      = [.damageRecipient] := by
  decide

/-- "Create a 1/1 white Soldier creature token." -/
theorem okSoldierToken :
    Instruction.check [] (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]))
      = [] := by
  decide

/-- "Create a 1/1 black Zombie artifact token.": each subtype is correlated to a card type the
object has [CR#205.3c] and Zombie is a creature type [CR#205.3m]. -/
theorem badZombieArtifactToken :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { colors := [.black], types := [.artifact], subtypes := [creatureType "Zombie"],
            power := some (.lit 1), toughness := some (.lit 1) } }) = [.subsFitLine] := by
  decide

/-- "Create a white Soldier creature token.": a creature has power and toughness [CR#208.1],
and a token has only the characteristics its creating ability defines [CR#111.3]. -/
theorem badCreatureTokenNoPt :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Soldier"] } })
      = [.tokenPtOk] := by
  decide

/-- "Create a 1/1 white token.": a token represents a permanent [CR#111.1], which carries a
permanent card type [CR#110.4a]. -/
theorem badTypelessToken :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { colors := [.white], power := some (.lit 1), toughness := some (.lit 1) } })
      = [.tokenTyped] := by
  decide

/-- "You gain life equal to your life total." -/
theorem okSingularLifeTotalRead : Amount.check [] (lifeTotalOf .you) = [] := by decide

/-- "You gain life equal to your opponents' life totals." -/
theorem badPluralLifeTotalRead :
    Amount.check [] (lifeTotalOf (.playerGroup .yourOpponents)) = [.singular] := by decide

/-- "You gain life equal to each player's life total." -/
theorem badDistributiveLifeTotalRead :
    Amount.check [] (lifeTotalOf (each .anyPlayer)) = [.singular] := by decide

/-- "Destroy target creature." -/
theorem okDestroyCreature : Instruction.check [] (destroy (target creature)) = [] := by decide

/-- "Destroy target source." -/
theorem badDestroySource : Instruction.check [] (destroy (target source)) = [.zoneFits] := by
  decide

/-- "creature that was dealt combat damage by this creature this turn" [CR#120.1] -/
theorem okCombatDamageComplement :
    Predicate.check .object []
      (.happenedTo (.mk .combatDamage .thisTurn (some (.involving thisCreature)))) = [] := by
  decide

/-- "creature that was dealt combat damage by a color this turn" -/
theorem badCombatDamageComplement :
    Predicate.check .object []
      (.happenedTo (.mk .combatDamage .thisTurn (some (.involving (a (quality .color))))))
      = [.lookbackComplement] := by
  decide

/-- "Target creature gets +1/+1 until end of turn." -/
theorem okGetsCreature :
    Instruction.check []
      (get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Target source gets +1/+1 until end of turn." [CR#609.7a] -/
theorem badGetsSource :
    Instruction.check []
      (get (target source) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
      = [.zoneIs .battlefield] := by
  decide

end Semantics.Proofs.Damage
