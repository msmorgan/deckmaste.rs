import English.RolePreference
import English.Documents
import English.SelectionWitnesses

namespace English.FrameInteractions
open Documents RolePreference

inductive Lexeme where
  | put | creature | artifact | on | during
  deriving DecidableEq

def plural : Agreement := ⟨.third, .plural⟩
def nominalCategory := Category.nounPhrase plural
def role : Complement := ⟨.complement, nominalCategory⟩
def baseFrame : List (FrameItem Lexeme) := [object plural]
def markedFrame : List (FrameItem Lexeme) := [object plural, .marked .on role]

def lexicon : Lexicon Lexeme where
  noun head number := (head = .creature ∨ head = .artifact) ∧ number = .plural
  adjective _ := False
  nounForm head number surface := number = .plural ∧
    ((head = .creature ∧ surface = (["creatures"] : Surface)) ∨
     (head = .artifact ∧ surface = (["artifacts"] : Surface)))
  adjectiveForm _ _ := False
  verb head form voice frame := head = .put ∧ form = .plain ∧ voice = .active ∧
    (frame = baseFrame ∨ frame = markedFrame)
  verbForm head form surface := head = .put ∧ form = .plain ∧ surface = (["put"] : Surface)
  preposition head category := (head = .on ∨ head = .during) ∧ category = nominalCategory
  markerForm head surface := (head = .on ∧ surface = (["on"] : Surface)) ∨
    (head = .during ∧ surface = (["during"] : Surface))

def creatures : Witness Lexeme lexicon nominalCategory :=
  ⟨.node .barePlural [.noun .creature .plural], ["creatures"],
   .node .barePlural (.cons (.noun (lexicon := lexicon) ⟨Or.inl rfl, rfl⟩) .nil),
   .node (.cons (.noun (lexicon := lexicon) ⟨rfl, Or.inl ⟨rfl, rfl⟩⟩) .nil) .barePlural⟩

def artifacts : Witness Lexeme lexicon nominalCategory :=
  ⟨.node .barePlural [.noun .artifact .plural], ["artifacts"],
   .node .barePlural (.cons (.noun (lexicon := lexicon) ⟨Or.inr rfl, rfl⟩) .nil),
   .node (.cons (.noun (lexicon := lexicon) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩) .nil) .barePlural⟩

def onArtifacts : Witness Lexeme lexicon .prepositionPhrase :=
  ⟨.node (.preposition .on nominalCategory) [artifacts.tree], ["on", "artifacts"],
   .node (.preposition ⟨Or.inl rfl, rfl⟩) (.cons artifacts.derives .nil),
   .node (.cons artifacts.realizes .nil) (.preposition (m := ["on"]) (Or.inl ⟨rfl, rfl⟩))⟩

def duringArtifacts : Witness Lexeme lexicon .prepositionPhrase :=
  ⟨.node (.preposition .during nominalCategory) [artifacts.tree], ["during", "artifacts"],
   .node (.preposition ⟨Or.inr rfl, rfl⟩) (.cons artifacts.derives .nil),
   .node (.cons artifacts.realizes .nil) (.preposition (m := ["during"]) (Or.inr ⟨rfl, rfl⟩))⟩

def postmodify (head : Witness Lexeme lexicon nominalCategory)
    (pp : Witness Lexeme lexicon .prepositionPhrase) : Witness Lexeme lexicon nominalCategory :=
  ⟨.node (.adjunct nominalCategory .prepositionPhrase) [head.tree, pp.tree],
   head.surface ++ pp.surface,
   .node (.adjunct .nounPhrase) (.cons head.derives (.cons pp.derives .nil)),
   .node (.cons head.realizes (.cons pp.realizes .nil)) .adjunct⟩

def pair (a b : Witness Lexeme lexicon nominalCategory) : Witness Lexeme lexicon nominalCategory :=
  ⟨.node (.coordinate .and_ nominalCategory) [a.tree, b.tree],
   a.surface ++ (["and"] : Surface) ++ b.surface,
   .node (.coordinate rfl) (.cons a.derives (.cons b.derives .nil)),
   .node (.cons a.realizes (.cons b.realizes .nil)) .coordinate⟩

