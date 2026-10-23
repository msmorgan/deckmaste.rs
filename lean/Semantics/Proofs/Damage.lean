import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Damage
import Semantics.Cards.Faces

/-!
# Semantics.Proofs.Damage

Port of `idris/src/Experimental/Proofs/Damage.idr`: the pins of the Damage family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Damage

/-- "Target creature fights target creature." -/
theorem okFightCreatures :
    Action.check (DRS.ofList []) (instruct (target isCreature) (fight (target isCreature))) = [] := by decide

private def fightParticipantPairs : Action → List (BindingId × BindingId)
  | .ownScope _ (.enact _ (.doIf _ (.simultaneously [
      .dealDamage (.statOf _ (.laterMention source _)) (.laterMention recipient (.inCaller _)) _,
      .instruct (.laterMention reverseSource _) (.dealDamage _ (.laterMention reverseRecipient _) _)]) none) _ _) =>
    [(source, recipient), (reverseSource, reverseRecipient)]
  | _ => []

theorem fightUsesTheTwoBoundParticipantsInOppositeDirections :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (instruct (.firstMention "fight.test.source" (target isCreature))
        (fight (.firstMention "fight.test.other" (target isCreature))))).1
    (identityBinding "fight.source" after).map (·.id) = some 0 ∧
      (identityBinding "fight.other" after).map (·.id) = some 2 ∧
      (identityBinding "fight.test.source" after).map (·.id) = some 0 ∧
      (identityBinding "fight.test.other" after).map (·.id) = some 2 ∧
      fightParticipantPairs (fight (target isCreature)) =
        [("fight.source", "fight.other"), ("fight.other", "fight.source")] := by decide

private def fightGuard : Action → Option Condition
  | .ownScope _ (.enact _ (.doIf guard _ none) _ _) => some guard
  | _ => none

