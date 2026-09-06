import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.ReferenceScopes

private def creatureBinding : Binding := ⟨.target, .one,
  .object [.creature] (some .battlefield) none none (some 1)⟩
private def artifactBinding : Binding := ⟨.target, .one,
  .object [.artifact] (some .battlefield) none none (some 1)⟩
private def playerBinding : Binding := ⟨.the, .one, .player false⟩
private def outer : Bindings := [artifactBinding, letterB .x]
private def creatureX : NounPhrase :=
  target (.and [creature, .compare [.stat .power] .eq (.letter .x)])
private def artifactX : NounPhrase :=
  target (.and [artifact, .compare [.stat .manaValue] .eq (.letter .x)])
private def playerX : NounPhrase :=
  each (.and [.anyPlayer, .compare [.playerStat .lifeTotal] .eq (.letter .x)])
private def powerAfter (bs : Bindings) : Bindings :=
  StaticSpec.intro bs (.modification (target creature) .power (.up (.letter .x)))
private def previous : Instruction := .putCounters (.lit 1) (.printed plusOnePlusOne) creatureX
private def subjectCondition : Condition := .matches creatureX creature
private def libraryCards : NounPhrase := .librarySlice .top (.lit 2) playerX

/-- Ordinary pronouns keep the ambiguity of their containing discourse. -/
theorem ordinaryItRemainsAmbiguous :
    NounPhrase.check (some .object) [creatureBinding, artifactBinding] it =
      [.anaphor .bare .one 2] := by decide

/-- The old empty-context depth included the unrelated artifact when X was already bound. -/
theorem formerDepthReachesOuterArtifact :
    NounPhrase.check (some .object) (powerAfter outer) (.pro .bare .one (.top 2)) =
      [.anaphor .bare .one 2] := by decide

theorem amountLetterReallyChangesThePrefix :
    (NounPhrase.introduced [] creatureX).length = 2 ∧
    (NounPhrase.introduced outer creatureX).length = 1 := by decide

theorem ownStatWithoutOuter :
    NounPhrase.check (some .object) (powerAfter [])
      (itsOther (target creature) (.up (.letter .x))) = [] := by decide

theorem ownStatWithOuter :
    NounPhrase.check (some .object) (powerAfter outer)
      (itsOther (target creature) (.up (.letter .x))) = [] := by decide

theorem ownStatKeepsCreature :
    NounPhrase.ty (powerAfter outer) (itsOther (target creature) (.up (.letter .x))) =
      [.creature] := by decide

theorem sharedSubjectWithoutOuter :
    NounPhrase.check (some .object) (selfSubjIntro [] creatureX) (ownSubject creatureX) = [] := by decide

theorem sharedSubjectWithOuter :
    NounPhrase.check (some .object) (selfSubjIntro outer creatureX) (ownSubject creatureX) = [] := by decide

theorem sharedSubjectKeepsCreature :
    NounPhrase.ty (selfSubjIntro outer creatureX) (ownSubject creatureX) = [.creature] := by decide

theorem previousInstructionWithoutOuter :
    NounPhrase.check (some .object) (Instruction.intro [] previous) (itPrior previous) = [] := by decide

theorem previousInstructionWithOuter :
    NounPhrase.check (some .object) (Instruction.intro outer previous) (itPrior previous) = [] := by decide

theorem previousInstructionKeepsCreature :
    NounPhrase.ty (Instruction.intro outer previous) (itPrior previous) = [.creature] := by decide

theorem conditionSubjectWithoutOuter :
    NounPhrase.check (some .object) (Condition.intro [] subjectCondition)
      (itCondSubject subjectCondition) = [] := by decide

theorem conditionSubjectWithOuter :
    NounPhrase.check (some .object) (Condition.intro outer subjectCondition)
      (itCondSubject subjectCondition) = [] := by decide

theorem conditionSubjectKeepsCreature :
    NounPhrase.ty (Condition.intro outer subjectCondition) (itCondSubject subjectCondition) =
      [.creature] := by decide

theorem agentWithoutOuter :
    NounPhrase.check (some .player) (agentIntro [] playerX) (agentRef playerX) = [] := by decide

theorem agentWithOuter :
    NounPhrase.check (some .player) (agentIntro [playerBinding, letterB .x] playerX)
      (agentRef playerX) = [] := by decide

