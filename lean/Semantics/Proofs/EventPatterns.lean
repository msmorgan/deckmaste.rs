import Semantics.Macros.Declared
import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.EventPatterns

private def nounZone (drs : DRS) (noun : NounPhrase) : Option Zone :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.head? >>= fun id => (result.1.findById? id).bind (·.zone)


private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

theorem unrestrictedDamageEventIsWellFormed :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount .any .any)) = [] := by decide

theorem damageSourceIsObject :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.dealDamage .anyAmount .any .any)) =
      [.kindMismatch .object .player] := by decide

theorem recipientMayBePlayer :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount .you .combatOnly)) = [] := by decide

theorem sourcePrecedesReflexiveRecipientReference :
    GameEvent.check (DRS.ofList [])
      (.does (.actor (a creature)) (.dealDamage .anyAmount (.refl .one) .any)) = [] := by decide

theorem bareRecipientCannotReferToCoargumentSource :
    GameEvent.check (DRS.ofList [])
      (.does (.actor (a creature)) (.dealDamage .anyAmount it .any)) =
      [.anaphor .bare .one 0] := by decide

theorem absentSourceCannotSupplyReference :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount it .any)) =
      [.anaphor .bare .one 0] := by decide

theorem damagePublishesParticipantsAndOutcome :
    let after := GameEvent.extend (DRS.ofList [])
      (.does (.actor (a creature)) (.dealDamage .anyAmount (a .anyPlayer) .any))
    countOutcomes .damageDealt after = 1 ∧
      (after.findById? 0).map (·.role) = some .subj ∧
      (after.findById? 2).map (·.role) = some .obj := by decide

theorem observedDamageDoesNotExportOrRefineReferents (kind : DamageKind) :
    let before := nounResult (DRS.ofList []) (target artifact)
    let after := (stepPredicate 64 before (.did ⟨.actor .which,
      .dealDamage .anyAmount .you kind, .thisWay⟩)).1
    after.referents = before.referents ∧ countOutcomes .damageDealt after = 0 := by
  cases kind <;> decide

theorem patientOnlyDoesNotManufactureSource :
    (GameEvent.extend (DRS.ofList [])
      (.does .any (.dealDamage .anyAmount thisCreature .any))).performer = none := by decide

theorem choiceOccasionsRemainDistinct :
    StaticSpec.choice .entry thisCreature (.quality .color) none .openly ≠
      StaticSpec.choice .attachment thisCreature (.quality .color) none .openly := by
  intro equal
  cases equal

theorem choiceDisclosureRemainsDistinct :
    StaticSpec.choice .entry thisCreature .player none .openly ≠
      StaticSpec.choice .entry thisCreature .player none .secretly := by
  intro equal
  cases equal

theorem everyChoiceOccasionPublishesItsSort (occasion : ChoiceOccasion) (drs : DRS)
    (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    (disclosure : Disclosure) :
    ((stepStaticSpec 64 drs (.choice occasion subject sort domain disclosure)).1.referents.head?).map
      (·.payload) = some sort.binding.payload := by
  cases sort <;> rfl

theorem everyChoiceOccasionChecksDomain (occasion : ChoiceOccasion) :
    StaticSpec.check (DRS.ofList [])
      (.choice occasion thisCreature (.quality .color) (some (.players .opponent)) .openly) =
      [.kindAxisSort] := by
  cases occasion <;> decide

theorem deathPublishesArrival :
    nounZone (GameEvent.extend (DRS.ofList []) (Declared.dies (a creature))) it =
      some .graveyard := by decide

theorem deathInterceptionKeepsDeparture :
    nounZone (WouldClause.extend (DRS.ofList [])
      (.would .any (.move (a creature) (.zones [battlefield]) (some graveyard) .departure none []))) it =
      some .battlefield := by decide

theorem unknownDepartureCanBeConstrained :
    GameEvent.check (DRS.ofList []) (Declared.dies (a .isCard)) = [] := by decide

theorem excludingOnePlayersZoneDoesNotExcludeEveryPlayersZone :
    GameEvent.check (DRS.ofList [])
      (.does .any (.move (a (.and [.isCard, .inZone (graveyardOf (a .opponent))])) (.anywhereBut [graveyardOf .you]) (some exileZone) .departure none [])) = [] := by decide

theorem arrivalCanBeDescribedAsCard :
    GameEvent.check (DRS.ofList [])
      (.does .any (.move (a (.and [.isCard, creature])) (.zones [.zone .library .bare]) (some graveyard) .arrival none [])) = [] := by decide

theorem anywhereAndBattlefieldHaveDifferentObservation :
    (match Primitives.GameEvent.putInto .this graveyard (some .anywhere) with
      | .does _ (.move _ _ _ point _ _) => point | _ => .departure) = .arrival ∧
    (match Primitives.GameEvent.putInto .this graveyard (some (.zones [battlefield])) with
      | .does _ (.move _ _ _ point _ _) => point | _ => .arrival) = .departure := by decide

theorem historyDoesNotMoveTheCurrentReference :
    let before := nounResult (DRS.ofList []) (a creature)
    let after := (stepPredicate 64 before (.did ⟨.any,
      .move .which (.zones [battlefield]) (some graveyard) .departure none [], .thisWay⟩)).1
    nounZone after it = some .battlefield := by decide

theorem anyZoneChangeIsMeaningful :
    GameEvent.check (DRS.ofList []) (.does .any (.move (a .isCard) .anywhere none .arrival none [])) = [] := by decide

theorem playerIsNotAChangingObject :
    GameEvent.check (DRS.ofList []) (.does .any (.move .you .anywhere none .arrival none [])) =
      [.kindLte .player (.join .object .pile)] := by decide

theorem explicitCardLocationIsRetained :
    nounZone (DRS.ofList []) (a (.and [.isCard, creature, .inZone graveyard])) =
      some .graveyard := by decide

theorem wholeTurnConditionNeedsNoPossessor :
    Condition.check (DRS.ofList []) (.duringPart .turn none) = [] := by decide

theorem partConditionChecksPlayer :
    Condition.check (DRS.ofList []) (.duringPart .combat (some thisCreature)) =
      [.kindMismatch .player .object] := by decide

theorem applicabilityReadsTheSpecSubject :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.partScope .turn (some (that .player))
        (.manaRetention (a .anyPlayer) (.unspent none))) = [] := by decide