/-- Fight's guard tests both participants as creatures, a present-tense guard that reads them as
they are when the fight happens: if either has left the battlefield or is no longer a creature,
neither fights or deals damage [CR#109.2,701.14b]. -/
theorem fightGuardTestsBothCreaturesNow :
    fightGuard (fight (target isCreature)) = some (.and [
      .matches (.firstMention "fight.other" (.inCaller (target isCreature))) isCreature,
      .matches (.firstMention "fight.source" .actor) isCreature]) := rfl

/-- Quenchable Fire's delayed trigger is unconditional: once it triggers, paying {U} no longer
stops it. Until it would trigger, "that player or that planeswalker's controller" may take a
special action, paying {U} to stop it from triggering [CR#116.2c]. -/
theorem quenchableFireStopsItsTriggerBySpecialAction :
    Cards.quenchableFire = .sequentially
      [ instruct this_ (dealDamage 3 Cards.targetPlayerOrPlaneswalker),
        .sequentially
          [ .delay (beginningOf .upkeep you nextTime) [] none
              (instruct this_ (dealDamage 3 thatJoin)),
            .establish
              (.specialAction (some splitOverPlaneswalker) none [pip .blue]
                (establish (cant (.actor (.laterMention "delay.trigger" thisAbility)) trigger) none))
              (some (untilEvent (beginningOf .upkeep you nextTime))) ] ] := rfl

/-- The special action stops the delayed trigger by establishing that it can't trigger, naming it
by its Binding Identity, the only reading of it: no text mentions the ability, so a word ("that
ability") finds nothing, and with no delayed trigger the identity is unbound [CR#116.2c].
Quenchable Fire checks with it. -/
theorem cantTriggerReadsTheDelayedTriggerByIdentity :
    Action.check (DRS.ofList [])
      (.sequentially [delay (beginningOfNext .endStep) (draw 1),
        establish (cant (.actor (.laterMention "delay.trigger" thisAbility)) trigger) none]) = [] ∧
    Action.check (DRS.ofList [])
      (.sequentially [delay (beginningOfNext .endStep) (draw 1),
        establish (cant (.actor (that .ability)) trigger) none])
      = [.anaphor (.word .ability) .one 0] ∧
    Action.check (DRS.ofList [])
      (establish (cant (.actor (.laterMention "delay.trigger" thisAbility)) trigger) none) =
      [.bindingIdentity "delay.trigger"] ∧
    Action.check (DRS.ofList []) Cards.quenchableFire = [] := by
  decide

/-- What "they" reads in the branch of Screaming Nemesis's triggered ability, as its referent's
kind and determiner: the trigger's damage dealt to "any other target", then the condition, then
"they" in the branch's clause. -/
private def screamingNemesisThey : Option (List (Kind × Determiner)) :=
  match (match Cards.screamingNemesis.card with
      | .singleFaced face => face.characteristics.text
      | _ => []) with
  | [_, .triggered _ _ _ _ _ _ _ (.sequentially [first, .doIf condition _ _])] =>
    let header := GameEvent.extend { DRS.ofList [] with sourceTypes := [.creature] }
      (.does .any (dealDamage (to := thisCreature)))
    let afterDamage := (stepAction 64 .postEvent .obj (nextClause header) first).1
    let afterCondition := (stepCondition 64 (nextClause afterDamage) condition).1
    let (read, ids, _) := stepNounPhrase 64 (nextClause afterCondition) .subj true they
    some (ids.filterMap fun id => (read.findById? id).map fun ref => (ref.kind, ref.det))
  | _ => none

/-- Screaming Nemesis: "If a player is dealt damage this way, they can't gain life for the rest of
the game." The condition reads back the damage dealt this way and publishes the player it was
dealt to, and "they" reads that player, not the target, which may be a creature or planeswalker
[CR#608.2c]. -/
theorem screamingNemesisTheyReadsTheDamagedPlayer :
    screamingNemesisThey = some [(.player, .a)] := by
  decide

private def objectAntecedents (a : Action) : List Bool :=
  (resolveCandidates .bare .one (nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) a).1)).map
    fun r => match r.payload with | .object .. => true | _ => false

/-- The Obzedat and Whip of Erebos shape, "… exile it at the beginning of the next end step. It
…": a delayed trigger is no antecedent, so a later "it" still reads the creature. -/
theorem pronounAfterDelayReadsTheCreature :
    objectAntecedents (.sequentially [tap (target isCreature),
      delay (beginningOfNext .endStep) (exile it)]) = [true] := by
  decide

/-- "At the beginning of the next end step, draw a card. Exile it.": nothing for "it" to read. -/
theorem badPronounAfterDelayedDraw :
    Action.check (DRS.ofList [])
      (.sequentially [delay (beginningOfNext .endStep) (draw 1), exile it]) =
      [.anaphor .bare .one 0] := by
  decide

/-- "Target creature fights two target creatures.": the group is the other creature, refused by
the singular power read and the per-member damage check. Handed the deed, a group performer
distributes it instead (`BindingIdentity.okFightHandedToEachOfAGroup`). -/
theorem badFightGroup :
    Action.check (DRS.ofList []) (instruct (target isCreature) (fight (.described (.target (exactly 2)) isCreature)))
      = [.perMember, .singular] := by
  decide

/-- "This deals 2 damage to any target and 1 damage to any other target." -/
theorem okAnyOtherTarget :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 2 (target isAnyTarget))),
          (Action.instruct .this (.dealDamage 1 (target isAnyOtherTarget))) ]) = [] := by
  decide

/-- "This deals 1 damage to any other target." With no earlier target, "other" contrasts with
the Source (`semantics-v3.md` §0.1, [CR#707.11]). -/
theorem okOtherAnchorsOnTheSource :
    let action := Action.instruct .this (.dealDamage 1 (target isAnyOtherTarget))
    Action.check (DRS.ofList []) action = [] ∧
      (stepAction 64 .postEvent .obj (DRS.ofList []) action).1.contrastAnchors = [.source] := by
  decide

/-- "Tap target creature. It gets −1/−1 until end of turn." (U+2212 minus signs: an ASCII
hyphen before the slash would open a comment.) -/
theorem okIt :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target isCreature),
          get it (.down 1) (.down 1) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "Target creature fights target creature. Tap it." -/
theorem fightSourceIsSelectedBySubjectPriority :
    let combat := instruct (target isCreature) (fight (target isCreature))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) combat).1
    Action.check (DRS.ofList []) (.sequentially [combat, .setStatus .tapped it]) = [] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Tap target creature." -/
theorem okTapBattlefield :
    Action.check (DRS.ofList []) (.setStatus .tapped (target isCreature)) = [] := by decide

theorem damageSourceRemainsTheSingularAntecedent :
    let damage := Action.instruct .this (.dealDamage 3 (each isCreature))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) damage).1
    Action.check (DRS.ofList []) (.sequentially [damage, .setStatus .tapped it]) = [] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.kind) = some .object := by decide

/-- "Choose two target creatures. Tap them." -/
theorem okThem :
    Action.check (DRS.ofList [])
      (.sequentially [choose (.described (.target (exactly 2)) isCreature), .setStatus .tapped them])
      = [] := by
  decide

