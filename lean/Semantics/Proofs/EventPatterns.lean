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
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount .any)) = [] := by decide

theorem damageSourceIsObject :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.dealDamage .anyAmount .any)) =
      [.kindMismatch .object .player] := by decide

theorem recipientMayBePlayer :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount .you (some .combat))) = [] := by decide

theorem sourcePrecedesReflexiveRecipientReference :
    GameEvent.check (DRS.ofList [])
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount (.refl .one))) = [] := by decide

theorem bareRecipientCannotReferToCoargumentSource :
    GameEvent.check (DRS.ofList [])
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount it)) =
      [.anaphor .bare .one 0] := by decide

theorem absentSourceCannotSupplyReference :
    GameEvent.check (DRS.ofList []) (.does .any (.dealDamage .anyAmount it)) =
      [.anaphor .bare .one 0] := by decide

theorem occurredDamageComposesSourceThenBodyForEveryContext
    (fuel : Nat) (drs : DRS) (source patient : NounPhrase) (kind : Option DamageKind) :
    let subjectContext := (stepEventSubjectWith (stepOps fuel) drs (.actor source) .postEvent
      (.dealDamage .anyAmount patient kind)).1
    let bodyContext := (stepAction fuel .postEvent .obj
      { subjectContext with actionPattern := true } (.dealDamage .anyAmount patient kind)).1
    (stepGameEvent (fuel + 1) drs
      (.does (.actor source) (.dealDamage .anyAmount patient kind))).1 =
      { (dropActorFrame bodyContext) with actionPattern := drs.actionPattern } := by rfl

/-- Every occurred damage event publishes its outcome ahead of participant referents. -/
theorem damageOutcomePrecedesParticipantsForEveryContext
    (drs : DRS) (kind : Option DamageKind) (source patient : NounPhrase) :
    ((GameEvent.extend drs
      (.does (.actor source) (.dealDamage .anyAmount patient kind))).referents.head?).map
        (·.payload) = some (.outcome .damageDealt) := by rfl

theorem damagePublishesParticipantsAndOutcome :
    let after := GameEvent.extend (DRS.ofList [])
      (.does (.actor (a isCreature)) (.dealDamage .anyAmount (a .anyPlayer)))
    countOutcomes .damageDealt after = 1 ∧
      (after.findById? 0).map (·.role) = some .subj ∧
      (after.findById? 2).map (·.role) = some .obj := by decide

theorem historicalDamagePublishesItsOutcomeLocallyForEveryContext
    (drs : DRS) (kind : Option DamageKind) (source patient : NounPhrase) :
    ((stepDidClause 64 drs
      ⟨.actor source, .dealDamage .anyAmount patient kind, some .thisWay⟩).1.referents.head?).map
        (·.payload) = some (.outcome .damageDealt) := by rfl

theorem observedDamageDoesNotExportOrRefineReferents (kind : Option DamageKind) :
    let before := nounResult (DRS.ofList []) (target isArtifact)
    let after := (stepPredicate 64 before (.did ⟨.actor .which,
      .dealDamage .anyAmount .you kind, some .thisWay⟩)).1
    after.referents = before.referents ∧ countOutcomes .damageDealt after = 0 := by
  rcases kind with _ | _ | _ <;> decide

theorem patientOnlyDoesNotManufactureSource :
    (GameEvent.extend (DRS.ofList [])
      (.does .any (.dealDamage .anyAmount thisCreature))).performer = none := by decide

theorem entryAndAttachmentChoicesRemainDistinct :
    entersChoosing thisCreature .color ≠ attachChoosing thisCreature .color := by
  intro equal
  cases equal

theorem choiceDisclosureRemainsDistinct :
    entersChoosingPlayer thisCreature anOpponent ≠
      entersChoosingPlayerSecretly thisCreature anOpponent := by
  intro equal
  cases equal

/-- A quality or a player chosen as a permanent enters, openly or secretly, or as it becomes
attached, is read by a later line of its text ("the chosen color", "the chosen player"), which
cannot read it without the choice. -/
theorem entryAndAttachmentChoicesPublishTheirSort :
    let colorReader := Ability.static (.characteristicChange .any
      [.quality .adds (.chosen (.ofChosen .theChoice .color))])
    let playerReader := at_ (beginningOfPossessed .upkeep (the isChosenPlayer)) (loseLife 1)
    Ability.checkText (DRS.ofList []) [.static (entersChoosing thisCreature .color), colorReader]
        = [] ∧
      Ability.checkText (DRS.ofList []) [.static (attachChoosing thisCreature .color), colorReader]
        = [] ∧
      Ability.checkText (DRS.ofList [])
          [.static (entersChoosingPlayer thisCreature anOpponent), playerReader] = [] ∧
      Ability.checkText (DRS.ofList [])
          [.static (entersChoosingPlayerSecretly thisCreature anOpponent), playerReader] = [] ∧
      Ability.checkText (DRS.ofList [])
          [.static (attachmentChoice thisCreature anOpponent), playerReader] = [] ∧
      Ability.checkText (DRS.ofList []) [colorReader] ≠ [] ∧
      Ability.checkText (DRS.ofList []) [playerReader] ≠ [] := by
  decide

