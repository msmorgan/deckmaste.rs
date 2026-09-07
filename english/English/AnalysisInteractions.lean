import English.Analysis
import English.FeatureInteractions
import English.DependencyInteractions

namespace English.AnalysisInteractions
open Analysis

namespace Frames
open FrameInteractions
open FrameScope.Witnesses

def features : Features.Declarations Lexeme where
  nounUse _ use := use = .count
  determinerUse _ use := use = .count
  temporalNoun _ := False

def dependencies : Dependencies.Declarations Lexeme := ⟨fun _ ↦ False,fun _ ↦ False⟩

private theorem noun_use (head : Lexeme) (number : Number) :
    Features.NominalUse features (.noun head number) .count := .noun rfl

private theorem left_checked : Dependencies.Admitted lexicon features dependencies left.tree
    (.verbPhrase .plain) left.surface := by
  refine ⟨⟨⟨left.derives,left.realizes⟩,?features⟩,?safe⟩
  case features =>
    simp [left,cluster,FrameScope.pairs,pair,creatures,artifacts,
      Features.Conforms, Features.ChildrenConform, Features.Local, noun_use,
      FrameCases, CaseAt, Syntax.nominalCase, markedFrame, nominalCategory,
      RolePreference.object, role,
      Relation.casePosition, Case.Allows, Case.Argument, Case.common]
  case safe =>
    simp [left,cluster,FrameScope.pairs,pair,creatures,artifacts,
      Dependencies.Safe,Dependencies.ChildrenSafe,Dependencies.Local,
      Dependencies.exposed,Dependencies.childrenExposed]

private theorem right_checked : Dependencies.Admitted lexicon features dependencies right.tree
    (.verbPhrase .plain) left.surface := by
  refine ⟨⟨⟨right.derives,right.realizes⟩,?features⟩,?safe⟩
  case features =>
    simp [right,cluster,FrameScope.pairs,pair,creatures,artifacts,
      Features.Conforms, Features.ChildrenConform, Features.Local, noun_use,
      FrameCases, CaseAt, Syntax.nominalCase, markedFrame, nominalCategory,
      RolePreference.object, role,
      Relation.casePosition, Case.Allows, Case.Argument, Case.common]
  case safe =>
    simp [right,cluster,FrameScope.pairs,pair,creatures,artifacts,
      Dependencies.Safe,Dependencies.ChildrenSafe,Dependencies.Local,
      Dependencies.exposed,Dependencies.childrenExposed]

def a : Reading lexicon features dependencies (.verbPhrase .plain) left.surface := ⟨_,left_checked⟩
def b : Reading lexicon features dependencies (.verbPhrase .plain) left.surface := ⟨_,right_checked⟩

theorem related : Analysis.Related a b :=
  .step (.node (.verb Lexeme.put .plain markedFrame) [] []
    (.boundary (.exchange .and_ nominalCategory Lexeme.on creatures.tree artifacts.tree
      creatures.tree artifacts.tree creatures.tree artifacts.tree)))

theorem both_selected : Selected [a,b] a ∧ Selected [a,b] b := by
  have equal : measure a.val = measure b.val := rfl
  constructor
  · refine ⟨by simp,?_⟩
    intro other member pref
    have smaller := preference_increases pref
    simp only [List.mem_cons,List.not_mem_nil,or_false] at member
    rcases member with rfl | rfl <;> omega
  · refine ⟨by simp,?_⟩
    intro other member pref
    have smaller := preference_increases pref
    simp only [List.mem_cons,List.not_mem_nil,or_false] at member
    rcases member with rfl | rfl <;> omega

theorem complete_packing : (package [a,b] a).readings a ∧ (package [a,b] a).readings b :=
  ⟨(package_exact _ _ _).mpr ⟨both_selected.1,.refl _⟩,
   (package_exact _ _ _).mpr ⟨both_selected.2,.symm related⟩⟩

end Frames

namespace Identities

def agreement : Agreement := ⟨.third,.singular⟩
def lexicon : Lexicon Bool where
  noun _ _ := False
  adjective _ := False
  nounForm _ _ _ := False
  adjectiveForm _ _ := False
  identity head category := head = false ∧ category = .nounPhrase agreement
  identityForm head category surface :=
    head = false ∧ category = .nounPhrase agreement ∧ surface = (["Echo"] : Surface)
  word head category := head = true ∧ category = .nounPhrase agreement
  wordForm head category surface :=
    head = true ∧ category = .nounPhrase agreement ∧ surface = (["Echo"] : Surface)
def features : Features.Declarations Bool where
  nounUse _ _ := False
  determinerUse _ _ := False
  temporalNoun _ := False
def dependencies : Dependencies.Declarations Bool := ⟨fun _ ↦ False,fun _ ↦ False⟩
def identity : Reading lexicon features dependencies (.nounPhrase agreement) ["Echo"] :=
  ⟨.identity false (.nounPhrase agreement),
    ⟨⟨⟨.identity .pronoun ⟨rfl,rfl⟩,.identity ⟨rfl,rfl,rfl⟩⟩,⟨trivial,trivial⟩⟩,
      ⟨trivial,trivial⟩⟩⟩
def lexical : Reading lexicon features dependencies (.nounPhrase agreement) ["Echo"] :=
  ⟨.word true (.nounPhrase agreement),
    ⟨⟨⟨.word .pronoun ⟨rfl,rfl⟩,.word ⟨rfl,rfl,rfl⟩⟩,⟨trivial,trivial⟩⟩,
      ⟨trivial,trivial⟩⟩⟩

theorem identity_preference : Prefers lexicon identity.val lexical.val :=
  .identity (.word (surface := ["Echo"]) ⟨rfl,rfl⟩ ⟨rfl,rfl,rfl⟩ ⟨rfl,rfl⟩ ⟨rfl,rfl,rfl⟩)

theorem identity_selected : Selected [identity,lexical] identity ∧
    ¬ Selected [identity,lexical] lexical := by
  constructor
  · refine ⟨by simp,?_⟩
    intro other member pref
    have smaller := preference_increases pref
    simp only [List.mem_cons,List.not_mem_nil,or_false] at member
    rcases member with rfl | rfl
    · omega
    · change 1 < 0 at smaller
      omega
  · intro selected
    exact selected.2 identity (by simp) identity_preference

end Identities

namespace Rejections
open FeatureInteractions

theorem reversed_target_not_candidate (category : Category) (surface : Surface) :
    ¬ Dependencies.Admitted lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩
      (.modify (.adjective .white) (.node (.targeting .target .plural) [creatures]))
      category surface := fun h ↦ reversed_target_excluded h.1.2

end Rejections
end English.AnalysisInteractions