/-- "Choose two target creatures. Choose two target creatures. Tap them." -/
theorem recentSecondChosenGroupIsSelected :
    let choices := .sequentially
      [choose (.described (.target (exactly 2)) isCreature),
        choose (.described (.target (exactly 2)) isCreature)]
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) choices).1
    Action.check (DRS.ofList []) (.sequentially
      [choose (.described (.target (exactly 2)) isCreature),
        choose (.described (.target (exactly 2)) isCreature), .setStatus .tapped them]) = [] ∧
      (firstReach .bare .many (nextClause after)).map (·.id) = some 1 ∧
      (after.findById? 1).map (·.ty) = some [.creature] := by decide

theorem recentSecondOpponentIsSelected :
    let combat := instruct (target (.and [isCreature, .hasPossessor .controller anOpponent]))
      (fight (target (.and [isCreature, .hasPossessor .controller anOpponent])))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) combat).1
    Action.check (DRS.ofList [])
      (.sequentially [combat, instruct (that .player) (loseLife 1)]) = [] ∧
      (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) = [3] ∧
      (after.findById? 3).map (·.kind) = some .player := by
  decide

/-- "A creature doesn't untap during your untap step." -/
theorem okUntapLockBattlefield :
    Ability.check (DRS.ofList []) (.static (doesntUntap (a isCreature) (some .you))) = [] := by decide

/-- "Target creature you control fights target creature you don't control." -/
theorem okFightControlledCreatures :
    Action.check (DRS.ofList []) (instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl]))))
      = [] := by
  decide

/-- "Target land fights target creature you don't control.": a land an effect has made a
creature fights, and one that is no longer a creature simply does not [CR#205.1b,701.14b]. -/
theorem okFightLand :
    Action.check (DRS.ofList []) (instruct (target isLand) (fight (target (and [isCreature, youDontControl])))) = [] := by
  decide

/-- "Target permanent fights target creature." [CR#701.14a] -/
theorem okFightPermanentUnderCreatureGuard :
    Action.check (DRS.ofList []) (instruct (target isPermanent) (fight (target isCreature)))
      = [] := by
  decide

/-- "This deals 3 damage to target creature." -/
theorem okDamageCreature :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 3 (target isCreature))) = [] := by decide

/-- "This deals 1 damage to a color." -/
theorem badDamageToColor :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 1 (a (quality .color))))
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-- "... loses 1 life for each attacking creature. You gain that much life." -/
theorem okThatMuchBound :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (target (.isOpponentOf .you)) (loseLife
            (forEach 1 (.and [isAttacking, isCreature, .hasPossessor .controller .you]))),
          gainLife .thatMuch ]) = [] := by
  decide

/-- "This deals that much damage to any target." -/
theorem badThatMuchUnbound :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage .thatMuch (target isAnyTarget)))
      = [.quantOutcomeInScope 0] := by
  decide

theorem badThatMuchAmbig :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 3 (target isAnyTarget))),
          loseLife 2,
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "This deals 3 damage to target creature. Destroy it." -/
theorem okDestroyDamagedCreature :
    Action.check (DRS.ofList [])
      (.sequentially [(Action.instruct .this (.dealDamage 3 (target isCreature))), destroy it]) = [] := by
  decide

/-- "This deals 1 damage to target creature." -/
theorem okDamageOneToCreature :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 1 (target isCreature))) = [] := by decide

/-- "creature you control or artifact you control" -/
theorem okStructuredDisjunction :
    Predicate.checkAt .object (DRS.ofList [])
      (.or [ .and [isCreature, .hasPossessor .controller .you],
             .and [isArtifact, .hasPossessor .controller .you] ]) = [] := by
  decide

/-- "other creature or land" -/
theorem badOtherInOr :
    Predicate.checkAt .object
      [({ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] } : Referent)]
      (.or [.and [isCreature, .other], isLand]) = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage to target creature or planeswalker." [CR#120.1] -/
theorem okDamageDisjunctHead :
    Action.check (DRS.ofList [])
      (Action.instruct .this (.dealDamage 2 (target (.or [isCreature, .hasType .planeswalker])))) = [] := by
  decide

/-- "attacking or blocking creature on the battlefield" -/
theorem okAttackingOrBlockingOnBattlefield :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .or [isAttacking, isBlocking], .in_ battlefield])
      = [] := by
  decide

