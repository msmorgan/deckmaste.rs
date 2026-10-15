import Semantics.Macros.Declared
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
    Instruction.check (DRS.ofList []) (instruct (target creature) (fight (target creature))) = [] := by decide

private def fightParticipantPairs : Instruction → List (BindingId × BindingId)
  | .ownScope _ (.enact _ (.doIf _ (.simultaneously [
      .dealDamage (.statOf _ (.laterMention source _)) (.inCaller (.laterMention recipient _)) _,
      .instruct (.laterMention reverseSource _) (.dealDamage _ (.laterMention reverseRecipient _) _)]) none) _ _) =>
    [(source, recipient), (reverseSource, reverseRecipient)]
  | _ => []

theorem fightUsesTheTwoBoundParticipantsInOppositeDirections :
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (instruct (.firstMention "fight.test.source" (target creature))
        (fight (.firstMention "fight.test.other" (target creature))))).1
    (identityBinding "fight.source" after).map (·.id) = some 0 ∧
      (identityBinding "fight.0" after).map (·.id) = some 2 ∧
      (identityBinding "fight.test.source" after).map (·.id) = some 0 ∧
      (identityBinding "fight.test.other" after).map (·.id) = some 2 ∧
      fightParticipantPairs (fight (target creature)) =
        [("fight.source", "fight.0"), ("fight.0", "fight.source")] := by decide

theorem declaredFightUsesTheSameBoundParticipantDefinition :
    Declared.fight (target creature) = fight (target creature) := by rfl

/-- "Target creature fights two target creatures.": the group is the other creature, refused by
the singular power read and the per-member damage check. Handed the deed, a group performer
distributes it instead (`BindingIdentity.okFightHandedToEachOfAGroup`). -/
theorem badFightGroup :
    Instruction.check (DRS.ofList []) (instruct (target creature) (fight (.described (.target (exactly 2)) creature)))
      = [.perMember, .singular] := by
  decide

/-- "This deals 2 damage to any target and 1 damage to any other target." -/
theorem okAnyOtherTarget :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ (Instruction.instruct .this (.dealDamage (.lit 2) (target anyTarget) .any)),
          (Instruction.instruct .this (.dealDamage (.lit 1) (target anyOtherTarget) .any)) ]) = [] := by
  decide

/-- "This deals 1 damage to any other target." The Idris pin discharged both obligations of
`anyOtherTarget` with one hypothesis; the list form names both. -/
theorem badOther :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 1) (target anyOtherTarget) .any))
      = [.anyTargeted (.join .object .player), .otherAnchored] := by
  decide

/-- "Tap target creature. It gets −1/−1 until end of turn." (U+2212 minus signs: an ASCII
hyphen before the slash would open a comment.) -/
theorem okIt :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target creature),
          get it (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "Target creature fights target creature. Tap it." -/
theorem fightSourceIsSelectedBySubjectPriority :
    let combat := instruct (target creature) (fight (target creature))
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) combat).1
    Instruction.check (DRS.ofList []) (.sequentially [combat, .setStatus .tapped it]) = [] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Tap target creature." -/
theorem okTapBattlefield :
    Instruction.check (DRS.ofList []) (.setStatus .tapped (target creature)) = [] := by decide

theorem damageSourceRemainsTheSingularAntecedent :
    let damage := Instruction.instruct .this (.dealDamage (.lit 3) (each creature) .any)
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) damage).1
    Instruction.check (DRS.ofList []) (.sequentially [damage, .setStatus .tapped it]) = [] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.kind) = some .object := by decide

/-- "Choose two target creatures. Tap them." -/
theorem okThem :
    Instruction.check (DRS.ofList [])
      (.sequentially [choose (.described (.target (exactly 2)) creature), .setStatus .tapped them])
      = [] := by
  decide

/-- "Choose two target creatures. Choose two target creatures. Tap them." -/
theorem recentSecondChosenGroupIsSelected :
    let choices := .sequentially
      [choose (.described (.target (exactly 2)) creature),
        choose (.described (.target (exactly 2)) creature)]
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) choices).1
    Instruction.check (DRS.ofList []) (.sequentially
      [choose (.described (.target (exactly 2)) creature),
        choose (.described (.target (exactly 2)) creature), .setStatus .tapped them]) = [] ∧
      (firstReach .bare .many (nextClause after)).map (·.id) = some 1 ∧
      (after.findById? 1).map (·.ty) = some [.creature] := by decide

