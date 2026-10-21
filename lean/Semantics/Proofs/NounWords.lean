import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.NounWords

private def creatureCard : Referent :=
  { det := .target, plur := .one, payload := .object [.creature] (some .exile) none none none, idents := [] }
private def artifactCard : Referent :=
  { det := .target, plur := .one, payload := .object [.artifact] (some .exile) none none none, idents := [] }
private def creaturePermanent : Referent :=
  { det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] }
private def sourceAbility : Referent := { det := .target, plur := .one, payload := .ability none, idents := [] }
private def copiedAbility : Referent := { det := .a, plur := .one, payload := .ability (some .copy), idents := [] }

theorem typedCardFindsMatchingCarrier :
    countReach (.word (.ofType .card .creature)) .one
      [artifactCard, creaturePermanent, creatureCard] = 1 := by decide

theorem typedCardSelectsMatchingIdentity :
    firstReach (.word (.ofType .card .creature)) .one
      [artifactCard, creaturePermanent, creatureCard] = some creatureCard := by rfl

theorem typedCardRejectsIncompatibleType :
    countReach (.word (.ofType .card .creature)) .one [artifactCard] = 0 := by decide

theorem typedCardRejectsPermanentCarrier :
    countReach (.word (.ofType .card .creature)) .one [creaturePermanent] = 0 := by decide

theorem typedCardRejectsAbilityCarrier :
    countReach (.word (.ofType .card .creature)) .one [sourceAbility] = 0 := by decide

theorem copiedAbilityRejectsOriginal :
    countReach (.word (.copied .ability)) .one [sourceAbility] = 0 := by decide

theorem copiedAbilityFindsCopy :
    firstReach (.word (.copied .ability)) .one [sourceAbility, copiedAbility] =
      some copiedAbility := by rfl

theorem copiedAbilityIsUniqueBesideOriginal :
    countReach (.word (.copied .ability)) .one [sourceAbility, copiedAbility] = 1 := by decide

theorem refinedWordStillExcludesSelf :
    countReach (.word (.copied .ability)) .one
      (DRS.ofList [{ det := .self, plur := .one, payload := .ability (some .copy), idents := [] }]) = 0 := by decide

theorem unrefinedAbilitySelectsMostRecent :
    countReach (.word .ability) .one [sourceAbility, copiedAbility] = 1 := by decide

theorem unrefinedAbilitySelectsMostRecentIdentity :
    (firstReach (.word .ability) .one [sourceAbility, copiedAbility]).map (·.id) = some 1 := by decide

theorem copiedAbilitySelectsMostRecentOfTwoCopies :
    countReach (.word (.copied .ability)) .one [copiedAbility, copiedAbility] = 1 := by decide

theorem copiedAbilitySelectsMostRecentIdentityOfTwoCopies :
    (firstReach (.word (.copied .ability)) .one [copiedAbility, copiedAbility]).map (·.id) = some 1 := by decide

theorem wordRefinementDoesNotBorrowTypeFromAnotherHalf :
    countReach (.word (.ofType .card .creature)) .one
      (DRS.ofList [{ det := .target, plur := .one, payload := .join artifactCard.payload creaturePermanent.payload, idents := [] }]) = 0 := by decide

private def copiedArtifact : Payload :=
  .object [.artifact] (some .battlefield) none (some .copy) none
private def copiedCreature : Payload :=
  .object [.creature] (some .battlefield) none (some .copy) none

theorem copiedAbilityRejectsCopiedObject :
    countReach (.word (.copied .ability)) .one
      (DRS.ofList [{ det := .a, plur := .one, payload := copiedCreature, idents := [] }]) = 0 := by decide

theorem unionRefinementDoesNotBorrowOriginFromAnotherHalf :
    countReach (.unionHalf (.copied (.type .creature))) .one
      (DRS.ofList [{ det := .target, plur := .one, payload := .join copiedArtifact creaturePermanent.payload, idents := [] }]) = 0 := by decide