theorem librarySliceWithoutOuter :
    NounPhrase.check (some .object) (nomIntro [] libraryCards) (lookedCards libraryCards) = [] := by decide

theorem librarySliceWithOuter :
    NounPhrase.check (some .object) (nomIntro outer libraryCards) (lookedCards libraryCards) = [] := by decide

theorem librarySliceKeepsLibraryCarrier :
    NounPhrase.zone (nomIntro outer libraryCards) (lookedCards libraryCards) = some .library := by decide

theorem attachWithoutOuterLetter :
    Instruction.check [creatureBinding] (attachToIt artifactX) = [] := by decide

theorem attachWithOuterLetter :
    Instruction.check [creatureBinding, playerBinding, letterB .x]
      (attachToIt artifactX) = [] := by decide

theorem attachKeepsOuterAmbiguity :
    Instruction.check [creatureBinding, creatureBinding, letterB .x]
      (attachToIt artifactX) = [.anaphor .bare .one 2] := by decide

theorem blockWithoutOuterLetter :
    Instruction.check [creatureBinding] (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem blockWithOuterLetter :
    Instruction.check [creatureBinding, playerBinding, letterB .x]
      (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem damageOwnPowerWithoutOuter :
    Instruction.check [] (dealDamageOwnPower creatureX (target creature)) = [] := by decide

theorem damageOwnPowerWithOuter :
    Instruction.check outer (dealDamageOwnPower creatureX (target creature)) = [] := by decide

theorem chooserScopeIgnoresOuter (bs : Bindings) :
    NounPhrase.check (some .object) (playerBinding :: bs) (aTheirChoice creature) = [] := by rfl

theorem comparisonScopeIgnoresOuter (bs : Bindings) :
    Predicate.check .object bs (comparesOwnStat .power creature .atLeast (.lit 1)) = [] := by rfl

theorem missingScopeCannotSearchPastAnotherKind :
    view (.introduced [.object]) [playerBinding, creatureBinding] = [] := by decide

theorem invalidScopeCannotMutateOuter :
    overWindow (setZoneReach .bare .one none (some .exile)) (.introduced [.object])
      [playerBinding, creatureBinding] = [playerBinding, creatureBinding] := by rfl

theorem scopedMovePreservesTheOuterBindings :
    moveIntro (powerAfter outer) none (itsOther (target creature) (.up (.letter .x)))
      (some .exile) =
        ⟨.target, .one, .object [.creature] (some .exile) none none (some 1)⟩ :: outer := by rfl

theorem ownStatStableForEveryOuterContext (bs : Bindings) :
    NounPhrase.check (some .object) (powerAfter bs)
      (itsOther (target creature) (.up (.letter .x))) = [] := by
  change NounPhrase.check (some .object)
    ((if countLetter .x bs == 0 then [letterB .x] else []) ++ creatureBinding :: bs)
    (.pro .bare .one (.introduced [.letter .x, .object])) = []
  split <;> rfl

theorem ownStatIdentityStableForEveryOuterContext (bs : Bindings) :
    NounPhrase.ty (powerAfter bs) (itsOther (target creature) (.up (.letter .x))) =
      [.creature] := by
  change NounPhrase.ty
    ((if countLetter .x bs == 0 then [letterB .x] else []) ++ creatureBinding :: bs)
    (.pro .bare .one (.introduced [.letter .x, .object])) = [.creature]
  split <;> rfl

theorem exactPrefixDoesNotInspectOuter (localBindings outer : Bindings)
    (noLetters : ∀ b ∈ localBindings, ∀ l, b.kind ≠ .letter l) :
    introductionWidth (localBindings.map Binding.kind) (localBindings ++ outer) = some localBindings.length := by
  induction localBindings with
  | nil => rfl
  | cons b rest ih =>
    have hrest : ∀ c ∈ rest, ∀ l, c.kind ≠ .letter l := by
      intro c hc l
      exact noLetters c (List.mem_cons_of_mem b hc) l
    have h := ih hrest
    cases hk : b.kind <;> simp_all [introductionWidth]

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.NounPhrase.pro] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scope witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw (.lit 1) (agent := .pro (.word .player) .one (.top 1)))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.NounPhrase.pro] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scope witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw (.lit 1)
        (agent := .pro (.word .player) .one (.introduced [.player])))] } }

end Semantics.Proofs.ReferenceScopes