theorem applicabilityPreservesNumericDefinition :
    Amount.check (stepStaticSpec 64 (DRS.ofList [])
      (Primitives.StaticSpec.partScope .turn none (.letterDefinition .x (.lit 2)))).1
      (.letter .x) = [] := by decide

theorem ownedOutsideCardsAreAdmitted :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (allOf (.and [.isCard, creature, .hasPossessor .owner .you, .not (.inZone battlefield)])) =
      [] := by decide

theorem controlledCreatureSpellsRemainIncluded :
    StaticSpec.check (DRS.ofList []) (.characteristicChange
      (allOf (.and [creature, spell, .hasPossessor .controller .you]))
      [.everyTypeOf .adds .creature]) = [] := by decide

theorem beforeObservationRetainsKnownUnspecifiedOrigin :
    nounZone (WouldClause.extend (DRS.ofList [])
      (.would .any (.move (a creature) .anywhere none .departure none []))) it =
      some .battlefield := by decide

theorem placelessCardsDoNotAcquireBattlefieldEvidence :
    nounZone (nounResult (DRS.ofList [])
      (a (.and [.isCard, creature, .not (.inZone battlefield)]))) it = none := by decide

theorem destinationReadsTheSourcePossessor :
    GameEvent.check (DRS.ofList []) (.does .any (.move (a .isCard) (.zones [graveyardOf (a .opponent)]) (some (handOf (that .player))) .departure none [])) =
      [] := by decide

theorem bodyReadsTheSourcePossessor :
    NounPhrase.checkAt (some .player)
      (GameEvent.extend (DRS.ofList []) (.does .any (.move (a .isCard) (.zones [graveyardOf (a .opponent)]) (some (handOf (that .player))) .departure none [])))
      (that .player) = [] := by decide

theorem excludingYourGraveyardAllowsAnOpponentsGraveyard :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (a (.and [.isCard, .inZone (graveyardOf (a .opponent)),
        .not (.inZone (graveyardOf .you))])) = [] := by decide

private def headerCreature : Referent :=
  { det := .the, plur := .one, role := .subj,
    payload := .object [.creature] (some .battlefield) none none none none }

private def headerArtifact : Referent :=
  { det := .a, plur := .one, role := .other,
    payload := .object [.artifact] (some .battlefield) none none none none }

theorem alternativeHeaderKeepsSourceIdentityAndClosesExtra :
    let primary := DRS.ofList [headerCreature]
    let alternative := DRS.ofList [headerCreature, headerArtifact]
    let common := commonHeaderContext (DRS.ofList []) [primary, alternative]
    common.visibleReferents.map (fun r => (r.id, r.ty)) = [(0, [.creature])] ∧
      common.nextId = 2 := by decide

theorem forkedIdsDoNotIdentifyDifferentHeaderParticipants :
    let creatureBranch := DRS.ofList [headerCreature]
    let artifactBranch := DRS.ofList [{ headerArtifact with role := .subj }]
    (commonHeaderContext (DRS.ofList []) [creatureBranch, artifactBranch]).referents = [] := by decide

end Semantics.Proofs.EventPatterns
