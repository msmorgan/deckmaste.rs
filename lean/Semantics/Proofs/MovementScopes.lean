import Semantics.Check.Phrase

open Semantics

namespace Semantics.Proofs.MovementScopes

private def original : DRS := DRS.ofList [
  { idents := ["subject"], det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none none none }]

private def moved : DRS :=
  applyZoneMove original [0] (some (.action "exile")) (some .exile)

theorem zoneChangeCreatesFreshIdentity :
    moved.referents.map (·.id) = [1] ∧ moved.nextId = 2 := by decide

theorem zoneChangeClearsParameterIdentity :
    identityBinding "subject" moved = none := by decide

theorem zoneChangePreservesTypeAndUpdatesZone :
    moved.referents.map (fun r => (r.ty, r.zone)) = [([.creature], some .exile)] := by decide

theorem unknownDestinationForgetsOldZone :
    (applyZoneMove original [0] none none).referents.map (·.zone) = [none] := by decide

theorem abilityZoneChangeCreatesFreshIdentity :
    let before := DRS.ofList [{ det := .a, plur := .one, payload := .ability none (some .stack) }]
    let after := applyZoneMove before [0] (some (.action "exile")) (some .exile)
    after.referents.map (fun r => (r.id, r.zone)) = [(1, some .exile)] := by decide

private def callerPart : Referent :=
  { det := .part, plur := .one, payload := .object [.creature] (some .battlefield) none none none none }
private def callerPlayer : Referent :=
  { det := .a, plur := .one, payload := .player false }

private def argumentResult (r : Referent) : DRS :=
  let inner := enterOwnScope (DRS.ofList [])
  leaveCaller inner ((enterCaller inner).push r .obj true)

theorem callerPartSurvivesDefinition :
    (leaveOwnScope (argumentResult callerPart)).referents.length = 1 := by decide

theorem callerPlayerSurvivesDefinition :
    (leaveOwnScope (argumentResult callerPlayer)).referents.length = 1 := by decide

theorem definitionInternalPartDoesNotEscape :
    (leaveOwnScope ((enterOwnScope (DRS.ofList [])).push callerPart .obj true)).referents = [] := by decide

theorem definitionInternalPlayerDoesNotEscape :
    (leaveOwnScope ((enterOwnScope (DRS.ofList [])).push callerPlayer .obj true)).referents = [] := by decide

theorem movedCallerPartSurvivesDefinition :
    let after := leaveOwnScope
      (applyZoneMove (argumentResult callerPart) [0] (some (.action "exile")) (some .exile))
    after.referents.map (fun r => (r.id, r.zone)) = [(1, some .exile)] := by decide

theorem entrySnapshotRemainsImmutable :
    let after := applyZoneMove (enterOwnScope original) [0] none (some .exile)
    after.scopeStack.map (·.preEntryIds) = [[0]] := by decide

theorem movedMaskedArgumentRemainsMasked :
    let after := applyZoneMove (enterOwnScope original) [0] none (some .exile)
    after.visibleReferents = [] := by decide

theorem departedPermanentCannotBeActionPatient :
    resolveCandidates (.word .permanent) .one moved = [] := by decide

theorem departedPermanentRemainsAvailableForCharacteristicRead :
    (resolveCandidates (.word .permanent) .one { moved with characteristicRead := true }).map
      (·.id) = [1] := by decide

theorem joinedDepartedCreatureCannotBypassZoneFilter :
    let context := DRS.ofList [{
      det := .a, plur := .one,
      payload := .join (.player false)
        (.object [.creature] (some .graveyard) none none none none) }]
    resolveCandidates (.word (.type .creature)) .one context = [] := by decide

theorem shuffleForgetsMaskedLibraryIdentity :
    let context := DRS.ofList [{
      det := .a, plur := .one, idents := ["library.card"],
      payload := .object [.creature] (some .library) none none none none }]
    let after := leaveOwnScope (afterShuffle (enterOwnScope context))
    after.referents = [] ∧ identityBinding "library.card" after = none := by decide

private def relativeResult : DRS :=
  let inner := (DRS.ofList []).push callerPart .other false
  let inner := inner.push callerPlayer .other false
  inner.push { callerPart with det := .the, attachmentHost := true } .subj true

theorem relativeClauseExportsOnlyAttachmentHost :
    let after := leaveRelative (DRS.ofList []) relativeResult
    after.referents.map (·.id) = [2] ∧ after.nextId = 3 := by decide

theorem projectedHostDoesNotReuseLocalIdentity :
    let after := (leaveRelative (DRS.ofList []) relativeResult).push callerPlayer .obj true
    after.referents.map (·.id) = [3, 2] := by decide

theorem explicitCallerArgumentPreservesAuxiliaryPlayer :
    let inner := enterOwnScope (DRS.ofList [])
    let argument := ((enterCaller inner).push callerPlayer .other false).push callerPart .obj true
    let after := leaveOwnScope (leaveCaller inner argument [1])
    after.referents.map (·.id) = [1, 0] := by decide

end Semantics.Proofs.MovementScopes
