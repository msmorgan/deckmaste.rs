import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

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
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount (.refl .one) .any)) = [] := by decide

theorem bareRecipientCannotReferToCoargumentSource :
    GameEvent.check (DRS.ofList [])
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount it .any)) =
      [.anaphor .bare .one 0] := by decide

theorem absentSourceCannotSupplyReference :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount it .any)) =
      [.anaphor .bare .one 0] := by decide

theorem occurredDamageComposesSourceThenBodyForEveryContext
    (fuel : Nat) (drs : DRS) (source patient : NounPhrase) (kind : DamageKind) :
    let subjectContext := (stepSubjectBeforeWith (stepOps fuel) drs (.actor source)).1
    let bodyContext := (stepAction fuel .postEvent .obj
      { subjectContext with actionPattern := true } (.dealDamage .anyAmount patient kind)).1
    (stepGameEvent (fuel + 1) drs
      (.does (.actor source) (.dealDamage .anyAmount patient kind))).1 =
      { (dropActorFrame bodyContext) with actionPattern := drs.actionPattern } := by rfl

/-- Every occurred damage event publishes its outcome ahead of participant referents. -/
theorem damageOutcomePrecedesParticipantsForEveryContext
    (drs : DRS) (kind : DamageKind) (source patient : NounPhrase) :
    ((GameEvent.extend drs
      (.does (.actor source) (.dealDamage .anyAmount patient kind))).referents.head?).map
        (·.payload) = some (.outcome .damageDealt) := by rfl

theorem damagePublishesParticipantsAndOutcome :
    let after := GameEvent.extend (DRS.ofList [])
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount (a .anyPlayer) .any))
    countOutcomes .damageDealt after = 1 ∧
      (after.findById? 0).map (·.role) = some .subj ∧
      (after.findById? 2).map (·.role) = some .obj := by decide

theorem historicalDamagePublishesItsOutcomeLocallyForEveryContext
    (drs : DRS) (kind : DamageKind) (source patient : NounPhrase) :
    ((stepDidClause 64 drs
      ⟨.actor source, .dealDamage .anyAmount patient kind, .thisWay⟩).1.referents.head?).map
        (·.payload) = some (.outcome .damageDealt) := by rfl

theorem observedDamageDoesNotExportOrRefineReferents (kind : DamageKind) :
    let before := nounResult (DRS.ofList []) (target isArtifact)
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
      (.choice occasion thisCreature (.quality .color) (some (.players (.isOpponentOf .you))) .openly) =
      [.kindAxisSort] := by
  cases occasion <;> decide

theorem deathPublishesArrival :
    nounZone (GameEvent.extend (DRS.ofList []) (does (a isCreature) die)) it =
      some .graveyard := by decide

theorem deathInterceptionKeepsDeparture :
    nounZone (WouldClause.extend (DRS.ofList [])
      (.would .any (.move (a isCreature) (battlefield) (some graveyard) .departure none []))) it =
      some .battlefield := by decide

theorem unknownDepartureCanBeConstrained :
    GameEvent.check (DRS.ofList []) (does (a .isCard) die) = [] := by decide

theorem excludingOnePlayersZoneDoesNotExcludeEveryPlayersZone :
    GameEvent.check (DRS.ofList [])
      (.does .any (.move (a (.and [.isCard, .in_ (graveyardOf (a (.isOpponentOf .you)))])) (.not (graveyardOf .you)) (some exileZone) .departure none [])) = [] := by decide

theorem arrivalCanBeDescribedAsCard :
    GameEvent.check (DRS.ofList [])
      (.does .any (.move (a (.and [.isCard, isCreature])) (.zone .library none) (some graveyard) .arrival none [])) = [] := by decide

theorem anywhereAndBattlefieldHaveDifferentObservation :
    (match does any (putInto .this graveyard (from_ := some .anyLocation)) with
      | .does _ (.move _ _ _ point _ _) => point | _ => .departure) = .arrival ∧
    (match does any (putInto .this graveyard (from_ := some (battlefield))) with
      | .does _ (.move _ _ _ point _ _) => point | _ => .arrival) = .departure := by decide

