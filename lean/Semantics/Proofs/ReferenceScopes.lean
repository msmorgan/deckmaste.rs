import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.ReferenceScopes

private def creatureBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none (some 1) none }
private def artifactBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.artifact] (some .battlefield) none none (some 1) none }
private def playerBinding : Referent :=
  { det := .the, plur := .one, payload := .player false }
private def outer : DRS := DRS.ofList [artifactBinding, letterB .x]
private def creatureX : NounPhrase :=
  target (.and [creature, .compare [.stat .power] .eq (.letter .x)])
private def artifactX : NounPhrase :=
  target (.and [artifact, .compare [.stat .manaValue] .eq (.letter .x)])
private def powerAfter (drs : DRS) : DRS :=
  (stepInstruction 64 .postEvent .obj drs
    (.establish (.modification (target creature) .power (.up (.letter .x))) none)).1

-- P16: a true tie in grammatical role and clause remains a refusal.
theorem ordinaryItRemainsAmbiguous :
    NounPhrase.checkAt (some .object) (DRS.ofList [creatureBinding, artifactBinding]) it =
      [.anaphor .bare .one 2] := by decide

-- P17: the newer clause wins without an explicit positional window.
theorem formerDepthReachesOuterArtifact :
    NounPhrase.checkAt (some .object) (powerAfter outer) it = [] := by decide

theorem ownStatWithoutOuter :
    NounPhrase.checkAt (some .object) (powerAfter (DRS.ofList [])) it = [] := by decide

theorem ownStatWithOuter :
    NounPhrase.checkAt (some .object) (powerAfter outer) it = [] := by decide

theorem ownStatKeepsCreature :
    (firstReach .bare .one (powerAfter outer)).map (·.ty) = some [.creature] := by decide

theorem attachWithoutOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding]) (attachToIt artifactX) = [] := by decide

theorem attachWithOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (attachToIt artifactX) = [] := by decide

-- P15: the new attachment's clause-mate exclusion preserves the outer tie.
theorem attachKeepsOuterAmbiguity :
    Instruction.check (DRS.ofList [creatureBinding, creatureBinding, letterB .x])
      (attachToIt artifactX) = [.anaphor .bare .one 2] := by decide

theorem blockWithoutOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding])
      (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem blockWithOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem damageOwnPowerWithoutOuter :
    Instruction.check (DRS.ofList []) (dealDamageOwnPower creatureX (target creature)) = [] := by decide

theorem damageOwnPowerWithOuter :
    Instruction.check outer (dealDamageOwnPower creatureX (target creature)) = [] := by decide

private def playerLoop : Instruction :=
  .doForEach (each .anyPlayer)
    (choose (target (.and [permanent, .hasPossessor .controller they])))

theorem loopPlayerRetainsPlayerKind :
    Instruction.check (DRS.ofList []) playerLoop = [] := by decide

theorem loopExportsPluralGroupWithoutSingularMember :
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) playerLoop).1
    (resolveCandidates (.word .player) .many (nextClause after)).length = 1 ∧
      resolveCandidates (.word .player) .one (nextClause after) = [] := by decide

theorem optionalBranchesDoNotExportCreatedObjects :
    let branch := Primitives.Instruction.offer (gainLife (.lit 1))
      (some (create (.lit 1) (creatureToken 1 1 [.white] [])))
      (some (create (.lit 2) (creatureToken 1 1 [.white] [])))
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) branch).1
    resolveCandidates (.word (.type .creature)) .one (nextClause after) = [] := by decide

theorem sourceTypeRefinesOneIdentity :
    let after := (stepNounPhrase 64 (DRS.ofList []) .subj true thisCreature).1
    after.referents.map (fun ref => (ref.id, ref.ty)) = [(0, [.creature])] ∧
      after.nextId = 1 := by decide

theorem negatedSourcePropertyKeepsSourceWithoutPositiveRefinement :
    let before := (stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (destroy (target creature))).1
    let after := (stepCondition 64 (nextClause before)
      (.not (.matches thisEnchantment creature))).1
    let selected := resolveCandidates .bare .one (nextClause after)
    selected.map (·.ty) = [[.enchantment]] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem optionalBodyReadsItsEnclosingPlayer :
    Instruction.check (DRS.ofList [])
      (instruct (target .opponent)
        (Primitives.Instruction.offer (instruct they (gainLife (.lit 1))) none none)) = [] := by
  decide

end Semantics.Proofs.ReferenceScopes