theorem immediate_role : RoleStep lexicon
    (selectedTree .put .on .plain .active plural role creatures.tree artifacts.tree)
    (postmodifierTree .put .on .plain .active plural role creatures.tree artifacts.tree) :=
  .immediate ⟨rfl, rfl, rfl, Or.inr rfl⟩ ⟨rfl, rfl, rfl, Or.inl rfl⟩
    ⟨Or.inl rfl, rfl⟩ (by simp [FirstRole, postmodifiers, creatures])
    creatures.derives artifacts.derives

theorem role_pair_admitted :
    Admissible lexicon
      (selectedTree .put .on .plain .active plural role creatures.tree artifacts.tree)
      (.verbPhrase .plain) ["put", "creatures", "on", "artifacts"] ∧
    Admissible lexicon
      (postmodifierTree .put .on .plain .active plural role creatures.tree artifacts.tree)
      (.verbPhrase .plain) ["put", "creatures", "on", "artifacts"] := by
  have forms := immediate_same_surface (lexicon := lexicon) (head := .put) (marker := .on)
    (form := .plain) (voice := .active) (agreement := plural) (role := role)
    (v := ["put"]) (m := ["on"]) ⟨rfl, rfl, rfl⟩ (Or.inl ⟨rfl, rfl⟩)
    creatures.realizes artifacts.realizes
  exact ⟨⟨.verb ⟨rfl, rfl, rfl, Or.inr rfl⟩
    (.argument (complement := ⟨.object, nominalCategory⟩) creatures.derives
      (.marked (marker := Lexeme.on) (complement := role) artifacts.derives .nil)), forms.1⟩,
    ⟨.verb ⟨rfl, rfl, rfl, Or.inl rfl⟩
     
      (.argument (complement := ⟨.object, nominalCategory⟩) 
        (postmodify creatures onArtifacts).derives .nil), forms.2⟩⟩

theorem earlier_eligible_blocks :
    ¬ FirstRole Lexeme.on role (postmodify creatures onArtifacts).tree :=
  role_candidate_cannot_skip ⟨rfl, rfl⟩

theorem earlier_ineligible_skipped :
    FirstRole Lexeme.on role (postmodify creatures duringArtifacts).tree := by
  simp [FirstRole, postmodifiers, postmodify, creatures, duringArtifacts, matchesRole]

theorem nonfinal_postmodifier_preserved :
    FirstRole Lexeme.on role (pair (postmodify creatures onArtifacts) artifacts).tree := by
  simp [FirstRole, postmodifiers, pair, artifacts]

def pairedArguments : Syntax Lexeme := .frameCoordination .and_
  [creatures.tree, .marker .on, artifacts.tree]
  [artifacts.tree, .marker .on, creatures.tree]

def putPairs : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .put .plain markedFrame) [pairedArguments],
   ["put", "creatures", "on", "artifacts", "and", "artifacts", "on", "creatures"],
   .verb ⟨rfl, rfl, rfl, Or.inr rfl⟩
     (.coordinate (coordinator := .and_)
      (.argument (complement := ⟨.object, nominalCategory⟩) creatures.derives
        (.marked (marker := Lexeme.on) (complement := role) artifacts.derives .nil))
       (.argument (complement := ⟨.object, nominalCategory⟩) artifacts.derives
      (.marked (marker := Lexeme.on) (complement := role) creatures.derives .nil))),
   .node (.cons (.frameCoordination
     (.cons creatures.realizes (.cons (.marker (Or.inl ⟨rfl, rfl⟩))
       (.cons artifacts.realizes .nil)))
     (.cons artifacts.realizes (.cons (.marker (Or.inl ⟨rfl, rfl⟩))
       (.cons creatures.realizes .nil)))) .nil)
      (.verb (lexicon := lexicon) (v := ["put"]) ⟨rfl, rfl, rfl⟩)⟩

