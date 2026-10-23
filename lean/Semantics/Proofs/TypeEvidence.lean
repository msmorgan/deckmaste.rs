import Semantics.Definitions
open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.TypeEvidence

/-- Normalizing keeps exactly the declared card types written: the order is the declared one
(`cardTypes`), and a raw `CardType` no card can spell is not remembered. -/
theorem normalizeTypes_mem (t : CardType) (ts : List CardType) :
    t ∈ normalizeTypes ts ↔ t ∈ cardTypes ∧ t ∈ ts := by
  simp [normalizeTypes]

/-- A declared card type is remembered exactly when it is written. -/
theorem normalizeTypes_mem_of_declared (t : CardType) (ts : List CardType) (h : t ∈ cardTypes) :
    t ∈ normalizeTypes ts ↔ t ∈ ts := by
  simp [normalizeTypes_mem, h]

/-- Two lists writing the same declared card types normalize alike. -/
theorem normalizeTypes_ext (xs ys : List CardType)
    (h : ∀ t, t ∈ cardTypes → (t ∈ xs ↔ t ∈ ys)) : normalizeTypes xs = normalizeTypes ys := by
  unfold normalizeTypes
  apply List.filter_congr
  intro t ht
  simp only [h t ht]

theorem pureWritten (ts : List CardType) :
    (ts.map Predicate.hasType).foldr (fun p acc => mergeTypes (p.seedTy 31) acc) [] =
      normalizeTypes ts := by
  induction ts with
  | nil => rfl
  | cons t ts ih =>
    simp only [List.map_cons, List.foldr_cons, Predicate.seedTy, ih]
    unfold mergeTypes
    apply normalizeTypes_ext
    intro u hu
    simp [normalizeTypes_mem, hu]

theorem pureSeed (ts : List CardType) :
    Predicate.seedTy (.and (ts.map Predicate.hasType)) = normalizeTypes ts := by
  exact pureWritten ts

theorem pureTypePermutation (xs ys : List CardType) (h : xs.Perm ys) :
    Predicate.seedTy (.and (xs.map Predicate.hasType)) =
      Predicate.seedTy (.and (ys.map Predicate.hasType)) := by
  rw [pureSeed, pureSeed]
  apply normalizeTypes_ext
  intro t _
  exact h.mem_iff

private def artifactCreature : Predicate := .and [isArtifact, isCreature]
private def creatureArtifact : Predicate := .and [isCreature, isArtifact]
private def afterTarget (p : Predicate) : DRS := (stepNounPhrase 64 (DRS.ofList []) .other false (target p)).1

theorem artifactFirstReadsCreature :
    countReach (.word (.type .creature)) .one (afterTarget artifactCreature) = 1 := by decide

theorem creatureFirstReadsCreature :
    countReach (.word (.type .creature)) .one (afterTarget creatureArtifact) = 1 := by decide

theorem artifactFirstReadsArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget artifactCreature) = 1 := by decide

theorem creatureFirstReadsArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget creatureArtifact) = 1 := by decide

theorem targetTypeOrderKeepsBinding :
    afterTarget artifactCreature = afterTarget creatureArtifact := by decide

theorem conjunctionDoesNotInventLand :
    countReach (.word (.type .land)) .one (afterTarget artifactCreature) = 0 := by decide

private def afterExile (p : Predicate) : DRS := (stepAction 64 .postEvent .obj (DRS.ofList []) (exile (target p))).1

theorem movedArtifactCreatureReadsCreatureCard :
    countReach (.word (.ofType .card .creature)) .one (afterExile artifactCreature) = 1 := by decide

theorem movedCreatureArtifactReadsArtifactCard :
    countReach (.word (.ofType .card .artifact)) .one (afterExile creatureArtifact) = 1 := by decide

theorem movedTypesKeepSameBinding :
    afterExile artifactCreature = afterExile creatureArtifact := by decide

theorem movedCreatureWordRejectsFormerPermanent :
    countReach (.word (.type .creature)) .one (afterExile artifactCreature) = 0 := by decide

theorem movedArtifactWordRejectsFormerPermanent :
    countReach (.word (.type .artifact)) .one (afterExile creatureArtifact) = 0 := by decide

theorem movedEvidenceDoesNotInventLand :
    countReach (.word (.ofType .card .land)) .one (afterExile artifactCreature) = 0 := by decide

theorem disjunctionCannotPromiseCreature :
    countReach (.word (.type .creature)) .one (afterTarget (.or [isArtifact, isCreature])) = 0 := by decide

theorem disjunctionCannotPromiseArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget (.or [isArtifact, isCreature])) = 0 := by decide

theorem disjunctionKeepsSharedCreature :
    countReach (.word (.type .creature)) .one
      (afterTarget (.or [artifactCreature, .and [isLand, isCreature]])) = 1 := by decide

theorem disjunctionDoesNotKeepUnsharedArtifact :
    countReach (.word (.type .artifact)) .one
      (afterTarget (.or [artifactCreature, .and [isLand, isCreature]])) = 0 := by decide

theorem conjunctionDistributesThroughAlternatives :
    Predicate.headTyAlts (.and [.or [isArtifact, isLand], isCreature]) =
      [[.creature, .artifact], [.creature, .land]] := by decide

private def joined : DRS := afterTarget (.or [artifactCreature, .anyPlayer])

theorem joinedObjectHalfReadsCreature :
    countReach (.unionHalf (.type .creature)) .one joined = 1 := by decide

theorem joinedObjectHalfReadsArtifact :
    countReach (.unionHalf (.type .artifact)) .one joined = 1 := by decide

theorem joinedObjectHalfDoesNotInventLand :
    countReach (.unionHalf (.type .land)) .one joined = 0 := by decide

theorem joinedObjectHalfDoesNotBorrowBetweenAlternatives :
    countReach (.unionHalf (.type .artifact)) .one
      (afterTarget (.or [artifactCreature, isCreature, .anyPlayer])) = 0 := by decide

theorem damageAcceptsArtifactCreature :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 1 (target artifactCreature))) = [] := by decide

private def equipmentHost : NounPhrase :=
  .attachHost .equipped (.ofType (.type .artifact) .creature)

theorem equipmentHostSeedsBothTypes :
    ((stepNounPhrase 64 (DRS.ofList []) .subj true equipmentHost).1.referents.map (·.ty)) =
      [[.creature, .artifact]] := by decide

theorem movedEquipmentHostKeepsCreatureCardEvidence :
    countReach (.word (.ofType .card .creature)) .one
      (stepAction 64 .postEvent .obj (DRS.ofList []) (exile equipmentHost)).1 = 1 := by decide

theorem movedEquipmentHostKeepsArtifactCardEvidence :
    countReach (.word (.ofType .card .artifact)) .one
      (stepAction 64 .postEvent .obj (DRS.ofList []) (exile equipmentHost)).1 = 1 := by decide

theorem movedEquipmentHostLosesPermanentCarrier :
    countReach (.word (.type .creature)) .one
      (stepAction 64 .postEvent .obj (DRS.ofList []) (exile equipmentHost)).1 = 0 := by decide

end Semantics.Proofs.TypeEvidence
