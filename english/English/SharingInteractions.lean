import English.Analysis
import English.Composition
import English.FeatureInteractions

namespace English.SharingInteractions

namespace Heads
open FeatureInteractions

def adjectives : Syntax Lexeme := .node (.coordinate .and_ .adjectivePhrase)
  [.adjective .white,.adjective .white]
def wide : Syntax Lexeme := .modify adjectives creatures
def gapped : Syntax Lexeme := .sharedCoordination .and_ (.nominal .plural)
  (.modify (.adjective .white) (.gap (.nominal .plural)))
  (.modify (.adjective .white) (.gap (.nominal .plural)))
def raised : Syntax Lexeme := .node (.rightNodeRaising (.nominal .plural) (.nominal .plural))
  [gapped,creatures]

theorem both_admitted : Admissible lexicon wide (.nominal .plural)
  ["white","and","white","creatures"] ∧
    Admissible lexicon raised (.nominal .plural) ["white","and","white","creatures"] := by
  have gap : Judges lexicon (.modify (.adjective .white) (.gap (.nominal .plural)))
      (.nominal .plural) [.nominal .plural] := .modify (.adjective rfl) .gap
  have gapR : Realizes lexicon (.modify (.adjective .white) (.gap (.nominal .plural))) ["white"] :=
    .modify (.adjective (lexicon := lexicon) ⟨rfl,rfl⟩) .gap
  exact ⟨⟨.modify (.node (.coordinate rfl)
    (.cons (.adjective rfl) (.cons (.adjective rfl) .nil))) (.noun (Or.inl ⟨rfl,rfl⟩)),
    .modify (.node (.cons (.adjective (lexicon := lexicon) ⟨rfl,rfl⟩) (.cons (.adjective
      ⟨rfl,rfl⟩) .nil)) .coordinate)
      (.noun (Or.inl ⟨rfl,rfl,rfl⟩))⟩,
    ⟨.rightNodeRaising (.sharedCoordination rfl gap gap) (.noun (Or.inl ⟨rfl,rfl⟩)),
      .node (.cons (.sharedCoordination gapR gapR) (.cons (.noun (Or.inl ⟨rfl,rfl,rfl⟩)) .nil))
        .rightNodeRaising⟩⟩

theorem both_checked :
    Dependencies.Admitted lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩ wide
      (.nominal .plural) ["white","and","white","creatures"] ∧
    Dependencies.Admitted lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩ raised
      (.nominal .plural) ["white","and","white","creatures"] := by
  constructor
  · refine ⟨⟨both_admitted.1,?_⟩,?_⟩
    · simp [wide,adjectives,creatures,Features.Conforms,Features.ChildrenConform,Features.Local,
        Features.containsTarget]
    · simp
      [wide,adjectives,creatures,Dependencies.Safe,Dependencies.ChildrenSafe,Dependencies.Local,
        Dependencies.exposed,Dependencies.childrenExposed]
  · refine ⟨⟨both_admitted.2,?_⟩,?_⟩
    · simp [raised,gapped,creatures,Features.Conforms,Features.Local,
        Features.containsTarget]
    · simp [raised,gapped,creatures,Dependencies.Safe,Dependencies.ChildrenSafe,Dependencies.Local,
        Dependencies.exposed]

def a : Analysis.Reading lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩
    (.nominal .plural) ["white","and","white","creatures"] := ⟨wide,both_checked.1⟩
def b : Analysis.Reading lexicon features ⟨fun _ ↦ False,fun _ ↦ False⟩
    (.nominal .plural) ["white","and","white","creatures"] := ⟨raised,both_checked.2⟩

theorem shared_head_class : Analysis.key a = Analysis.key b :=
  (Analysis.key_exact _ _).mpr
    (.step (.sharedHead (.adjective Lexeme.white) (.adjective Lexeme.white) creatures .plural))

theorem distinct : wide ≠ raised := by intro h; cases h

end Heads

namespace Determiners
open Composition

def two : Syntax Lexeme := .node (.quantify .plural) [.word .two (.cardinalNumeral .plural)]
def wide : Syntax Lexeme := .node (.determine .plural)
  [two,.node (.coordinate .and_ (.nominal .plural)) [.noun .creature .plural,.noun .artifact
    .plural]]
def narrow : Syntax Lexeme := .node (.coordinate .and_ (.nounPhrase plural))
  [.node (.determine .plural) [two,.noun .creature .plural],artifacts]

theorem both_admitted : Admissible lexicon wide (.nounPhrase plural)
  ["two","creatures","and","artifacts"] ∧
    Admissible lexicon narrow (.nounPhrase plural) ["two","creatures","and","artifacts"] := by
  have twoD : Derives lexicon two (.determinativePhrase .plural) :=
    .node .quantify (.cons (.word .quantity (Or.inr (Or.inl ⟨rfl,rfl⟩))) .nil)
  have twoR : Realizes lexicon two ["two"] :=
    .node (.cons (.word (Or.inl ⟨rfl,rfl,rfl⟩)) .nil) .quantify
  have cn : Derives lexicon (.noun .creature .plural) (.nominal .plural) := .noun (Or.inl rfl)
  have an : Derives lexicon (.noun .artifact .plural) (.nominal .plural) := .noun (Or.inr (Or.inl
    rfl))
  have cr : Realizes lexicon (.noun .creature .plural) ["creatures"] := .noun (Or.inl ⟨rfl,rfl,rfl⟩)
  have ar : Realizes lexicon (.noun .artifact .plural) ["artifacts"] := .noun (Or.inr (Or.inl
    ⟨rfl,rfl,rfl⟩))
  exact ⟨⟨.node .determine (.cons twoD (.cons (.node (.coordinate rfl) (.cons cn (.cons an .nil)))
    .nil)),
    .node (.cons twoR (.cons (.node (.cons cr (.cons ar .nil)) .coordinate) .nil)) .determine⟩,
    ⟨.node (.coordinate rfl) (.cons (.node .determine (.cons twoD (.cons cn .nil)))
      (.cons artifacts_derives .nil)),
    .node (.cons (.node (.cons twoR (.cons cr .nil)) .determine)
      (.cons artifacts_surface .nil)) .coordinate⟩⟩

theorem shared_determiner_class :
    FrameScope.Related ⟨wide,both_admitted.1⟩ ⟨narrow,both_admitted.2⟩ :=
  .step (.determiner two (.noun Lexeme.creature .plural) (.noun Lexeme.artifact .plural))

end Determiners
end English.SharingInteractions