/-- "This deals 2 damage to target creature. Tap it." -/
theorem okTapDamagedCreature :
    Action.check (DRS.ofList [])
      (.sequentially [(Action.instruct .this (.dealDamage 2 (target isCreature))), .setStatus .tapped it])
      = [] := by
  decide

/-- "You gain 2 life if you control a creature. Tap it." The Idris pin refutes the anaphor. -/
theorem badConditionAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ doOnlyIf (gainLife 2) (exists_ (and [isCreature, youControl])) none,
          .setStatus .tapped it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "You may sacrifice a creature. If you don't, exile it." -/
theorem badIfNotReadsMayBody :
    Action.check (DRS.ofList []) (may (sacrifice (a isCreature)) none (some (exile it)))
      = [.anaphor .bare .one 0] := by
  decide

/-- "other than this creature" -/
theorem okComplementThisCreature :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan thisCreature) = [] := by decide

/-- "other than a creature" -/
theorem badComplementAnchorAnnounces :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan (a isCreature)) = [.complementAnchor] := by decide

/-- "other than up to two target creatures" -/
theorem badPluralComplementAnchor :
    Predicate.checkAt .object (DRS.ofList []) (.otherThan (.described (.target (upTo 2)) isCreature))
      = [.complementAnchor] := by
  decide

/-- "each creature other than this creature" -/
theorem okComplementSameHead :
    Action.check (DRS.ofList [])
      (Action.instruct .this (.dealDamage 1 (each (.and [isCreature, .otherThan thisCreature])))) = [] := by
  decide

/-- "each creature other than this land" -/
theorem badComplementCrossHead :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 1 (each (.and [isCreature, .otherThan thisLand]))))
      = [.otherAnchored] := by
  decide

/-- "each creature other than this creature other than this creature" -/
theorem badDoubleComplement :
    Action.check (DRS.ofList [])
      (Action.instruct .this (.dealDamage 1 (each (.and [isCreature, .otherThan thisCreature, .otherThan thisCreature]))))
      = [.otherAnchored] := by
  decide

/-- "each other creature other than this creature" -/
theorem badOtherAndComplement :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target isCreature),
          (Action.instruct .this (.dealDamage 1 (each (.and [isCreature, .other, .otherThan thisCreature])))) ])
      = [.otherAnchored] := by
  decide

/-- "creature or land" -/
theorem okDisjunctWithoutAComplement :
    Predicate.checkAt .object (DRS.ofList []) (.or [isCreature, isLand]) = [] := by decide

/-- "creature other than this creature, or land" -/
theorem badComplementInOr :
    Predicate.checkAt .object (DRS.ofList []) (.or [.and [isCreature, .otherThan thisCreature], isLand])
      = [.coordinableDisjuncts] := by
  decide

/-- "This deals 2 damage divided as you choose among two target creatures." -/
theorem okDivideAmongTargets :
    Action.check (DRS.ofList [])
      (dealDivided .this 2 (.described (.target (oneThrough 2)) isCreature)) = [] := by
  decide

/-- "This deals 2 damage divided as you choose among each creature." -/
theorem badDivideAmongDescription :
    Action.check (DRS.ofList []) (dealDivided .this 2 (each isCreature)) = [.groupMention] := by
  decide

/-- "Put one of them into your hand and the rest into your graveyard." -/
theorem okRestAfterPart :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice 4),
          putInto (someOf (exactly 1) them) hand,
          putInto (theRest .object) graveyard ]) = [] := by
  decide

/-- "Put the rest into your graveyard." -/
theorem badRestWithoutGroup :
    Action.check (DRS.ofList []) (putInto (theRest .object) graveyard) = [.theRestFits .object] := by decide

/-- "Look at the top four cards of your library. Put the rest on the bottom." -/
theorem badRestWithoutPart :
    Action.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice 4), putInto (theRest .object) onBottom])
      = [.theRestFits .object] := by
  decide

theorem badRestDisposedTwice :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice 4),
          putInto (someOf (exactly 1) them) hand,
          putInto (theRest .object) onBottom,
          putInto (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

theorem badRestOverTwoAnnouncements :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl]))),
          putInto (theRest .object) graveyard ]) = [.theRestFits .object] := by
  decide

/-- "This creature deals 2 damage to each opponent." -/
theorem okEachOpponentDamage :
    Action.check (DRS.ofList []) (Action.instruct thisCreature (.dealDamage 2 (each (.isOpponentOf .you)))) = [] := by decide