theorem unionRefinementAcceptsOneMatchingHalf :
    countReach (.unionHalf (.copied (.type .creature))) .one
      (DRS.ofList [{ det := .target, plur := .one, payload := .join artifactCard.payload copiedCreature, idents := [] }]) = 1 := by decide

private def movedCopy (deed : Deed) (origin : Option Origin) : Referent :=
  { det := .target, plur := .one, payload := .object [.creature] (some .exile) (some ⟨deed, true, true, []⟩) origin none, idents := [] }

theorem participialRefinementsKeepCarrierTypeAndOrigin :
    (participialCandidates (.action "exile") (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "exile") (some .copy)]).length = 1 := by decide

theorem participialRefinementsRejectWrongDeed :
    (participialCandidates (.action "exile") (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "destroy") (some .copy)]).length = 0 := by decide

theorem participialRefinementsRejectOriginal :
    (participialCandidates (.action "exile") (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "exile") none]).length = 0 := by decide

/-! A Participial Description reads the object its deed stamped whether or not its other
conjuncts name a Head Noun: "the exiled card" and the bare "the exiled one" both reach it. -/

theorem participialWithoutHeadNounReachesTheStampedObject :
    (participialCandidates (.action "exile") (some .one) none false
      [movedCopy (.action "exile") none]).length = 1 := by decide

theorem participialWithHeadNounReachesTheStampedObject :
    (participialCandidates (.action "exile") (some .one) (some .card) false
      [movedCopy (.action "exile") none]).length = 1 := by decide

theorem participialRejectsWrongDeed :
    (participialCandidates (.action "exile") (some .one) none false
      [movedCopy (.action "destroy") none]).length = 0 := by decide

theorem typeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (b : Referent) :
    wordReaches (.copied (.ofType word ty)) b =
      wordReaches (.ofType (.copied word) ty) b := by
  simp only [wordReaches, Bool.and_right_comm]

theorem halfTypeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (pl : Payload) :
    halfWordReaches (.copied (.ofType word ty)) pl =
      halfWordReaches (.ofType (.copied word) ty) pl := by
  simp only [halfWordReaches, Bool.and_right_comm]

theorem stampedTypeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (st : Stamp)
    (remembered : List CardType) (zone : Option Zone) (origin : Option Origin) :
    stampedWordOk (.copied (.ofType word ty)) st remembered zone origin =
      stampedWordOk (.ofType (.copied word) ty) st remembered zone origin := by
  simp only [stampedWordOk, Bool.and_right_comm]

private def selectedOpponent : NounPhrase :=
  .someOf (.counted (exactly 1)) none (allOf (.isOpponentOf .you))

theorem playerSubsetRetainsItsGroupKindAndIdentity :
    let (after, ids, errors) := stepNounPhrase 64 (DRS.ofList []) .other false selectedOpponent
    errors = [] ∧ ids = [1] ∧ selectedOpponent.kind? = some .player ∧
      (after.findById? 0).map (fun ref => (ref.kind, ref.plur)) =
        some (.player, .many) ∧
      (after.findById? 1).map (fun ref => (ref.kind, ref.plur, ref.det, ref.payload)) =
        some (.player, .one, .part, .player false) ∧
      after.selectionEvidence =
        [{ kind := .player, sourceGroup := some 0, chosen := false, scope := 0 }] := by decide

theorem playerSubsetCanPerformAPlayerAction :
    Action.check (DRS.ofList []) (instruct selectedOpponent (draw (.lit 1))) = [] := by
  decide

theorem playerSubsetCannotFillAnObjectContext :
    NounPhrase.checkAt (some .object) (DRS.ofList []) selectedOpponent =
      [.kindMismatch .object .player] := by decide

theorem objectSubsetRetainsItsTypeZoneAndExactSize :
    let (after, ids, errors) := stepNounPhrase 64 (DRS.ofList []) .other false
      (.someOf (.counted (exactly 1)) none (allOf isCreature))
    errors = [] ∧ ids = [1] ∧
      (after.findById? 1).map (·.payload) =
        some (.object [.creature] (some .battlefield) none none (some 1)) := by decide

end Semantics.Proofs.NounWords