theorem recentSecondOpponentIsSelected :
    let combat := instruct (target (.and [creature, .hasPossessor .controller anOpponent]))
      (fight (target (.and [creature, .hasPossessor .controller anOpponent])))
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) combat).1
    Instruction.check (DRS.ofList [])
      (.sequentially [combat, instruct (that .player) (loseLife (.lit 1))]) = [] ∧
      (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) = [3] ∧
      (after.findById? 3).map (·.kind) = some .player := by
  decide

/-- "A creature doesn't untap during your untap step." -/
theorem okUntapLockBattlefield :
    Ability.check (DRS.ofList []) (.static (doesntUntap (a creature) (some .you))) = [] := by decide

theorem badUntapLockGraveyard :
    Ability.check (DRS.ofList [])
      (.static (doesntUntap (a (.and [creature, .inZone (graveyardOf .you)])) (some .you)))
      = [.deedFits] := by
  decide

/-- "Target creature you control fights target creature you don't control." -/
theorem okFightControlledCreatures :
    Instruction.check (DRS.ofList []) (instruct (target creatureYouControl) (fight (target creatureYouDontControl)))
      = [] := by
  decide

/-- "Target creature card in your graveyard fights target creature.": a card in a graveyard is
not a permanent [CR#110.1]. This fixture refuses handing it the fight action, whose
participants must remain battlefield creatures [CR#701.14b]. -/
theorem badFightGraveyard :
    Instruction.check (DRS.ofList [])
      (instruct (target (.and [creature, .inZone (graveyardOf .you)])) (fight (target creature)))
      = [.handoffPerformer] := by
  decide

/-- "Target land fights target creature you don't control.": a land an effect has made a
creature fights, and one that is no longer a creature simply does not [CR#205.1b,701.14b]. -/
theorem okFightLand :
    Instruction.check (DRS.ofList []) (instruct (target land) (fight (target creatureYouDontControl))) = [] := by
  decide

/-- "Target permanent fights target creature." [CR#701.14a] -/
theorem okFightPermanentUnderCreatureGuard :
    Instruction.check (DRS.ofList []) (instruct (target permanent) (fight (target creature)))
      = [] := by
  decide

/-- "This deals 3 damage to target creature." -/
theorem okDamageCreature :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 3) (target creature) .any)) = [] := by decide

/-- "This deals 1 damage to a color." -/
theorem badDamageToColor :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 1) (a (quality .color)) .any))
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-- "... loses 1 life for each attacking creature. You gain that much life." -/
theorem okThatMuchBound :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct (target .opponent) (loseLife
            (forEach 1 (.and [attacking, creature, .hasPossessor .controller .you]))),
          gainLife .thatMuch ]) = [] := by
  decide

/-- "This deals that much damage to any target." -/
theorem badThatMuchUnbound :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage .thatMuch (target anyTarget) .any))
      = [.quantOutcomeInScope 0] := by
  decide

theorem badThatMuchAmbig :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ (Instruction.instruct .this (.dealDamage (.lit 3) (target anyTarget) .any)),
          loseLife (.lit 2),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "This deals 3 damage to target creature. Destroy it." -/
theorem okDestroyDamagedCreature :
    Instruction.check (DRS.ofList [])
      (.sequentially [(Instruction.instruct .this (.dealDamage (.lit 3) (target creature) .any)), destroy it]) = [] := by
  decide

/-- "This deals 1 damage to target creature." -/
theorem okDamageOneToCreature :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 1) (target creature) .any)) = [] := by decide

/-- "creature you control or artifact you control" -/
theorem okStructuredDisjunction :
    Predicate.checkAt .object (DRS.ofList [])
      (.or [ .and [creature, .hasPossessor .controller .you],
             .and [artifact, .hasPossessor .controller .you] ]) = [] := by
  decide

/-- "other creature or land" -/
theorem badOtherInOr :
    Predicate.checkAt .object
      [({ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] } : Referent)]
      (.or [.and [creature, .other], land]) = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage to target creature or planeswalker." [CR#120.1] -/
theorem okDamageDisjunctHead :
    Instruction.check (DRS.ofList [])
      (Instruction.instruct .this (.dealDamage (.lit 2) (target (.or [creature, .hasType .planeswalker])) .any)) = [] := by
  decide