/-- "This creature deals 2 damage to opponents.": the universal bare plural is "all opponents"
(`bare` expands to `.all`), each dealt the damage. -/
theorem okBarePluralPlayerDamageRecipient :
    Action.check (DRS.ofList []) (Action.instruct thisCreature (.dealDamage 2 (bare (.isOpponentOf .you))))
      = [] := by
  decide

/-- "Prevent all damage that would be dealt to any player or permanent." -/
theorem okPreventDealtToPermanent :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient (a (.or [isPermanent, .anyPlayer])))
        (.prevent .all none) .repeatedly) = [] := by
  decide

/-- "… is dealt to target attacking creature instead." -/
theorem okRedirectToSingleCreature :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you)
        (.redirect .all (target (.and [isCreature, isAttacking]))) .repeatedly) = [] := by
  decide

theorem badRedirectToPlural :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you)
        (.redirect .all (allOf (and [isCreature, youControl]))) .repeatedly) = [.singular] := by
  decide

/-- "… If damage from a red source is prevented this way, you gain 3 life." -/
def preventedFromSourceAnnounced : StaticSpec :=
  .damageRule .unattributed (.toRecipient .you)
    (.prevent .all
      (some (doIf (.preventedFromSource (.and [isSource, .colorIs .red])) (gainLife 3))))
    .repeatedly

theorem okPreventedFromSourceAnnounced :
    StaticSpec.check (DRS.ofList []) preventedFromSourceAnnounced = [] := by decide

/-- "Prevent all damage that would be dealt to you this turn. If damage from a red source is
prevented this way, you gain 3 life." -/
theorem okPreventedFromSourceInAClause :
    Action.check (DRS.ofList []) (.establish preventedFromSourceAnnounced (some thisTurn)) = [] := by
  decide

/-- "If damage from a red source is prevented this way, you gain 3 life." -/
theorem badPreventedFromSourceUnannounced :
    Action.check (DRS.ofList [])
      (doIf (.preventedFromSource (.and [isSource, .colorIs .red])) (gainLife 3))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… You gain life equal to the damage prevented this way." -/
def preventedThisWayAnnounced : StaticSpec :=
  .damageRule .unattributed (.toRecipient .you)
    (.prevent .all (some (gainLife preventedThisWay))) .repeatedly

theorem okPreventedThisWayAnnounced : StaticSpec.check (DRS.ofList []) preventedThisWayAnnounced = [] := by
  decide

/-- "Prevent all damage that would be dealt to you this turn. You gain life equal to the damage
prevented this way." -/
theorem okPreventedThisWayInAClause :
    Action.check (DRS.ofList []) (.establish preventedThisWayAnnounced (some thisTurn)) = [] := by
  decide

theorem badPreventedThisWayAfterDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 3 (target isCreature))),
          .changeLife (.up preventedThisWay) ]) = [.outcomeInScope .damagePrevented
              0] := by
  decide

/-- "You gain life equal to the damage prevented this way." -/
theorem badPreventedThisWayUnannounced :
    Action.check (DRS.ofList []) (.changeLife (.up preventedThisWay))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "… They gain 2 life for each card less than two they drew this way." -/
theorem okShortOfCeilingAnnounced :
    let drawing := instruct (each .anyPlayer)
      (.sequentially [may (drawCount (upToOf 2)),
        instruct they (gainLife (times 2 shortOfCeiling))])
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) drawing).1
    Action.check (DRS.ofList []) drawing = [] ∧
      (after.visibleReferents.filter (fun ref => ref.payload == .outcome .ceilingShortfall)).map
        (·.id) = [2] := by decide

theorem singularTheyCannotReadThePublishedPlayerGroup :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (may (instruct they (drawCount (upToOf 2)))),
          instruct they (gainLife (times 2 shortOfCeiling)) ]) =
      [.anaphor (.word .player) .one 0] := by decide

theorem optionalDrawSummaryDoesNotExportDrawnCards :
    let drawing := instruct .you (may (drawCount (upToOf 2)))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) drawing).1
    countOutcomes .ceilingShortfall after = 1 ∧
      NounPhrase.checkAt (some .object) (nextClause after) (those .card) =
        [.anaphor (.word .card) .many 0] := by decide

/-- "You gain 2 life for each card less than two you draw this way." -/
theorem badShortOfCeilingUnannounced :
    Action.check (DRS.ofList []) (gainLife (times 2 shortOfCeiling))
      = [.outcomeInScope .ceilingShortfall 0] := by
  decide

