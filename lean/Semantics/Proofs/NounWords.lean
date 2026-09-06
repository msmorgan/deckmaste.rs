import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.NounWords

private def creatureCard : Binding :=
  ⟨.target, .one, .object [.creature] (some .exile) none none none⟩
private def artifactCard : Binding :=
  ⟨.target, .one, .object [.artifact] (some .exile) none none none⟩
private def creaturePermanent : Binding :=
  ⟨.target, .one, .object [.creature] (some .battlefield) none none none⟩
private def sourceAbility : Binding := ⟨.target, .one, .ability none⟩
private def copiedAbility : Binding := ⟨.a, .one, .ability (some .copy)⟩

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
      [⟨.self, .one, .ability (some .copy)⟩] = 0 := by decide

theorem unrefinedAbilityRemainsAmbiguous :
    countReach (.word .ability) .one [sourceAbility, copiedAbility] = 2 := by decide

theorem copiedAbilityRemainsAmbiguousWithTwoCopies :
    countReach (.word (.copied .ability)) .one [copiedAbility, copiedAbility] = 2 := by decide

theorem wordRefinementDoesNotBorrowTypeFromAnotherHalf :
    countReach (.word (.ofType .card .creature)) .one
      [⟨.target, .one, .join artifactCard.payload creaturePermanent.payload⟩] = 0 := by decide

private def copiedArtifact : Payload :=
  .object [.artifact] (some .battlefield) none (some .copy) none
private def copiedCreature : Payload :=
  .object [.creature] (some .battlefield) none (some .copy) none

theorem copiedAbilityRejectsCopiedObject :
    countReach (.word (.copied .ability)) .one
      [⟨.a, .one, copiedCreature⟩] = 0 := by decide

theorem unionRefinementDoesNotBorrowOriginFromAnotherHalf :
    countReach (.unionHalf (.copied (.type .creature))) .one
      [⟨.target, .one, .join copiedArtifact creaturePermanent.payload⟩] = 0 := by decide

theorem unionRefinementAcceptsOneMatchingHalf :
    countReach (.unionHalf (.copied (.type .creature))) .one
      [⟨.target, .one, .join artifactCard.payload copiedCreature⟩] = 1 := by decide

private def movedCopy (deed : Deed) (origin : Option Origin) : Binding :=
  ⟨.target, .one,
    .object [.creature] (some .exile) (some ⟨deed, true, true⟩) origin none⟩

theorem verbedRefinementsKeepCarrierTypeAndOrigin :
    countReach (.verbed (.action "Exile") (.copied (.ofType .card .creature)) .attributive)
      .one [movedCopy (.action "Exile") (some .copy)] = 1 := by decide

theorem verbedRefinementsRejectWrongDeed :
    countReach (.verbed (.action "Exile") (.copied (.ofType .card .creature)) .attributive)
      .one [movedCopy (.action "Destroy") (some .copy)] = 0 := by decide

theorem verbedRefinementsRejectOriginal :
    countReach (.verbed (.action "Exile") (.copied (.ofType .card .creature)) .attributive)
      .one [movedCopy (.action "Exile") none] = 0 := by decide

theorem typeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (b : Binding) :
    wordReaches (.copied (.ofType word ty)) b =
      wordReaches (.ofType (.copied word) ty) b := by
  simp only [wordReaches, Bool.and_right_comm]

theorem halfTypeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (pl : Payload) :
    halfWordReaches (.copied (.ofType word ty)) pl =
      halfWordReaches (.ofType (.copied word) ty) pl := by
  simp only [halfWordReaches, Bool.and_right_comm]

theorem verbedTypeAndCopyRefinementsCommute (word : NounWord) (ty : CardType) (st : Stamp)
    (remembered : List CardType) (zone : Option Zone) (origin : Option Origin) :
    verbedWordOk (.copied (.ofType word ty)) st remembered zone origin =
      verbedWordOk (.ofType (.copied word) ty) st remembered zone origin := by
  simp only [verbedWordOk, Bool.and_right_comm]

end Semantics.Proofs.NounWords