/-- "attacking or blocking creature on the battlefield" -/
theorem okAttackingOrBlockingOnBattlefield :
    Predicate.checkAt .object (DRS.ofList []) (.and [creature, .or [attacking, blocking], .inZone battlefield])
      = [] := by
  decide

/-- "This deals 2 damage to target creature. Tap it." -/
theorem okTapDamagedCreature :
    Instruction.check (DRS.ofList [])
      (.sequentially [(Instruction.instruct .this (.dealDamage (.lit 2) (target creature) .any)), .setStatus .tapped it])
      = [] := by
  decide

/-- "You gain 2 life if you control a creature. Tap it." The Idris pin refutes the anaphor. -/
theorem badConditionAntecedent :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .doOnlyIf (gainLife (.lit 2)) (exists_ creatureYouControl) none,
          .setStatus .tapped it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "You may sacrifice a creature. If you don't, exile it." -/
theorem badIfNotReadsMayBody :
    Instruction.check (DRS.ofList []) (Primitives.Instruction.offer (sacrifice (a creature)) none (some (exile it)))
      = [.anaphor .bare .one 0] := by
  decide

/-- "other than this creature" -/
theorem okComplementThisCreature :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan thisCreature) = [] := by decide

/-- "other than a creature" -/
theorem badComplementAnchorAnnounces :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan (a creature)) = [.complementAnchor] := by decide

/-- "other than up to two target creatures" -/
theorem badPluralComplementAnchor :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan (.described (.target (upTo 2)) creature))
      = [.complementAnchor] := by
  decide

/-- "each creature other than this creature" -/
theorem okComplementSameHead :
    Instruction.check (DRS.ofList [])
      (Instruction.instruct .this (.dealDamage (.lit 1) (each (.and [creature, .otherThan thisCreature])) .any)) = [] := by
  decide

/-- "each creature other than this land" -/
theorem badComplementCrossHead :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 1) (each (.and [creature, .otherThan thisLand])) .any))
      = [.otherAnchored] := by
  decide

/-- "each creature other than this creature other than this creature" -/
theorem badDoubleComplement :
    Instruction.check (DRS.ofList [])
      (Instruction.instruct .this (.dealDamage (.lit 1) (each (.and [creature, .otherThan thisCreature, .otherThan thisCreature])) .any))
      = [.otherAnchored] := by
  decide

/-- "each other creature other than this creature" -/
theorem badOtherAndComplement :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target creature),
          (Instruction.instruct .this (.dealDamage (.lit 1) (each (.and [creature, .other, .otherThan thisCreature])) .any)) ])
      = [.otherAnchored] := by
  decide

/-- "creature or land" -/
theorem okDisjunctWithoutAComplement :
    Predicate.checkAt .object (DRS.ofList []) (.or [creature, land]) = [] := by decide

/-- "creature other than this creature, or land" -/
theorem badComplementInOr :
    Predicate.checkAt .object (DRS.ofList []) (.or [.and [creature, .otherThan thisCreature], land])
      = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage divided as you choose among two target creatures." -/
theorem okDivideAmongTargets :
    Instruction.check (DRS.ofList [])
      (dealDivided .this (.lit 2) (.described (.target (oneThrough 2)) creature)) = [] := by
  decide

/-- "This deals 2 damage divided as you choose among each creature." -/
theorem badDivideAmongDescription :
    Instruction.check (DRS.ofList []) (dealDivided .this (.lit 2) (each creature)) = [.groupMention] := by
  decide

/-- "Put one of them into your hand and the rest into your graveyard." -/
theorem okRestAfterPart :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          move (someOf (exactly 1) them) hand,
          move (theRest .object) graveyard ]) = [] := by
  decide

/-- "Put the rest into your graveyard." -/
theorem badRestWithoutGroup :
    Instruction.check (DRS.ofList []) (move (theRest .object) graveyard) = [.theRestFits .object] := by decide

/-- "Look at the top four cards of your library. Put the rest on the bottom." -/
theorem badRestWithoutPart :
    Instruction.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice (.lit 4)), move (theRest .object) onBottom])
      = [.theRestFits .object] := by
  decide