/-- "Prevent the next 3 damage that would be dealt to you this turn." -/
theorem okShieldRepeatedly :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you) (.prevent (.shield 3) none)
        .repeatedly) = [] := by
  decide

theorem badShieldSizedByItsOwnPrevention :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you)
        (.prevent (.shield preventedThisWay) none) .repeatedly)
      = [.outcomeInScope .damagePrevented 0] := by
  decide

/-- "This creature deals 3 damage to target creature." -/
theorem okDamageToCreature :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 3 (target isCreature))) = [] := by decide

/-- "…target nonartifact…" -/
theorem okNonartifact : Predicate.checkAt .object (DRS.ofList []) (.not isArtifact) = [] := by decide

/-- "…target nonsource…" -/
theorem badNonsource : Predicate.checkAt .object (DRS.ofList []) (.not isSource) = [.negatable] := by decide

/-- "If damage would be dealt to you, it's dealt to target creature you control instead." -/
theorem okRedirectToOne :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you)
        (.redirect .all (target (and [isCreature, youControl]))) .repeatedly) = [] := by
  decide

theorem badRedirectToGroup :
    StaticSpec.check (DRS.ofList [])
      (.damageRule .unattributed (.toRecipient .you)
        (.redirect .all (youAnd (allOf (.and [isPermanent, .hasPossessor .controller .you]))))
        .repeatedly) = [.singular] := by
  decide

/-- "Target opponent loses 1 life. You gain that much life." -/
theorem okThatMuchAfterLifeLoss :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (target (.isOpponentOf .you)) (loseLife 1), gainLife .thatMuch]) = [] := by
  decide

/-- "If a source would deal damage to a player or permanent, it deals double that damage
instead." -/
theorem okScaleDoubled :
    StaticSpec.check (DRS.ofList [])
      (.damageRule (.dealtBy (a isSource)) (.toRecipient (a (.or [isPermanent, .anyPlayer])))
        (.scale (.multiplied .doubled)) .repeatedly) = [] := by
  decide

/-- "… it deals that much damage plus that much instead." -/
theorem badScaleShiftByThatMuch :
    StaticSpec.check (DRS.ofList [])
      (.damageRule (.dealtBy (a isSource)) (.toRecipient (a (.or [isPermanent, .anyPlayer])))
        (.scale (.shifted (.up .thatMuch))) .repeatedly) = [.quantOutcomeInScope 0] := by
  decide

/-- "Whenever this creature is dealt damage, it deals that much damage to any target." -/
theorem okThatMuchAfterDamageEvent :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount thisCreature))
        (Action.instruct thisCreature (.dealDamage .thatMuch (target isAnyTarget)))) = [] := by
  decide

theorem badPreventedThisWayAfterDamageEvent :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.dealDamage .anyAmount thisCreature))
        (Action.instruct it (.dealDamage preventedThisWay (target isAnyTarget))))
      = [.outcomeInScope .damagePrevented 0] := by
  decide

theorem badThatMuchAfterDeath :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) die)
        (Action.instruct thisCreature (.dealDamage .thatMuch (target isAnyTarget)))) = [.quantOutcomeInScope 0] := by
  decide

/-- The context in which a handoff's body reads "its", and what the handoff's subject resolved
to: `subject` stepped as the subject of the clause after `before`, under its performer frame,
while a characteristic is read (the steps `Action.instruct` and `Amount.statOf` take). -/
private def possessiveContext (before : DRS) (subject : NounPhrase) : DRS × List ReferentId :=
  let (after, ids, _) := stepNounPhrase 64 (nextClause before) .subj true subject
  let shape : ActorShape := { kind := .object, isYou := false, ascribable := false, refIds := ids }
  ({ after.push { det := .the, plur := .one, payload := .actor shape } .other false with
      characteristicRead := true }, ids)

/-- After "This creature deals damage equal to its power to target creature.": this creature is 0,
the target creature 2. -/
private def yetiFirst : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList [])
    (dealDamageOwnPower thisCreature (target isCreature))).1

/-- "… That creature deals damage equal to its power to this creature.": "that creature" reads the
creature just dealt damage (2), not this creature (0). -/
theorem okThatCreatureAfterDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ dealDamageOwnPower thisCreature (target isCreature),
          dealDamageOwnPower (that (.type .creature)) thisCreature ]) = [] ∧
      (possessiveContext yetiFirst (that (.type .creature))).2 = [2] ∧
      (yetiFirst.findById? 2).map (·.det) = some .target := by
  decide