/-- Only the final part of the declared frame is repeated under the shared verb and Object. -/
def putSharedObject : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .put .plain markedFrame)
     [creatures.tree, .frameCoordination .and_
       [.marker .on, artifacts.tree] [.marker .on, creatures.tree]],
   ["put", "creatures", "on", "artifacts", "and", "on", "creatures"],
   .verb ⟨rfl, rfl, rfl, Or.inr rfl⟩
      (.argument (complement := ⟨.object, nominalCategory⟩) creatures.derives
     (.coordinate
      (.marked (marker := Lexeme.on) (complement := role) artifacts.derives .nil)
        (.marked (marker := Lexeme.on) (complement := role) creatures.derives .nil))),
   .node (.cons creatures.realizes (.cons (.frameCoordination
     (.cons (.marker (Or.inl ⟨rfl, rfl⟩)) (.cons artifacts.realizes .nil))
     (.cons (.marker (Or.inl ⟨rfl, rfl⟩)) (.cons creatures.realizes .nil))) .nil))
     (.verb (lexicon := lexicon) (v := ["put"]) ⟨rfl, rfl, rfl⟩)⟩

theorem paired_arguments_text :
    Spells putPairs.surface "put creatures on artifacts and artifacts on creatures" :=
  .cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))

theorem shared_object_text :
    Spells putSharedObject.surface "put creatures on artifacts and on creatures" :=
  .cons (.cons (.cons (.cons (.cons (.cons .single)))))

theorem marked_role_cannot_use_other_marker :
    ¬ JudgeFrame lexicon [.marker .during, artifacts.tree] [.marked .on role] [] := by
  intro h
  cases h

theorem concrete_role_selection :
    Selected [⟨_, role_pair_admitted.1⟩, ⟨_, role_pair_admitted.2⟩]
      ⟨_, role_pair_admitted.1⟩ ∧
    ¬ Selected [⟨_, role_pair_admitted.1⟩, ⟨_, role_pair_admitted.2⟩]
      ⟨_, role_pair_admitted.2⟩ :=
  role_pair_selected _ _ immediate_role

/-- The acyclicity half of `SelectionWitnesses.acyclic_tie`, re-spelled over a preference that is
actually inhabited. `SelectionWitnesses.neutral` prefers nothing (`neutral_prefers_nothing`), so
acyclicity over it is a statement about the empty relation; `immediate_role` inhabits
`RolePreference.Prefers` in this lexicon, so ruling out cycles here has content. The tie half is
retained in the same shape: two unequal trees, independently admitted on one surface. -/
theorem role_acyclic_tie :
    (∃ better worse : Syntax Lexeme, Prefers lexicon better worse) ∧
    (∀ tree : Syntax Lexeme, ¬ Relation.TransGen (Prefers lexicon) tree tree) ∧
    selectedTree .put .on .plain .active plural role creatures.tree artifacts.tree ≠
      postmodifierTree .put .on .plain .active plural role creatures.tree artifacts.tree ∧
    Admissible lexicon
      (selectedTree .put .on .plain .active plural role creatures.tree artifacts.tree)
      (.verbPhrase .plain) ["put", "creatures", "on", "artifacts"] ∧
    Admissible lexicon
      (postmodifierTree .put .on .plain .active plural role creatures.tree artifacts.tree)
      (.verbPhrase .plain) ["put", "creatures", "on", "artifacts"] := by
  refine ⟨⟨_, _, .role immediate_role⟩, fun _ ↦ no_role_preference_cycle, ?_,
    role_pair_admitted.1, role_pair_admitted.2⟩
  intro same
  have increase := role_step_increases immediate_role
  rw [same] at increase
  omega

theorem role_preference_does_not_choose_homographs :
    ¬ Prefers SelectionWitnesses.homographs (.noun false .plural) (.noun true .plural) :=
  equal_role_counts_cannot_prefer rfl

end English.FrameInteractions
