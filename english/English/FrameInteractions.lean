import English.RolePreference
import English.Documents
import English.PreferenceWitnesses

namespace English.FrameInteractions
open Documents RolePreference

inductive Lexeme where
  | put | creature | artifact | on | during
  deriving DecidableEq

def plural : Agreement := ⟨.third, .plural⟩
def nominalCategory := Category.nounPhrase plural
def role : FrameSlot := ⟨.complement, nominalCategory⟩
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
    (.argument (slot := ⟨.object, nominalCategory⟩) creatures.derives
      (.marked (marker := Lexeme.on) (slot := role) artifacts.derives .nil)), forms.1⟩,
    ⟨.verb ⟨rfl, rfl, rfl, Or.inl rfl⟩
     
      (.argument (slot := ⟨.object, nominalCategory⟩) 
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
      (.argument (slot := ⟨.object, nominalCategory⟩) creatures.derives
        (.marked (marker := Lexeme.on) (slot := role) artifacts.derives .nil))
       (.argument (slot := ⟨.object, nominalCategory⟩) artifacts.derives
      (.marked (marker := Lexeme.on) (slot := role) creatures.derives .nil))),
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
      (.argument (slot := ⟨.object, nominalCategory⟩) creatures.derives
     (.coordinate
      (.marked (marker := Lexeme.on) (slot := role) artifacts.derives .nil)
        (.marked (marker := Lexeme.on) (slot := role) creatures.derives .nil))),
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

/-- The acyclicity half of `PreferenceWitnesses.acyclic_tie`, re-spelled over a preference that is
actually inhabited. `PreferenceWitnesses.neutral` prefers nothing (`neutral_prefers_nothing`), so
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

/-! ### The preference layer instantiated on syntax

`docs/decisions/english-lexical-analysis.md` supersedes structural-specificity arbitration *as
admission* and preserves it *as an optional, non-destructive preference*. That is the role the
`Preference.Claims`/`Preference.Policy` algebra keeps, and this is its instantiation on real
syntax: a policy over `Syntax Lexeme` whose claims read declared features only — how many
declared marked roles an analysis realizes (`RolePreference.roleCount`) and how many declared
lexical-identity anchors it binds (`RolePreference.identityCount`). No lexeme, spelling or card
identity is named. The comparison region is the ordered lexical-identity sequence, so only
analyses of the same words are ever compared. -/

def marked : Syntax Lexeme :=
  selectedTree .put .on .plain .active plural role creatures.tree artifacts.tree
def postmodified : Syntax Lexeme :=
  postmodifierTree .put .on .plain .active plural role creatures.tree artifacts.tree

def roleSurface : Surface := ["put", "creatures", "on", "artifacts"]

def admittedHere (tree : Syntax Lexeme) : Prop :=
  Admissible lexicon tree (.verbPhrase .plain) roleSurface

def analyses : List (Syntax Lexeme) := [marked, postmodified]

/-- Claims read from declared features; the region is the ordered lexical identities. -/
def framePolicy : Preference.Policy (Syntax Lexeme) (List Lexeme) where
  region tree := GrammaticalScope.lexicalLeaves tree
  claims tree := ⟨decide (0 < roleCount tree), decide (0 < identityCount tree)⟩

/-- The same policy with the frame-role claim suppressed and nothing else changed. -/
def roleBlindPolicy : Preference.Policy (Syntax Lexeme) (List Lexeme) where
  region tree := GrammaticalScope.lexicalLeaves tree
  claims tree := ⟨false, decide (0 < identityCount tree)⟩

/-- The claims function is not constant: an identity anchor raises the identity claim. -/
theorem frame_policy_claims_read_features :
    framePolicy.claims marked = ⟨true, false⟩ ∧
    framePolicy.claims postmodified = ⟨false, false⟩ ∧
    framePolicy.claims (.identity Lexeme.creature nominalCategory) = ⟨false, true⟩ := by
  refine ⟨?_, ?_, ?_⟩ <;>
    simp [framePolicy, marked, postmodified, selectedTree, postmodifierTree, object,
      roleCount, roleChildren, identityCount, identityChildren, markedCount,
      creatures, artifacts]

/-- Both analyses are in one comparison region: they bind the same lexical identities in order. -/
theorem frame_policy_same_region : framePolicy.region marked = framePolicy.region postmodified := by
  simp [framePolicy, marked, postmodified, selectedTree, postmodifierTree, object,
    GrammaticalScope.lexicalLeaves, GrammaticalScope.childLeaves,
    GrammaticalScope.constructionLexemes, creatures, artifacts]

theorem frame_policy_prefers_marked : Preference.Prefers framePolicy marked postmodified := by
  refine ⟨frame_policy_same_region, ?_⟩
  simp [Preference.Claims.rank, frame_policy_claims_read_features.1,
    frame_policy_claims_read_features.2.1]

/-- The marked-frame analysis is the one the preference selects, over the two admitted analyses of
one surface. -/
theorem frame_policy_selects_marked :
    Preference.Selected admittedHere framePolicy analyses marked := by
  refine ⟨by simp [analyses], role_pair_admitted.1, ?_⟩
  rintro other member _ ⟨region, rank⟩
  simp [analyses] at member
  rcases member with rfl | rfl
  · omega
  · rw [frame_policy_claims_read_features.1, frame_policy_claims_read_features.2.1] at rank
    simp [Preference.Claims.rank] at rank

/-- Preference is not admission: the analysis it does not select stays admitted. -/
theorem frame_policy_does_not_revoke_admission :
    ¬ Preference.Selected admittedHere framePolicy analyses postmodified ∧
      admittedHere postmodified :=
  ⟨fun selected ↦
      selected.2.2 marked (by simp [analyses]) role_pair_admitted.1 frame_policy_prefers_marked,
    role_pair_admitted.2⟩

/-- The weakened-premise counterexample: suppress the frame-role claim — the one declared feature
that separates these two analyses — and the postmodifier analysis is selected as well. The layer
is therefore carrying the distinction, not restating admission. -/
theorem role_blind_policy_selects_postmodifier :
    Preference.Selected admittedHere roleBlindPolicy analyses postmodified := by
  refine ⟨by simp [analyses], role_pair_admitted.2, ?_⟩
  rintro other member _ ⟨_, rank⟩
  simp [analyses] at member
  rcases member with rfl | rfl <;>
    simp [roleBlindPolicy, Preference.Claims.rank, marked, selectedTree, postmodified,
      postmodifierTree, object, identityCount, identityChildren, creatures, artifacts] at rank

theorem role_preference_does_not_choose_homographs :
    ¬ Prefers PreferenceWitnesses.homographs (.noun false .plural) (.noun true .plural) :=
  equal_role_counts_cannot_prefer rfl

end English.FrameInteractions
