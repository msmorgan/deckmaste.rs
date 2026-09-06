import Semantics.Macros
open Semantics Semantics.Macros

namespace Semantics.Proofs.TypeEvidence

theorem normalizeTypes_mem (t : CardType) (ts : List CardType) :
    t ∈ normalizeTypes ts ↔ t ∈ ts := by
  cases t <;> simp [normalizeTypes]

theorem normalizeTypes_ext (xs ys : List CardType)
    (h : ∀ t, t ∈ xs ↔ t ∈ ys) : normalizeTypes xs = normalizeTypes ys := by
  unfold normalizeTypes
  apply List.filter_congr
  intro t _
  simp only [h t]

theorem pureWritten (ts : List CardType) :
    Predicate.writtenTypesAll (ts.map Predicate.hasType) = normalizeTypes ts := by
  induction ts with
  | nil => rfl
  | cons t ts ih =>
    simp only [List.map_cons, Predicate.writtenTypesAll, Predicate.writtenTypes, ih]
    unfold mergeTypes
    apply normalizeTypes_ext
    intro u
    simp [normalizeTypes_mem]

theorem pureSeed (ts : List CardType) :
    Predicate.seedTy (.and (ts.map Predicate.hasType)) = normalizeTypes ts := by
  cases ts with
  | nil => rfl
  | cons t ts =>
    simp only [Predicate.seedTy, pureWritten]
    have h : (normalizeTypes (t :: ts)).isEmpty = false := by
      have hm := (normalizeTypes_mem t (t :: ts)).mpr (by simp)
      cases he : normalizeTypes (t :: ts) with
      | nil => simp [he] at hm
      | cons u us => rfl
    simp [h]

theorem pureTypePermutation (xs ys : List CardType) (h : xs.Perm ys) :
    Predicate.seedTy (.and (xs.map Predicate.hasType)) =
      Predicate.seedTy (.and (ys.map Predicate.hasType)) := by
  rw [pureSeed, pureSeed]
  apply normalizeTypes_ext
  intro t
  exact h.mem_iff

private def artifactCreature : Predicate := .and [artifact, creature]
private def creatureArtifact : Predicate := .and [creature, artifact]
private def afterTarget (p : Predicate) : Bindings := nomIntro [] (target p)

theorem artifactFirstReadsCreature :
    countReach (.word (.type .creature)) .one (afterTarget artifactCreature) = 1 := by decide

theorem creatureFirstReadsCreature :
    countReach (.word (.type .creature)) .one (afterTarget creatureArtifact) = 1 := by decide

theorem artifactFirstReadsArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget artifactCreature) = 1 := by decide

theorem creatureFirstReadsArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget creatureArtifact) = 1 := by decide

theorem targetTypeOrderKeepsBinding :
    afterTarget artifactCreature = afterTarget creatureArtifact := by rfl

theorem conjunctionDoesNotInventLand :
    countReach (.word (.type .land)) .one (afterTarget artifactCreature) = 0 := by decide

private def afterExile (p : Predicate) : Bindings := Instruction.intro [] (exile (target p))

theorem movedArtifactCreatureReadsCreatureCard :
    countReach (.word (.ofType .card .creature)) .one (afterExile artifactCreature) = 1 := by decide

theorem movedCreatureArtifactReadsArtifactCard :
    countReach (.word (.ofType .card .artifact)) .one (afterExile creatureArtifact) = 1 := by decide

theorem movedTypesKeepSameBinding :
    afterExile artifactCreature = afterExile creatureArtifact := by rfl

theorem movedCreatureWordRejectsFormerPermanent :
    countReach (.word (.type .creature)) .one (afterExile artifactCreature) = 0 := by decide

theorem movedArtifactWordRejectsFormerPermanent :
    countReach (.word (.type .artifact)) .one (afterExile creatureArtifact) = 0 := by decide

theorem movedEvidenceDoesNotInventLand :
    countReach (.word (.ofType .card .land)) .one (afterExile artifactCreature) = 0 := by decide

theorem disjunctionCannotPromiseCreature :
    countReach (.word (.type .creature)) .one (afterTarget (.or [artifact, creature])) = 0 := by decide

theorem disjunctionCannotPromiseArtifact :
    countReach (.word (.type .artifact)) .one (afterTarget (.or [artifact, creature])) = 0 := by decide

theorem disjunctionKeepsSharedCreature :
    countReach (.word (.type .creature)) .one
      (afterTarget (.or [artifactCreature, .and [land, creature]])) = 1 := by decide

theorem disjunctionDoesNotKeepUnsharedArtifact :
    countReach (.word (.type .artifact)) .one
      (afterTarget (.or [artifactCreature, .and [land, creature]])) = 0 := by decide

theorem conjunctionDistributesThroughAlternatives :
    Predicate.headTyAlts (.and [.or [artifact, land], creature]) =
      [[.creature, .artifact], [.creature, .land]] := by decide

private def joined : Bindings := afterTarget (.or [artifactCreature, .anyPlayer])

theorem joinedObjectHalfReadsCreature :
    countReach (.unionHalf (.type .creature)) .one joined = 1 := by decide

theorem joinedObjectHalfReadsArtifact :
    countReach (.unionHalf (.type .artifact)) .one joined = 1 := by decide

theorem joinedObjectHalfDoesNotInventLand :
    countReach (.unionHalf (.type .land)) .one joined = 0 := by decide

theorem joinedObjectHalfDoesNotBorrowBetweenAlternatives :
    countReach (.unionHalf (.type .artifact)) .one
      (afterTarget (.or [artifactCreature, creature, .anyPlayer])) = 0 := by decide

theorem multipleTypeRefinementsCanDescribeEquipmentHost :
    attachHeadOk .equipped (.ofType (.type .artifact) .creature) = true := by decide

theorem multipleTypeRefinementsCommuteForEquipmentHost :
    attachHeadOk .equipped (.ofType (.type .creature) .artifact) = true := by decide

theorem unrelatedTypeRefinementsDoNotDescribeEquipmentHost :
    attachHeadOk .equipped (.ofType (.type .artifact) .land) = false := by decide

theorem damageAcceptsArtifactCreature :
    Instruction.check [] (.dealDamage .this (.lit 1) (target artifactCreature)) = [] := by decide

theorem damageRejectsArtifactAlternative :
    Instruction.check [] (.dealDamage .this (.lit 1) (target (.or [artifact, creature]))) =
      [.damageRecipient] := by decide

private def equipmentHost : NounPhrase :=
  .attachHost .equipped (.ofType (.type .artifact) .creature)

theorem equipmentHostSeedsBothTypes :
    NounPhrase.selfSubjIntroduced equipmentHost =
      [⟨.the, .one, .object [.creature, .artifact] (some .battlefield) none none none⟩] := by rfl

theorem movedEquipmentHostKeepsCreatureCardEvidence :
    countReach (.word (.ofType .card .creature)) .one
      (Instruction.intro [] (exile equipmentHost)) = 1 := by decide

theorem movedEquipmentHostKeepsArtifactCardEvidence :
    countReach (.word (.ofType .card .artifact)) .one
      (Instruction.intro [] (exile equipmentHost)) = 1 := by decide

theorem movedEquipmentHostLosesPermanentCarrier :
    countReach (.word (.type .creature)) .one
      (Instruction.intro [] (exile equipmentHost)) = 0 := by decide

end Semantics.Proofs.TypeEvidence
