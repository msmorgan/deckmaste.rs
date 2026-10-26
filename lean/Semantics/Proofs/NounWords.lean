import Semantics.Definitions
import Semantics.Check.Card

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
  { det := .target, plur := .one, idents := [],
    payload := .object [.creature] (some .exile)
      (some (moveStamp (some deed) (some .battlefield) (some .exile))) origin none }

theorem participialRefinementsKeepCarrierTypeAndOrigin :
    (participialCandidates (.deed (.action "exile")) (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "exile") (some .copy)]).length = 1 := by decide

theorem participialRefinementsRejectWrongDeed :
    (participialCandidates (.deed (.action "exile")) (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "destroy") (some .copy)]).length = 0 := by decide

theorem participialRefinementsRejectOriginal :
    (participialCandidates (.deed (.action "exile")) (some .one)
      (some (.copied (.ofType .card .creature))) false
      [movedCopy (.action "exile") none]).length = 0 := by decide

/-! A Participial Description reads the object its deed stamped whether or not its other
conjuncts name a Head Noun: "the exiled card" and the bare "the exiled one" both reach it. -/

theorem participialWithoutHeadNounReachesTheStampedObject :
    (participialCandidates (.deed (.action "exile")) (some .one) none false
      [movedCopy (.action "exile") none]).length = 1 := by decide

theorem participialWithHeadNounReachesTheStampedObject :
    (participialCandidates (.deed (.action "exile")) (some .one) (some .card) false
      [movedCopy (.action "exile") none]).length = 1 := by decide

theorem participialRejectsWrongDeed :
    (participialCandidates (.deed (.action "exile")) (some .one) none false
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
    Action.check (DRS.ofList []) (instruct selectedOpponent (draw 1)) = [] := by
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

/-! ## "That source" -/

private def chosenSource : Referent :=
  { det := .a, plur := .one, payload := .object [] none none none none,
    introducedAsSource := true }
private def chosenSourceOnStack : Referent :=
  { chosenSource with payload := .object [.instant] (some .stack) none none none }
private def theSource : Referent :=
  { det := .self, plur := .one, payload := .object [.instant] (some .stack) none none none,
    introducedAsSource := true }

/-- "That source" reads an antecedent introduced as a source, in any zone [CR#609.7a]. -/
theorem sourceWordReadsAChosenSourceInAnyZone :
    countReach (.word .source) .one [chosenSource] = 1 ∧
      countReach (.word .source) .one [chosenSourceOnStack] = 1 := by
  decide

/-- An object not introduced as a source is no antecedent of "that source", whatever its zone. -/
theorem sourceWordRejectsAnObjectNotIntroducedAsASource :
    countReach (.word .source) .one [creaturePermanent, creatureCard, sourceAbility] = 0 := by
  decide

/-- The Source of the text is never read by "that source". -/
theorem sourceWordNeverReadsTheSource :
    countReach (.word .source) .one [theSource] = 0 := by
  decide

/-- Introducing an object by a description that names it a source records it as one. -/
theorem sourceDescriptionRecordsTheFlag :
    (bindFor .a .one .object isSource).introducedAsSource = true ∧
      (bindFor .a .one .object (and [isSource, hasColor .red])).introducedAsSource = true ∧
      (bindFor .a .one .object isCreature).introducedAsSource = false := by
  decide

private def departedSpell (origin : Zone := .stack) : Referent :=
  { det := .target, plur := .one,
    payload := .object [.instant] (some .graveyard)
      (some (moveStamp (some (.action "counter")) (some origin) (some .graveyard))) none none }

/-- A departed spell is readable as "spell" or a Spell Type only for information reads
[CR#608.2h]; neither an action slot nor a Participial Description gets that exception. -/
theorem departedSpellInformationReadOnly :
    let ref := departedSpell
    reachesCharacteristic (.word .spell) .one ref = true ∧
      reachesCharacteristic (.word (.subtype (.spell "Arcane"))) .one ref = true ∧
      reaches (.atSlot .spell) .one ref = false ∧
      reaches (.word .spell) .one ref = false ∧
      (participialCandidates (.deed (.action "counter")) (some .one) (some .spell) false
        (DRS.ofList [ref])).isEmpty = true ∧
      reachesCharacteristic (.word .spell) .one (departedSpell .hand) = false := by decide

/-- Countering an already countered spell remains refused [CR#109.2b]. -/
theorem badCounterDepartedSpell :
    Action.check (DRS.ofList [])
      (sequentially [counter (target isSpell), counter (that .spell)]) =
        [.anaphor (.word .spell) .one 0] := by decide

/-- A possessor read may explicitly name the departed spell [CR#608.2h]. -/
theorem departedSpellsControllerIsReadable :
    Action.check (DRS.ofList [])
      (sequentially [counter (target isSpell),
        instruct (controllerOf (that .spell)) (draw 1)]) = [] := by decide

/-- A historical counter reads its departed spell; it performs no second counter [CR#608.2h]. -/
theorem counteredThisWayReadsDepartedSpell :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) (counter (target isSpell))).1
    Condition.check (nextClause after) (did any (counter (that .spell)) thisWay) = [] := by decide

end Semantics.Proofs.NounWords