theorem entryAndAttachmentChoicesCheckTheirDomain :
    StaticSpec.check (DRS.ofList [])
        (entryChoice thisCreature (a (qualityFrom .color (.players (.isOpponentOf .you))))) =
      [.kindAxisSort] ∧
      StaticSpec.check (DRS.ofList [])
          (attachmentChoice thisCreature (a (qualityFrom .color (.players (.isOpponentOf .you))))) =
        [.kindAxisSort] := by
  decide

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
      .move .which (battlefield) (some graveyard) .departure none [], some .thisWay⟩)).1
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
      (partScope .turn (some (that .player))
        (.manaRetention (a .anyPlayer) (.unspent none))) = [] := by decide

theorem applicabilityPreservesNumericDefinition :
    Amount.check (stepStaticSpec 64 (DRS.ofList [])
      (partScope .turn none (.letterDefinition .x 2))).1
      (.letter .x) = [] := by decide

theorem ownedOutsideCardsAreAdmitted :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (allOf (.and [.isCard, isCreature, .hasPossessor .owner .you, .not (.in_ battlefield)])) =
      [] := by decide

theorem controlledCreatureSpellsRemainIncluded :
    StaticSpec.check (DRS.ofList []) (.characteristicChange
      (allOf (.and [isCreature, isSpell, .hasPossessor .controller .you]))
      [.quality .adds (.everyTypeOf .creature)]) = [] := by decide

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
    payload := .object [.creature] (some .battlefield) none none none }

private def headerArtifact : Referent :=
  { det := .a, plur := .one, role := .other,
    payload := .object [.artifact] (some .battlefield) none none none }

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
    let moved := { headerCreature with id := 1, role := .obj, payload := .object [.creature] (some .graveyard) none none none }
    let primary := DRS.ofList [moved]
    let alternative := DRS.ofList [{ moved with id := 2 }]
    let common := commonHeaderContext before [primary, alternative]
    common.findById? 0 = none ∧
      common.visibleReferents.map (fun r => (r.id, r.zone)) = [(1, some .graveyard)] ∧
      common.nextId = 3 := by decide

/-! ## Control changes

"Gain control of …" as an instruction is a Continuous Effect, `establish (controlGrant …)`
[CR#611.2a,613.1b]. As a trigger event (one that looks back in time [CR#603.10d]) or a forbidden
act it names the control change, `gainControlOf` (`Action.changeControl`). -/

/-- The imperative's tree is unchanged: `gainControl` is the layer-2 effect. -/
theorem gainControlImperativeIsTheControlGrant (subject controller : NounPhrase)
    (duration : Option Duration) :
    gainControl subject duration (controller := controller) =
      .establish (.controlGrant controller subject) duration := rfl

/-- A control change in a resolving position is refused; as an event or a deontic it checks. -/
theorem controlChangeIsOnlyAPattern :
    Action.check (DRS.ofList []) (gainControlOf (target isCreature)) = [.controlChangePattern] ∧
      GameEvent.check (DRS.ofList [])
        (.does (.actor anOpponent) (gainControlOf (a isPermanent) (some you))) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (cant (bare (and [isPlayer, isOther])) (gainControlOf thisCreature)) = [] := by decide

/-- The player control is taken from is a player. -/
theorem controlIsTakenFromAPlayer :
    GameEvent.check (DRS.ofList [])
      (.does (.actor anOpponent) (gainControlOf (a isPermanent) (some (a isCreature)))) =
        [.kindMismatch .player .object] := by
  decide

/-- The control change stamps its patient as `controlGrant` does, so "the permanent … gained
control of" reads either. -/
theorem controlChangeStampsItsPatientAsTheGrantDoes :
    let after := GameEvent.extend (DRS.ofList []) (.does (.actor anOpponent) (gainControlOf (a isPermanent)))
    (participialCandidates (.deed (.core .gainControl)) (some .one) none false
      (nextClause after)).length = 1 := by decide

end Semantics.Proofs.EventPatterns