theorem historyDoesNotMoveTheCurrentReference :
    let before := nounResult (DRS.ofList []) (a isCreature)
    let after := (stepPredicate 64 before (.did ⟨.any,
      .move .which (battlefield) (some graveyard) .departure none [], .thisWay⟩)).1
    nounZone after it = some .battlefield := by decide

theorem anyZoneChangeIsMeaningful :
    GameEvent.check (DRS.ofList []) (.does .any (.move (a .isCard) .anyLocation none .arrival none [])) = [] := by decide

theorem playerIsNotAChangingObject :
    GameEvent.check (DRS.ofList []) (.does .any (.move .you .anyLocation none .arrival none [])) =
      [.kindLte .player (.join .object .pile)] := by decide

theorem explicitCardLocationIsRetained :
    nounZone (DRS.ofList []) (a (.and [.isCard, isCreature, .in_ graveyard])) =
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
      (allOf (.and [.isCard, isCreature, .hasPossessor .owner .you, .not (.in_ battlefield)])) =
      [] := by decide

theorem controlledCreatureSpellsRemainIncluded :
    StaticSpec.check (DRS.ofList []) (.characteristicChange
      (allOf (.and [isCreature, isSpell, .hasPossessor .controller .you]))
      [.everyTypeOf .adds .creature]) = [] := by decide

theorem beforeObservationRetainsKnownUnspecifiedOrigin :
    nounZone (WouldClause.extend (DRS.ofList [])
      (.would .any (.move (a isCreature) .anyLocation none .departure none []))) it =
      some .battlefield := by decide

theorem placelessCardsDoNotAcquireBattlefieldEvidence :
    nounZone (nounResult (DRS.ofList [])
      (a (.and [.isCard, isCreature, .not (.in_ battlefield)]))) it = none := by decide

theorem destinationReadsTheSourcePossessor :
    GameEvent.check (DRS.ofList []) (.does .any (.move (a .isCard) (graveyardOf (a (.isOpponentOf .you))) (some (handOf (that .player))) .departure none [])) =
      [] := by decide

theorem bodyReadsTheSourcePossessor :
    NounPhrase.checkAt (some .player)
      (GameEvent.extend (DRS.ofList []) (.does .any (.move (a .isCard) (graveyardOf (a (.isOpponentOf .you))) (some (handOf (that .player))) .departure none [])))
      (that .player) = [] := by decide

theorem excludingYourGraveyardAllowsAnOpponentsGraveyard :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (a (.and [.isCard, .in_ (graveyardOf (a (.isOpponentOf .you))),
        .not (.in_ (graveyardOf .you))])) = [] := by decide

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

theorem uniqueCounterpartSurvivesVoiceRoleChange :
    let source := headerCreature
    let counterpart := { headerCreature with det := .a, role := .obj, id := 2 }
    let primary := DRS.ofList [counterpart, source]
    let alternative := DRS.ofList [{ counterpart with role := .other, id := 1 }, source]
    let common := commonHeaderContext (DRS.ofList []) [primary, alternative]
    common.visibleReferents.map (·.id) = [2, 0] ∧
      (firstReach (.word (.type .creature)) .one common).map (·.id) = some 2 := by decide

theorem indistinguishableCounterpartsCannotShareOneAlternativeId :
    let first := { headerCreature with det := .a, role := .obj, id := 1 }
    let second := { headerCreature with det := .a, role := .subj }
    let primary := DRS.ofList [first, second]
    let alternative := DRS.ofList [{ first with role := .other, id := 0 }]
    (commonHeaderContext (DRS.ofList []) [primary, alternative]).referents = [] := by decide

theorem commonMovedHeaderSuccessorDoesNotResurrectOldIdentity :
    let before := DRS.ofList [headerCreature]
    let moved := { headerCreature with id := 1, role := .obj, payload := .object [.creature] (some .graveyard) none none none none }
    let primary := DRS.ofList [moved]
    let alternative := DRS.ofList [{ moved with id := 2 }]
    let common := commonHeaderContext before [primary, alternative]
    common.findById? 0 = none ∧
      common.visibleReferents.map (fun r => (r.id, r.zone)) = [(1, some .graveyard)] ∧
      common.nextId = 3 := by decide

end Semantics.Proofs.EventPatterns