/-- Karplusan Yeti: "This creature deals damage equal to its power to target creature. That
creature deals damage equal to its power to this creature." In the second sentence "its" reads
the re-mentioned subject, that creature (2), not the first sentence's subject, this creature (0),
though this creature is still visible: `D′` ranks a referent re-mentioned in a direct argument
slot of the current clause there. -/
theorem karplusanYetiItsReadsThatCreature :
    Cards.karplusanYeti =
        sequentially
          [ dealDamageOwnPower thisCreature (target isCreature),
            dealDamageOwnPower (that creature) thisCreature ] ∧
      (possessiveContext yetiFirst (that creature)).2 = [2] ∧
      (firstReach .bare .one (possessiveContext yetiFirst (that creature)).1).map (·.id) =
        some 2 ∧
      (possessiveContext yetiFirst (that creature)).1.visibleReferents.any (·.id == 0) = true ∧
      (yetiFirst.findById? 0).map (·.det) = some .the :=
  ⟨rfl, by decide⟩

/-- After "Garruk Relentless deals 3 damage to target creature.": Garruk is 0, the creature 2. -/
private def garrukFirst : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList [])
    (instruct thisPlaneswalker (dealDamage 3 (target isCreature)))).1

/-- Garruk Relentless: "Garruk Relentless deals 3 damage to target creature. That creature deals
damage equal to its power to him." "Its" reads that creature (2), not Garruk (0). -/
theorem garrukRelentlessItsReadsThatCreature :
    Cards.garrukRelentlessFight =
        sequentially
          [ instruct thisPlaneswalker (dealDamage 3 (target isCreature)),
            dealDamageOwnPower (that creature) thisPlaneswalker ] ∧
      (possessiveContext garrukFirst (that creature)).2 = [2] ∧
      (firstReach .bare .one (possessiveContext garrukFirst (that creature)).1).map (·.id) =
        some 2 ∧
      (possessiveContext garrukFirst (that creature)).1.visibleReferents.any (·.id == 0) = true :=
  ⟨rfl, by decide⟩

/-- Lightning Bolt: "Lightning Bolt deals 3 damage to any target." The spell is the Source of the
damage [CR#120.1], so the damage is handed to it, and inside that Handoff the Actor of
`dealDamage` is the spell: an Object, with a controller. A bare `dealDamage`'s Actor is the
controller, a Player, which has none. -/
theorem boltSourceIsTheActor :
    Cards.bolt = instruct this_ (dealDamage 3 (target isAnyTarget)) ∧
      Action.check (DRS.ofList [])
          (instruct this_ (dealDamage 3 (controllerOf theActor))) = [] ∧
      Action.check (DRS.ofList []) (dealDamage 3 (controllerOf theActor)) =
        [.possessorKind .controller .player] :=
  ⟨rfl, by decide⟩

/-- "When this creature dies, it deals 1 damage to target creature. That creature's controller
loses 1 life." -/
theorem okThatCreatureAfterTargetedDamage :
    Ability.check (DRS.ofList [])
      (when (does thisCreature die)
        (.sequentially
          [ (Action.instruct thisCreature (.dealDamage 1 (target isCreature))),
            instruct (controllerOf (that (.type .creature))) (loseLife 1) ])) = [] := by
  decide

theorem damagedSelfIsReadableAsThatCreature :
    let event := GameEvent.does .any (.dealDamage .anyAmount thisCreature)
    let after := GameEvent.extend (DRS.ofList []) event
    Ability.check (DRS.ofList [])
      (whenever event
        (Action.instruct (that (.type .creature)) (.dealDamage .thatMuch (target isAnyTarget)))) = [] ∧
      (firstReach (.word (.type .creature)) .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Prevent all damage sources of the last chosen color would deal to you." -/
def preventLastChosenColor : StaticSpec :=
  .damageRule (.dealtBy (allOf (.and [isSource, ofTheLastChosen .color]))) (.toRecipient .you)
    (.prevent .all none) .repeatedly

theorem okLastChosenAfterChooser :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static (entersChoosing thisEnchantment .color),
                .static preventLastChosenColor ] } }) = [] := by
  decide

theorem badLastChosenBeforeChooser :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static preventLastChosenColor,
                .static (entersChoosing thisEnchantment .color) ] } })
      = [.choiceRef .theLatestChoice (.quality .color) 0] := by
  decide