theorem badRestDisposedTwice :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          move (someOf (exactly 1) them) hand,
          move (theRest .object) onBottom,
          move (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

theorem badRestOverTwoAnnouncements :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct (target creatureYouControl) (fight (target creatureYouDontControl)),
          move (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

/-- "This creature deals 2 damage to each opponent." -/
theorem okEachOpponentDamage :
    Instruction.check (DRS.ofList []) (Instruction.instruct thisCreature (.dealDamage (.lit 2) (each .opponent) .any)) = [] := by decide

/-- "This creature deals 2 damage to your opponents." -/
theorem badPluralPlayerDamageRecipient :
    Instruction.check (DRS.ofList []) (Instruction.instruct thisCreature (.dealDamage (.lit 2) (.playerGroup .yourOpponents) .any))
      = [.perMember] := by
  decide

/-- "Prevent all damage that would be dealt to any player or permanent." -/
theorem okPreventDealtToPermanent :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.prevent .all none) .repeatedly) = [] := by
  decide

/-- "… is dealt to target attacking creature instead." -/
theorem okRedirectToSingleCreature :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (target (.and [creature, attacking]))) .repeatedly) = [] := by
  decide

theorem badRedirectToPlural :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (allOf creatureYouControl)) .repeatedly) = [.singular] := by
  decide

/-- "… If damage from a red source is prevented this way, you gain 3 life." -/
def preventedFromSourceAnnounced : StaticSpec :=
  .damageRule .any .unattributed (.toRecipient .you)
    (.prevent .all
      (some (doIf (.preventedFromSource (.and [source, .colorIs .red])) (gainLife (.lit 3)))))
    .repeatedly

theorem okPreventedFromSourceAnnounced :
    StaticSpec.check (DRS.ofList []) preventedFromSourceAnnounced = [] := by decide

/-- "Prevent all damage that would be dealt to you this turn. If damage from a red source is
prevented this way, you gain 3 life." -/
theorem okPreventedFromSourceInAClause :
    Instruction.check (DRS.ofList []) (.establish preventedFromSourceAnnounced (some thisTurn)) = [] := by
  decide

/-- "If damage from a red source is prevented this way, you gain 3 life." -/
theorem badPreventedFromSourceUnannounced :
    Instruction.check (DRS.ofList [])
      (doIf (.preventedFromSource (.and [source, .colorIs .red])) (gainLife (.lit 3)))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… You gain life equal to the damage prevented this way." -/
def preventedThisWayAnnounced : StaticSpec :=
  .damageRule .any .unattributed (.toRecipient .you)
    (.prevent .all (some (gainLife preventedThisWay))) .repeatedly

theorem okPreventedThisWayAnnounced : StaticSpec.check (DRS.ofList []) preventedThisWayAnnounced = [] := by
  decide

/-- "Prevent all damage that would be dealt to you this turn. You gain life equal to the damage
prevented this way." -/
theorem okPreventedThisWayInAClause :
    Instruction.check (DRS.ofList []) (.establish preventedThisWayAnnounced (some thisTurn)) = [] := by
  decide

theorem badPreventedThisWayAfterDamage :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ (Instruction.instruct .this (.dealDamage (.lit 3) (target creature) .any)),
          .changeLife (.up preventedThisWay) ]) = [.outcomeInScope .damagePrevented
              0] := by
  decide

/-- "You gain life equal to the damage prevented this way." -/
theorem badPreventedThisWayUnannounced :
    Instruction.check (DRS.ofList []) (.changeLife (.up preventedThisWay))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… They gain 2 life for each card less than two they drew this way." -/
theorem okShortOfCeilingAnnounced :
    let drawing := instruct (each .anyPlayer)
      (.sequentially [offer (draw (.upTo (.lit 2))),
        instruct they (gainLife (times (.lit 2) shortOfCeiling))])
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) drawing).1
    Instruction.check (DRS.ofList []) drawing = [] ∧
      (after.visibleReferents.filter (fun ref => ref.payload == .outcome .ceilingShortfall)).map
        (·.id) = [2] := by decide

theorem singularTheyCannotReadThePublishedPlayerGroup :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (offer (instruct they (draw (.upTo (.lit 2))))),
          instruct they (gainLife (times (.lit 2) shortOfCeiling)) ]) =
      [.anaphor (.word .player) .one 0] := by decide

theorem optionalDrawSummaryDoesNotExportDrawnCards :
    let drawing := instruct .you (offer (draw (.upTo (.lit 2))))
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) drawing).1
    countOutcomes .ceilingShortfall after = 1 ∧
      NounPhrase.checkAt (some .object) (nextClause after) (those .card) =
        [.anaphor (.word .card) .many 0] := by decide

