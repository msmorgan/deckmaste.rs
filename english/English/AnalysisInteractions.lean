import English.Analysis
import English.FrameScope
import English.FrameScopeWitnesses
import English.FeatureInteractions
import English.DependencyInteractions

namespace English.AnalysisInteractions


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

abbrev CheckedSyntax (category : Category) (surface : Surface) :=
  {tree : Syntax Lexeme // Dependencies.Admitted lexicon features dependencies tree category surface}

def a : CheckedSyntax (.verbPhrase .plain) left.surface := ⟨_,left_checked⟩
def b : CheckedSyntax (.verbPhrase .plain) left.surface := ⟨_,right_checked⟩

theorem related : FrameScope.Related ⟨a.val, a.property.1.1⟩ ⟨b.val, b.property.1.1⟩ :=
  .step (.node (.verb Lexeme.put .plain markedFrame) [] []
    (.boundary (.exchange .and_ nominalCategory Lexeme.on creatures.tree artifacts.tree
      creatures.tree artifacts.tree creatures.tree artifacts.tree)))

theorem both_retained :
    Dependencies.Admitted lexicon features dependencies left.tree (.verbPhrase .plain) left.surface ∧
    Dependencies.Admitted lexicon features dependencies right.tree (.verbPhrase .plain) left.surface :=
  ⟨left_checked, right_checked⟩

theorem distinct_frame_readings : a.val ≠ b.val := by intro h; cases h

end Frames

namespace Rejections
open FeatureInteractions

theorem reversed_target_not_candidate (category : Category) (surface : Surface) :
    ¬ Dependencies.Admitted lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩
      (.modify (.adjective .white) (.node (.targeting .target .plural) [creatures]))
      category surface := fun h ↦ reversed_target_excluded h.1.2

end Rejections
end English.AnalysisInteractions