theorem badLastChosenWrongSort :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.enchantment],
            text :=
              [ .static
                  (entersChoosing thisEnchantment (.subtype .creature)),
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
    Amount.check (DRS.ofList []) (.aggregate .sum (.stat .power) (target isCreature)) = [.plural] := by decide

/-- "the greatest life total among all creatures" -/
theorem badAggregateWrongSort :
    Amount.check (DRS.ofList []) (.aggregate .max (.playerStat .lifeTotal) (allOf isCreature))
      = [.projScope .object] := by
  decide

/-- "1-20 | Draw a card." -/
theorem okLiteralRollRow :
    Action.check (DRS.ofList [])
      (.sequentially [rollDice 1 (.sides 20), .applyResultsTable [⟨fromTo 1 20, draw 1⟩]])
      = [] := by
  decide

/-- "up to X | Draw a card." -/
theorem badAmountRollRow :
    Action.check (DRS.ofList [])
      (.sequentially
        [rollDice 1 (.sides 20), .applyResultsTable [⟨.upToOf (.letter .x), draw 1⟩]])
      = [.quantLiteral] := by
  decide

theorem badCreatureHalfRead :
    Action.check (DRS.ofList [])
      (.sequentially
        [ (Action.instruct .this (.dealDamage 3 (target (.or [.hasType .planeswalker, .anyPlayer])))),
          discard (a (.and [.isCard, .in_ (handOf (eitherOf (.pro (.unionHalf .player) .one)
              (controllerOf (that (.type .creature)))))])) ]) = [.anaphor (.word (.type .creature))
                  .one 0] := by
  decide

/-- "This deals 3 damage to any target." -/
theorem okJoinDamageRecipient :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 3 (target isAnyTarget))) = [] := by decide

/-- "Create a 1/1 white Soldier creature token." -/
theorem okSoldierToken :
    Action.check (DRS.ofList []) (create 1 (.written (creatureToken 1 1 [.white] [soldier])))
      = [] := by
  decide

/-- "You gain life equal to your life total." -/
theorem okSingularLifeTotalRead : Amount.check (DRS.ofList []) (lifeTotalOf .you) = [] := by decide

/-- "You gain life equal to your opponents' life totals." -/
theorem badPluralLifeTotalRead :
    Amount.check (DRS.ofList []) (lifeTotalOf (.described .all (.isOpponentOf .you))) = [.singular] := by decide

/-- "You gain life equal to each player's life total." -/
theorem badDistributiveLifeTotalRead :
    Amount.check (DRS.ofList []) (lifeTotalOf (each .anyPlayer)) = [.singular] := by decide

/-- "Destroy target creature." -/
theorem okDestroyCreature : Action.check (DRS.ofList []) (destroy (target isCreature)) = [] := by decide

/-- "creature that was dealt combat damage by this creature this turn" [CR#120.1] -/
theorem okCombatDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .by thisCreature
              action := .dealDamage .anyAmount (.headed .permanent .which) (some .combat)
              within := some (.duration soFarThisTurn none) }) = [] := by
  decide

/-- "creature that was dealt combat damage by a color this turn" -/
theorem badCombatDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .by (a (quality .color))
              action := .dealDamage .anyAmount (.headed .permanent .which) (some .combat)
              within := some (.duration soFarThisTurn none) })
      = [.kindMismatch .object (.quality .color)] := by
  decide

/-- "creature that was dealt damage this turn": a relative clause conjoined with its head's
other conjuncts reads `which` without a marker wrapper. -/
theorem okConjunctRelativeDamaged :
    Predicate.checkAt .object (DRS.ofList [])
      (and [isCreature, was (dealDamage anyAmount which) soFarThisTurn]) = [] := by
  decide

/-- "Destroy target creature that blocked this turn." -/
theorem okConjunctRelativeBlocked :
    Action.check (DRS.ofList [])
      (destroy (target (and [isCreature, did which block soFarThisTurn]))) = [] := by
  decide

/-- A conjunct relative clause's `which` is seeded from its enclosing predicate: "creature that
…" binds a creature on the battlefield, not a bare object. -/
theorem conjunctRelativeWhichReadsItsSiblingConjuncts :
    (relativeSeed .object (and [isCreature, did which block soFarThisTurn])).payload =
      .object [.creature] (some .battlefield) none none none := by
  decide

/-- "Target creature gets +1/+1 until end of turn." -/
theorem okGetsCreature :
    Action.check (DRS.ofList [])
      (get (target isCreature) (.up 1) (.up 1) (some untilEndOfTurn)) = [] := by
  decide

end Semantics.Proofs.Damage