/-- "You gain 2 life for each card less than two you draw this way." -/
theorem badShortOfCeilingUnannounced :
    Instruction.check (DRS.ofList []) (gainLife (times (.lit 2) shortOfCeiling))
      = [.outcomeInScope .ceilingShortfall 0] := by
  decide

/-- "Prevent the next 3 damage that would be dealt to you this turn." -/
theorem okShieldRepeatedly :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you) (.prevent (.shield (.lit 3)) none)
        .repeatedly) = [] := by
  decide

theorem badShieldSizedByItsOwnPrevention :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you)
        (.prevent (.shield preventedThisWay) none) .repeatedly)
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "This creature deals 3 damage to target creature." -/
theorem okDamageToCreature :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 3) (target creature) .any)) = [] := by decide

/-- "…target nonartifact…" -/
theorem okNonartifact : Predicate.checkAt .object (DRS.ofList []) (.not artifact) = [] := by decide

/-- "…target nonsource…" -/
theorem badNonsource : Predicate.checkAt .object (DRS.ofList []) (.not source) = [.negatable] := by decide

/-- "If damage would be dealt to you, it's dealt to target creature you control instead." -/
theorem okRedirectToOne :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (target creatureYouControl)) .repeatedly) = [] := by
  decide

theorem badRedirectToGroup :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any .unattributed (.toRecipient .you)
        (.redirect .all (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
        .repeatedly) = [.singular] := by
  decide

/-- "Target opponent loses 1 life. You gain that much life." -/
theorem okThatMuchAfterLifeLoss :
    Instruction.check (DRS.ofList [])
      (.sequentially [instruct (target .opponent) (loseLife (.lit 1)), gainLife .thatMuch]) = [] := by
  decide

/-- "If a source would deal damage to a player or permanent, it deals double that damage
instead." -/
theorem okScaleDoubled :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any (.dealtBy (a source)) (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.scale (.multiplied .doubled)) .repeatedly) = [] := by
  decide

/-- "… it deals that much damage plus that much instead." -/
theorem badScaleShiftByThatMuch :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .any (.dealtBy (a source)) (.toRecipient (a (.or [permanent, .anyPlayer])))
        (.scale (.shifted .up .thatMuch)) .repeatedly) = [.quantOutcomeInScope 0] := by
  decide

/-- "Whenever this creature is dealt damage, it deals that much damage to any target." -/
theorem okThatMuchAfterDamageEvent :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount thisCreature .any))
        (Instruction.instruct thisCreature (.dealDamage .thatMuch (target anyTarget) .any))) = [] := by
  decide

theorem badPreventedThisWayAfterDamageEvent :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount thisCreature .any))
        (Instruction.instruct it (.dealDamage preventedThisWay (target anyTarget) .any)))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

theorem badThatMuchAfterDeath :
    Ability.check (DRS.ofList [])
      (whenever (Declared.dies (a creature))
        (Instruction.instruct thisCreature (.dealDamage .thatMuch (target anyTarget) .any))) = [.quantOutcomeInScope 0] := by
  decide

/-- "… That creature deals damage equal to its power to this creature." -/
theorem okThatCreatureAfterDamage :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ dealDamageOwnPower thisCreature (target creature),
          instruct (that (.type .creature)) (.dealDamage (powerOf it) thisCreature .any) ]) = [] := by
  decide

/-- "When this creature dies, it deals 1 damage to target creature. That creature's controller
loses 1 life." -/
theorem okThatCreatureAfterTargetedDamage :
    Ability.check (DRS.ofList [])
      (when (Declared.dies thisCreature)
        (.sequentially
          [ (Instruction.instruct thisCreature (.dealDamage (.lit 1) (target creature) .any)),
            instruct (controllerOf (that (.type .creature))) (loseLife (.lit 1)) ])) = [] := by
  decide

theorem damagedSelfIsReadableAsThatCreature :
    let event := GameEvent.does .any (.dealDamage .anyAmount thisCreature .any)
    let after := GameEvent.extend (DRS.ofList []) event
    Ability.check (DRS.ofList [])
      (whenever event
        (Instruction.instruct (that (.type .creature)) (.dealDamage .thatMuch (target anyTarget) .any))) = [] ∧
      (firstReach (.word (.type .creature)) .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

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
              [ .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .color) none .openly),
                .static preventLastChosenColor ] } }) = [] := by
  decide

theorem badLastChosenBeforeChooser :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static preventLastChosenColor,
                .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .color) none .openly) ] } })
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

theorem badLastChosenWrongSort :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static
                  (Primitives.StaticSpec.entryChoice thisEnchantment (.quality (.subtype .creature)) none .openly),
                .static preventLastChosenColor ] } })
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

/-- "sources of the last chosen color", a colour choice standing -/
theorem okLastChosenColorRead :
    Predicate.checkAt .object [({ det := .a, plur := .one, payload := .quality .color, idents := [] } : Referent)] (ofTheLastChosen .color)
      = [] := by
  decide

/-- "sources of the last chosen color" -/
theorem badLastChosenColorNoChooser :
    Predicate.checkAt .object (DRS.ofList []) (ofTheLastChosen .color)
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

/-- "the greatest life total among all players" -/
theorem okPlayerAggregate :
    Amount.check (DRS.ofList []) (.aggregate .max (.playerStat .lifeTotal) (allOf .anyPlayer)) = [] := by decide

/-- "the total power of target creature" -/
theorem badSingularAggregate :
    Amount.check (DRS.ofList []) (.aggregate .sum (.stat .power) (target creature)) = [.plural] := by decide

/-- "the greatest life total among all creatures" -/
theorem badAggregateWrongSort :
    Amount.check (DRS.ofList []) (.aggregate .max (.playerStat .lifeTotal) (allOf creature))
      = [.projScope .object] := by
  decide

/-- "1-20 | Draw a card." -/
theorem okLiteralRollRow :
    Instruction.check (DRS.ofList [])
      (.sequentially [rollDice 1 20, .applyResultsTable [⟨fromTo 1 20, draw (.lit 1)⟩]])
      = [] := by
  decide

/-- "up to X | Draw a card." -/
theorem badAmountRollRow :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [rollDice 1 20, .applyResultsTable [⟨.upToOf (.letter .x), draw (.lit 1)⟩]])
      = [.quantLiteral] := by
  decide

theorem badCreatureHalfRead :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ (Instruction.instruct .this (.dealDamage (.lit 3) (target (.or [.hasType .planeswalker, .anyPlayer])) .any)),
          discard (a (.and [.isCard, .inZone (handOf (Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one)
              (controllerOf (that (.type .creature)))))])) ]) = [.anaphor (.word (.type .creature))
                  .one 0] := by
  decide

/-- "This deals 3 damage to any target." -/
theorem okJoinDamageRecipient :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 3) (target anyTarget) .any)) = [] := by decide

/-- "Create a 1/1 white Soldier creature token." -/
theorem okSoldierToken :
    Instruction.check (DRS.ofList []) (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]))
      = [] := by
  decide

/-- "You gain life equal to your life total." -/
theorem okSingularLifeTotalRead : Amount.check (DRS.ofList []) (lifeTotalOf .you) = [] := by decide

/-- "You gain life equal to your opponents' life totals." -/
theorem badPluralLifeTotalRead :
    Amount.check (DRS.ofList []) (lifeTotalOf (.playerGroup .yourOpponents)) = [.singular] := by decide

/-- "You gain life equal to each player's life total." -/
theorem badDistributiveLifeTotalRead :
    Amount.check (DRS.ofList []) (lifeTotalOf (each .anyPlayer)) = [.singular] := by decide

/-- "Destroy target creature." -/
theorem okDestroyCreature : Instruction.check (DRS.ofList []) (destroy (target creature)) = [] := by decide

/-- "creature that was dealt combat damage by this creature this turn" [CR#120.1] -/
theorem okCombatDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.by thisCreature, .dealDamage .anyAmount (.asMarker .permanent .which) .combatOnly, .duration soFarThisTurn none⟩) = [] := by
  decide

/-- "creature that was dealt combat damage by a color this turn" -/
theorem badCombatDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.by (a (quality .color)), .dealDamage .anyAmount (.asMarker .permanent .which) .combatOnly, .duration soFarThisTurn none⟩)
      = [.kindMismatch .object (.quality .color)] := by
  decide

/-- "Target creature gets +1/+1 until end of turn." -/
theorem okGetsCreature :
    Instruction.check (DRS.ofList [])
      (get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) = [] := by
  decide

end Semantics.Proofs.Damage
